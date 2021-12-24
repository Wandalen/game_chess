
use std::iter::successors;

pub use pleco::
{
  core::Player,
  core::PieceType,
  core::Piece,
  board::piece_locations::PieceLocations, //Minimal board impl

  core::piece_move::MoveType,
  core::piece_move::BitMove as Move, //https://docs.rs/pleco/latest/pleco/core/piece_move/index.html,
  core::move_list::MoveList,

  core::sq::SQ as Cell,
  core::bitboard::BitBoard as CellsSet

};

/* Our structure:

Game = History + Board
   - history : Vec<Fen>
   - move
   - board
Board
  - pleco_board : pleco::Board
  - move
*/

/* List of resources to show

  tools::eval::Eval,
  helper::Helper,
  helper::prelude,
  bot_prelude

*/

/*
Commands

.game.new - creates game with default board
.game.from.fen - creates game [feature: game from fen]
[issue: implement command game.from.fen]

.games.list - list games [feature: persistence][issue: imlement persistency]
.game.open [id] - opens the game from storage [feature: persistence]
.game.save - saves cureent game state [feature: persistence]

.quit - exit
[issue: prompt for quit]
[issue: prompt for save][feature:persistence]

.resume [issue: implement timer ][feature: timer]
.pause [feature: timer]

.status - print board, current turn, last move
[issue:extend status to print score][feature:board score]

.move a1a2 - make a move

.moves.list - prints list of legal moves
[issue:moves list][feature:moves list]

.move.random - make a random legal move
[feature:moves list]
[issue:random move][feature:random move]

.moves.history - prints list of moves
[issue: history]

.move.undo - undo last move
[feature:history]
[issue:undo move][feature:undo move]

.gg - current player forfeits
[issue:forfeit][feature:forfeit]

.online.new
.online.list
.online.join
[feature: basic multiplayer]
[issue: multiplayer]

.online.spectate
[feature: basic multiplayer]
[feature: spectating]
[issue: spectating]

.online.msg
[feature: basic multiplayer]
[feature: chatting]
[issue: chatting]

*/

/*

Commands minimal

.game.new - creates game with default board
.quit - exit
.status - print board, current turn, last move
.move a1a2 - make a move

*/

pub struct Board
{
  pleco_board : pleco::Board
}

impl Board
{
  pub fn default() -> Self
  {
    Self
    {
      pleco_board : pleco::Board::start_pos()
    }
  }

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

  pub fn move_is_valid( &self, uci_move : &str ) -> bool
  {
    match self.move_from_uci( uci_move )
    {
      Some( m ) => self.pleco_board.pseudo_legal_move( m ) && self.pleco_board.legal_move( m ),
      _ => false
    }
  }

  pub fn move_from_uci( &self, uci_move : &str ) -> Option< Move >
  {
    let all_moves: MoveList = self.pleco_board.generate_moves();
    all_moves.iter()
              .find(| m | m.stringify() == uci_move )
              .cloned()
  }

  pub fn score( &self ) -> i32
  {
    0
  }

  pub fn is_checkmate( &self ) -> bool
  {
    self.pleco_board.checkmate()
  }

  pub fn is_stalemate( &self ) -> bool
  {
    self.pleco_board.stalemate()
  }

  pub fn current_turn( &self ) -> Player
  {
    self.pleco_board.turn()
  }

  pub fn print( &self )
  {
    self.pleco_board.pretty_print();
  }

  pub fn fen( &self ) -> Fen
  {
    self.pleco_board.fen()
  }
}

pub type Fen = String;

/// Interface for playing chess game

pub struct Game
{
  board : Board,
  history : Vec<Fen>
}

/// Status of the game
#[derive( Debug )]
pub enum GameStatus
{
  Continuing,
  Checkmate,
  Stalemate
}

impl Game
{
  pub fn default() -> Self
  {
    Self
    {
      board : Board::default(),
      history : Vec::new(),
    }
  }

  /// Makes a move on the board. Accepts move in UCI format. For example, "e2e4".
  /// Updates histort and returns `true` if move was succesfuly applied, otherwise returns `false`.
  /// The board and history are not changed in case of fail.
  pub fn make_move( &mut self, uci_move : &str ) -> bool
  {
    let new_board = self.board.make_move( uci_move );
    let success = !new_board.is_none();
    if success
    {
      self.board = new_board.unwrap();
      self.history.push( self.board.fen() );
    }
    success
  }

  pub fn current_turn( &self ) -> Player
  {
    self.board.current_turn()
  }

  pub fn board_print( &self )
  {
    self.board.print();
  }

  // pub fn print_current_turn( &self )
  // {
  //   println!( "Next move: {}", self.current_turn() );
  // }

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
}

/*
cargo test test_game -- --show-output
*/

#[test]
fn test_game()
{
  let mut game = Game::default();

  game.board_print();
  game.make_move( "a2a4" );
  game.board_print();
  println!( "{:?}", game.status() );
}