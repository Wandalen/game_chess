#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

//!
//! Implement mechanics of the game chess.
//!

pub mod ai;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::ops::Deref;
#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};
pub use pleco::{
  core::Player,
  core::PieceType,
  core::Piece,
  board::piece_locations::PieceLocations, //Minimal board impl

  core::piece_move::MoveType,
  core::piece_move::BitMove as Move, //https://docs.rs/pleco/latest/pleco/core/piece_move/index.html
  core::move_list::MoveList,
  core::sq::SQ as Cell,
  core::bitboard::BitBoard as CellsSet,
};

use serde::{Serialize, Deserialize, Serializer, Deserializer};

/* Structure:

UCI( String )

Board
  pleco_board : pleco::Board

HistoryEntry
  fen : String
  last_move : Move https://docs.rs/pleco/0.5.0/pleco/core/piece_move/struct.BitMove.html

Game
   board : Board
   history : Vec<HistoryEntry>
*/

/* List of resources to show

  tools::eval::Eval,
  helper::Helper,
  helper::prelude,
  bot_prelude

*/

const SAVES_FOLDER_NAME : &str = "saves";
const SAVE_FILE_EXTENSION : &str = ".save";

///
/// Move in UCI format
///

#[derive(Debug)]
pub struct UCI(pub String);

impl From<&str> for UCI
{
  fn from(src : &str) -> Self { Self(src.to_string()) }
}

impl From<Move> for UCI
{
  fn from(src : Move) -> Self { Self(src.stringify()) }
}

impl TryFrom<UCI> for Move
{
  type Error = ();

  fn try_from(_src : UCI) -> Result<Self, Self::Error>
  {
    unimplemented!();
  }
}

///
/// Game board
///

#[derive(Debug, Clone)]
pub struct Board
{
  pleco_board : pleco::Board,
}

impl Board
{
  ///
  /// Constructs a board with the starting position
  ///
  pub fn default() -> Self
  {
    Self {
      pleco_board : pleco::Board::start_pos(),
    }
  }

  ///
  /// Constructs a board from FEN
  ///
  pub fn from_fen(fen : &Fen) -> Self
  {
    match pleco::Board::from_fen(fen)
    {
      Ok(pleco_board) => Self { pleco_board },
      _ => Self::default(),
    }
  }

  ///
  /// Makes move on the board. Accepts move in UCI format.
  ///
  pub fn make_move(&mut self, uci_move : UCI) -> Option<Self>
  {
    let mut pleco_board : pleco::Board = self.pleco_board.clone();
    let result = pleco_board.apply_uci_move(&uci_move.0);
    if result
    {
      Some(Self { pleco_board })
    }
    else
    {
      None
    }
  }

  ///
  /// Checks if the move is valid. Accepts move in UCI format.
  ///
  pub fn move_is_valid(&self, uci_move : UCI) -> bool
  {
    match self.move_from_uci(uci_move)
    {
      Some(m) => self.pleco_board.pseudo_legal_move(m) && self.pleco_board.legal_move(m),
      _ => false,
    }
  }

  ///
  /// Looks for a valid [Move](https://docs.rs/pleco/0.5.0/pleco/core/piece_move/struct.BitMove.html) from move in UCI format.
  ///
  pub fn move_from_uci(&self, uci_move : UCI) -> Option<Move>
  {
    let all_moves : MoveList = self.pleco_board.generate_moves();
    all_moves.iter().find(|m| m.stringify() == uci_move.0).cloned()
  }

  ///
  /// Looks for a move that results in the best board state for the current player and applies it
  ///
  pub fn make_move_ai(&mut self)
  {
    let turn = self.pleco_board.turn();

    let best_move = self
      .pleco_board
      .generate_moves()
      .into_iter()
      .map(|m| {
        self.pleco_board.apply_move(m);
        let score = pleco::tools::eval::Eval::eval_low(&self.pleco_board);
        self.pleco_board.undo_move();
        (m, score)
      })
      .max_by(|(_, a), (_, b)| {
        if turn == Player::Black
        {
          a.cmp(b)
        }
        else
        {
          b.cmp(a)
        }
      })
      .unwrap()
      .0;

    self.pleco_board.apply_move(best_move);
  }

  ///
  /// Returns the piece located at the square
  ///
  pub fn piece_at(&self, sq : u8) -> Piece { self.pleco_board.piece_at_sq(Cell(sq)) }

  ///
  /// Evaluates the score of a [Board] for the current side to move.
  ///
  pub fn score(&self) -> i32
  {
    pleco::tools::eval::Eval::eval_low(&self.pleco_board)
    //0
    /* ttt : implement me */
  }

  ///
  /// True if the current side to move is in check mate.
  ///
  pub fn is_checkmate(&self) -> bool { self.pleco_board.checkmate() }

  ///
  /// Is the current side to move is in stalemate.
  ///
  pub fn is_stalemate(&self) -> bool { self.pleco_board.stalemate() }

  ///
  /// Return the `Player` whose turn it is to move.
  ///
  pub fn current_turn(&self) -> Player { self.pleco_board.turn() }

  ///
  /// Return the last move played, if any.
  ///
  pub fn last_move(&self) -> Option<Move> { self.pleco_board.last_move() }

  ///
  /// Returns pretty-printed string representation of the board
  ///
  pub fn to_pretty_string(&self) -> String
  {
    let mut s = String::with_capacity(pleco::core::masks::SQ_CNT * 2 + 40);
    let mut rank = 8;

    for sq in pleco::core::masks::SQ_DISPLAY_ORDER.iter()
    {
      if sq % 8 == 0
      {
        s.push(char::from_digit(rank, 10).unwrap());
        s.push_str(" | ");
        rank -= 1;
      }

      let op = self.pleco_board.get_piece_locations().piece_at(pleco::SQ(*sq));
      let char = if op != Piece::None { op.character_lossy() } else { '-' };
      s.push(char);
      s.push(' ');

      if sq % 8 == 7
      {
        s.push('\n');
      }
    }

    s.push_str("  ------------------\n");
    s.push_str("    a b c d e f g h");

    s
  }

  ///
  /// Prints board to the terminal.
  ///
  pub fn print(&self) /* qqq : remove. instead return string */
  {
    println!("{}", self.to_pretty_string());
  }

  ///
  /// Creates a 'Fen` string of the board.
  ///
  pub fn to_fen(&self) -> Fen { Fen::from(self.pleco_board.fen()) }
}

///
///Positions on the board in [FEN](https://www.chess.com/terms/fen-chess#what-is-fen) format
///

#[derive(Serialize, Deserialize, Debug)]
pub struct FenString(String);

impl Deref for FenString
{
  type Target = String;

  fn deref(&self) -> &Self::Target { &self.0 }
}

impl From<String> for FenString
{
  fn from(value : String) -> Self { FenString(value) }
}

///
/// Type alias for `FenString`
///

pub type Fen = FenString;

///
/// Contains information about move made in the past.
/// Field `fen` contains representation of the board as FEN string
/// Field `last_move` information about last [Move]https://docs.rs/pleco/0.5.0/pleco/core/piece_move/struct.BitMove.html)
///

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryEntry
{
  fen : Fen,
  #[serde(serialize_with = "move_ser", deserialize_with = "move_der")]
  last_move : Move,
}

///
/// Serialize [Move](https://docs.rs/pleco/0.5.0/pleco/core/piece_move/struct.BitMove.html)
///

pub fn move_ser<S : Serializer>(m : &Move, s : S) -> Result<S::Ok, S::Error> { s.serialize_u16(m.get_raw()) }

///
/// Deserialize [Move](https://docs.rs/pleco/0.5.0/pleco/core/piece_move/struct.BitMove.html)
///

pub fn move_der<'de, D : Deserializer<'de>>(d : D) -> Result<Move, D::Error>
{
  let bits : u16 = Deserialize::deserialize(d)?;
  Ok(Move::new(bits))
}

///
/// Status of the game
///

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum GameStatus
{
  /// The game is not finished, and the game is still in play.
  Continuing,
  /// The game has the winner.
  Checkmate,
  /// The game is drawn.
  Stalemate,
  /// Forfeit
  GG,
}

///
/// Interface for playing chess game.
///
/// Basically Board + History.
///

#[derive(Serialize, Deserialize, Debug)]
pub struct Game
{
  #[serde(serialize_with = "board_ser", deserialize_with = "board_der")]
  board : Board,
  is_forfeited : bool,
  history : Vec<HistoryEntry>,
  ///
  /// AI Engine responsible for finding best moves
  ///
  pub ai : Option<ai::Engine>,
  #[cfg(not(target_arch = "wasm32"))]
  date : SystemTime, // unix timestamp
  #[cfg(target_arch = "wasm32")]
  date : f64, // unix timestamp
}

impl Game
{
  ///
  /// Constructs a new game with default board setup
  ///
  pub fn default() -> Self
  {
    Self {
      board : Board::default(),
      history : Vec::new(),
      is_forfeited : false,
      ai : None,
      #[cfg(not(target_arch = "wasm32"))]
      date : SystemTime::now(),
      #[cfg(target_arch = "wasm32")]
      date : js_sys::Date::now(),
    }
  }

  ///
  /// Constructs a new game from FEN.
  ///

  pub fn from_fen(fen : &str) -> Self
  {
    Self {
      board : Board::from_fen(&Fen::from(fen.to_owned())),
      history : Vec::new(),
      is_forfeited : false,
      ai : None,

      #[cfg(not(target_arch = "wasm32"))]
      date : SystemTime::now(),
      #[cfg(target_arch = "wasm32")]
      date : js_sys::Date::now(),
    }
  }

  ///
  /// Generates moves list.
  ///

  pub fn moves_list(&self) -> MoveList { self.board.pleco_board.generate_moves() }

  /* xxx : ? */

  ///
  /// Calling member board
  ///

  pub fn count_score(&self) -> i32 { self.board.score() }

  ///
  /// Makes a move on the board. Accepts move in UCI format. For example, "e2e4".
  /// Updates histort and returns `true` if move was succesfuly applied, otherwise returns `false`.
  /// The board and history are not changed in case of fail.
  ///

  pub fn make_move(&mut self, uci_move : UCI) -> bool
  {
    let new_board = self.board.make_move(uci_move);
    let success = new_board.is_some();
    if success
    {
      self.board = new_board.unwrap();
      let last_move = self.board.last_move().unwrap();
      self.history.push(HistoryEntry {
        fen : self.board.to_fen(),
        last_move,
      });
    }
    success
  }

  ///
  /// Check if game has AI engine
  ///
  pub fn has_ai(&self) -> bool { self.ai.is_some() }

  ///
  /// AI makes the move using internal AI algorithm
  /// Updates history with the applied move.
  ///
  pub fn make_move_ai(&mut self)
  {
    match &self.ai
    {
      Some(engine) => self.board.pleco_board.apply_move(engine.best_move(self.board.clone())),
      None => self.board.make_move_ai(),
    };

    let last_move = self.board.last_move().unwrap();
    self.history.push(HistoryEntry {
      fen : self.board.to_fen(),
      last_move,
    });
  }

  ///
  /// Return the [Player] whose turn it is to move.
  ///
  pub fn current_turn(&self) -> Player { self.board.current_turn() }

  ///
  /// Prints board to the terminal.
  ///
  pub fn board_print(&self) { self.board.print(); }

  ///
  /// Prints history to the terminal.
  ///
  pub fn history_print(&self)
  {
    for mov in &self.history
    {
      println!("{}", mov.last_move);
    }
  }

  ///
  /// Returns current game status as [GameStatus].
  ///
  pub fn status(&self) -> GameStatus
  {
    if self.is_forfeited
    {
      return GameStatus::GG;
    }

    if self.board.is_checkmate()
    {
      return GameStatus::Checkmate;
    }

    if self.board.is_stalemate()
    {
      return GameStatus::Stalemate;
    }

    GameStatus::Continuing
  }

  ///
  /// Returns last move as UCI string. For example: "a2a4"
  /// Returns None if there are no moves.
  ///
  pub fn last_move(&self) -> Option<UCI>
  {
    self.history.last().map( | h | h.last_move.into() )
  }

  ///
  /// Returns last move as [Move](https://docs.rs/pleco/0.5.0/pleco/core/piece_move/struct.BitMove.html))
  /// Returns None if there are no moves.
  ///
  pub fn last_move_raw(&self) -> Option<Move>
  {
    self.history.last().map( | h | h.last_move )
  }

  ///
  /// Returns the piece located at the square
  ///
  pub fn piece_at(&self, sq : u8) -> Piece { self.board.piece_at(sq) }

  ///
  /// Saves game to file
  ///
  pub fn save(&self) -> std::io::Result<String>
  {
    fs::create_dir_all(SAVES_FOLDER_NAME)?;

    let serialized = serde_json::to_string(&self).unwrap();
    let file_id = get_unix_timestamp(None);
    let filename = format!("{}/{}{}", SAVES_FOLDER_NAME, file_id, SAVE_FILE_EXTENSION);
    let filepath = Path::new(&filename);

    let mut file = File::create(filepath).unwrap();

    match file.write_all(serialized.as_bytes())
    {
      Ok(_) => Ok(filename),
      Err(error) => Err(error),
    }
  }

  ///
  /// Gives ability to forfeit
  ///
  pub fn forfeit(&mut self) { self.is_forfeited = true }

  // FOLLOWING METHODS ARE ADDED FOR MULTIPLAYER FUNCTIONALITY

  ///
  /// Returns board state as `String` for `MultiPlayer`.
  ///
  pub fn board_state_printable(&self) -> String { self.board.to_pretty_string() }

  ///
  /// Checks validity of a given move for `MultiPlayer`.
  ///
  pub fn move_is_valid(&self, uci_move : UCI) -> bool { self.board.move_is_valid(uci_move) }
}

///
/// Get unix timestamp in seconds.
///

#[cfg(target_arch = "wasm32")]
pub fn get_unix_timestamp(_start : Option<js_sys::Date>) -> u64 { js_sys::Date::now() as u64 }

///
/// Get unix timestamp in seconds.
///

#[cfg(not(target_arch = "wasm32"))]
pub fn get_unix_timestamp(start : Option<SystemTime>) -> u64
{
  let start = start.unwrap_or_else(SystemTime::now);
  let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");

  since_the_epoch.as_secs()
}

///
/// Serialize game to string.
///

pub fn board_ser<S : Serializer>(board : &Board, s : S) -> Result<S::Ok, S::Error> { s.serialize_str(&board.to_fen()) }

///
/// Deserialize game from string to FEN and make board.
///

pub fn board_der<'de, D : Deserializer<'de>>(d : D) -> Result<Board, D::Error>
{
  let fen : String = Deserialize::deserialize(d)?;
  Ok(Board::from_fen(&Fen::from(fen)))
}
