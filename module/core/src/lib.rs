#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

//!
//! Implement mechanics of the game chess.
//!

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
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

#[derive(Debug)]
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
  /// Constructs a aborad from FEN
  ///
  pub fn from_fen(fen : &Fen) -> Self
  {
    match pleco::Board::from_fen(&fen)
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
  /// Evaluates the score of a [Board] for the current side to move.
  ///
  pub fn score(&self) -> i32
  {
    0
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
  pub fn to_fen(&self) -> Fen { self.pleco_board.fen() }
}

///
///Positions on the board in [FEN](https://www.chess.com/terms/fen-chess#what-is-fen) format
///

pub type Fen = String;

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

#[derive(Debug, PartialEq)]
pub enum GameStatus
{
  /// The game is not finished, and the game is still in play.
  Continuing,
  /// The game has the winner.
  Checkmate,
  /// The game is drawn.
  Stalemate,
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
  history : Vec<HistoryEntry>,
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

      #[cfg(not(target_arch = "wasm32"))]
      date : SystemTime::now(),
      #[cfg(target_arch = "wasm32")]
      date : js_sys::Date::now(),
    }
  }

  /* xxx : ? */

  ///
  /// Makes a move on the board. Accepts move in UCI format. For example, "e2e4".
  /// Updates histort and returns `true` if move was succesfuly applied, otherwise returns `false`.
  /// The board and history are not changed in case of fail.
  ///
  pub fn make_move(&mut self, uci_move : UCI) -> bool
  {
    let new_board = self.board.make_move(uci_move);
    let success = !new_board.is_none();
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
  /// Return the [Player] whose turn it is to move.
  ///
  pub fn current_turn(&self) -> Player { self.board.current_turn() }

  ///
  /// Prints board to the terminal.
  ///
  pub fn board_print(&self) { self.board.print(); }

  ///
  /// Returns current game status as [GameStatus].
  ///
  pub fn status(&self) -> GameStatus
  {
    if self.board.is_checkmate()
    {
      return GameStatus::Checkmate;
    }

    if self.board.is_stalemate()
    {
      return GameStatus::Stalemate;
    }

    return GameStatus::Continuing;
  }

  ///
  /// Returns last move as UCI string. For example: "a2a4"
  /// Returns None if there are no moves.
  ///
  pub fn last_move(&self) -> Option<UCI>
  {
    match self.history.last()
    {
      Some(h) => Some(h.last_move.into()),
      _ => None,
    }
  }

  ///
  /// Returns last move as [Move](https://docs.rs/pleco/0.5.0/pleco/core/piece_move/struct.BitMove.html))
  /// Returns None if there are no moves.
  ///
  pub fn last_move_raw(&self) -> Option<Move>
  {
    match self.history.last()
    {
      Some(h) => Some(h.last_move.clone()),
      _ => None,
    }
  }

  ///
  /// Saves game to file
  ///
  pub fn save(&self) -> std::io::Result<String>
  {
    fs::create_dir_all(SAVES_FOLDER_NAME)?;

    let serialized = serde_json::to_string(&self).unwrap();
    let file_id = get_unix_timestamp(None);
    let filename = format!("{}/{}{}", SAVES_FOLDER_NAME, file_id.to_string(), SAVE_FILE_EXTENSION);
    let filepath = Path::new(&filename);

    let mut file = File::create(filepath).unwrap();

    match file.write_all(serialized.as_bytes())
    {
      Ok(_) => Ok(filename),
      Err(error) => Err(error),
    }
  }
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
  let start = start.unwrap_or(SystemTime::now());
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
  Ok(Board::from_fen(&fen))
}
