#![warn( missing_debug_implementations )]
#![warn( missing_docs )]

//!
//! Implement mechanics of the game chess.
//!

pub use pleco::
{
  core::Player,
  core::PieceType,
  core::Piece,
  board::piece_locations::PieceLocations, //Minimal board impl

  core::piece_move::MoveType,
  core::piece_move::BitMove as Move, //https://docs.rs/pleco/latest/pleco/core/piece_move/index.html
  core::move_list::MoveList,

  core::sq::SQ as Cell,
  core::bitboard::BitBoard as CellsSet
};

use serde::
{
  ser::Serializer,
  Serialize,
  Deserialize,
  Deserializer,
};

/* Structure:

Board
  pleco_board : pleco::Board

HistoryEntry
  fen : String
  uci_move : String

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

/// Game board
#[derive( Debug )]
pub struct Board
{
  pleco_board : pleco::Board
}

impl Board
{
  ///
  /// Constructs a board with the starting position
  ///
  pub fn default() -> Self
  {
    Self
    {
      pleco_board : pleco::Board::start_pos()
    }
  }

  ///
  /// Creates board from a [Fen] string
  ///
  pub fn from_fen( fen : &Fen ) -> Self
  {
    match pleco::Board::from_fen( fen )
    {
      Ok( pleco_board ) => Self { pleco_board },
      _ => Self::default()
    }
  }

  ///
  /// Makes move on the board. Accepts move in UCI format.
  ///
  pub fn make_move( &mut self, uci_move : &str ) -> Option< Self >
  {
    let mut pleco_board : pleco::Board = self.pleco_board.clone();
    let result = pleco_board.apply_uci_move( &uci_move );
    if result
    {
      Some( Self { pleco_board } )
    }
    else
    {
      None
    }
  }

  ///
  /// Checks if the move is valid. Accepts move in UCI format.
  ///
  pub fn move_is_valid( &self, uci_move : &str ) -> bool
  {
    match self.move_from_uci( uci_move )
    {
      Some( m ) => self.pleco_board.pseudo_legal_move( m ) && self.pleco_board.legal_move( m ),
      _ => false
    }
  }

  ///
  /// Makes [Move] from move in UCI format.
  ///
  pub fn move_from_uci( &self, uci_move : &str ) -> Option< Move >
  {
    let all_moves : MoveList = self.pleco_board.generate_moves();
    all_moves.iter()
    .find(| m | m.stringify() == uci_move )
    .cloned()
  }

  ///
  /// Evaluates the score of a [Board] for the current side to move.
  ///
  pub fn score( &self ) -> i32
  {
    0
    /* ttt : implement me */
  }

  ///
  /// True if the current side to move is in check mate.
  ///
  pub fn is_checkmate( &self ) -> bool
  {
    self.pleco_board.checkmate()
  }

  ///
  /// Is the current side to move is in stalemate.
  ///
  pub fn is_stalemate( &self ) -> bool
  {
    self.pleco_board.stalemate()
  }

  ///
  /// Return the `Player` whose turn it is to move.
  ///
  pub fn current_turn( &self ) -> Player
  {
    self.pleco_board.turn()
  }

  ///
  /// Return the last move played, if any.
  ///
  pub fn last_move( &self ) -> Option< Move >
  {
    self.pleco_board.last_move()
  }

  ///
  /// Prints board to the terminal.
  ///
  pub fn print( &self ) /* qqq : remove. instead return string */
  {
    self.pleco_board.pretty_print();
  }

  ///
  /// Creates a 'Fen` string of the board.
  ///
  pub fn to_fen( &self ) -> Fen
  {
    self.pleco_board.fen()
  }
}

///
///Positions on the board in [FEN](https://www.chess.com/terms/fen-chess#what-is-fen) format
///
pub type Fen = String;

///
/// Contains information about move made in the past.
/// Field `fen` contains representation of the board as FEN string
/// Field `uci_move` contains move in UCI format
///
#[derive( Serialize, Deserialize, Debug )]
pub struct HistoryEntry
{
  fen : Fen,
  uci_move : String
}

///
/// Status of the game
///
#[derive( Debug, PartialEq )]
pub enum GameStatus
{
  /// The game is not finished, and the game is still in play.
  Continuing,
  /// The game has the winner.
  Checkmate,
  /// The game is drawn.
  Stalemate
}

///
/// Interface for playing chess game.
///
/// Basically Board + History.
///
#[derive( Serialize, Deserialize, Debug )]
pub struct Game
{
  #[serde(deserialize_with = "board_deserialize", serialize_with = "board_serialize")]
  board : Board,
  history : Vec<HistoryEntry>
}

impl Game
{
  ///
  /// Constructs a new game with default board setup
  ///
  pub fn default() -> Self
  {
    Self
    {
      board : Board::default(),
      history : Vec::new(),
    }
  }
  /* xxx : ? */

  ///
  /// Makes a move on the board. Accepts move in UCI format. For example, "e2e4".
  /// Updates histort and returns `true` if move was succesfuly applied, otherwise returns `false`.
  /// The board and history are not changed in case of fail.
  ///
  pub fn make_move( &mut self, uci_move : &str ) -> bool
  {
    let new_board = self.board.make_move( uci_move );
    let success = !new_board.is_none();
    if success
    {
      self.board = new_board.unwrap();
      self.history.push( HistoryEntry{ fen : self.board.to_fen(), uci_move : uci_move.to_string() } );
    }
    success
  }

  ///
  /// Return the [Player] whose turn it is to move.
  ///
  pub fn current_turn( &self ) -> Player
  {
    self.board.current_turn()
  }

  ///
  /// Prints board to the terminal.
  ///
  pub fn board_print( &self )
  {
    self.board.print();
  }

  ///
  /// Returns current game status as [GameStatus].
  ///
  pub fn status( &self ) -> GameStatus
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
  pub fn last_move( &self ) -> Option< String >
  {
    match self.history.last()
    {
      Some( h ) => Some( h.uci_move.clone() ),
      _ => None
    }
  }
}

//

fn board_deserialize< 'de, D >( deserializer : D ) -> Result< Board, D::Error >
where
  D : Deserializer< 'de >,
{
  let fen: String = Deserialize::deserialize( deserializer )?;
  Ok( Board::from_fen( &fen ) )
}

fn board_serialize< S >( board : &Board, s : S ) -> Result< S::Ok, S::Error >
where
  S: Serializer,
{
  s.serialize_str( &board.to_fen() )
}
