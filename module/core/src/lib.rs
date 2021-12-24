
pub use pleco::
{
  core::Piece,
  core::PieceType,
  core::Player,
  core::move_list::MoveList,
  core::piece_move::BitMove, //https://docs.rs/pleco/latest/pleco/core/piece_move/index.html,
  core::piece_move::MoveType,
  core::sq::SQ,

  board::piece_locations::PieceLocations,
  board::castle_rights::Castling,
  board::Board, // Main struct in pleco
  board::board_state::BoardState,
  board::BoardError,

  tools::eval::Eval,

  helper::Helper,
  helper::prelude,

  bot_prelude
};

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

/*

Game = History + Board
Board

*/


/// Interface for playing chess game
/// ```
/// let game = Game::default();
/// game.print_board();
/// ```
pub struct Game
{
  board : Board

  //moves history
}

/// Status of the game
#[derive( Debug )]
pub enum GameStatus
{
  Continuing,
  Checkmate,
  Stalemate,
  Error( BoardError )
}

impl Default for Game
{
  fn default() -> Self
  {
    Self
    {
      board : Board::start_pos()
    }
  }
}


impl Game
{
  pub fn status( &self ) -> GameStatus
  {
    if self.board.checkmate()
    {
      return GameStatus::Checkmate;
    }

    if self.board.stalemate()
    {
      return GameStatus::Stalemate
    }

    if !self.board.is_ok_quick()
    {
      return match self.board.is_okay()
      {
        Ok( _ ) => panic!( "Unexpected behavior: board.is_ok_quick() returned false, but board.is_okay() says that board is ok" ),
        Err( err ) => GameStatus::Error( err )
      }
    }

    return GameStatus::Continuing
  }

  pub fn apply_move_u8( &mut self, src : u8, dst : u8 ) -> bool
  {
    let mv = BitMove::make_quiet( SQ( src ), SQ( dst ) );
    self.apply_move( mv )
  }

  pub fn apply_move_sq( &mut self, src : SQ, dst : SQ ) -> bool
  {
    let mv = BitMove::make_quiet( src, dst );
    self.apply_move( mv )
  }

  pub fn apply_move( &mut self, mv : BitMove ) -> bool
  {
    let result = self.move_is_valid( mv );

    if result
    {
      self.board.apply_move( mv );
    }

    result
  }

  pub fn move_is_valid( &self, mv : BitMove ) -> bool
  {
    self.board.pseudo_legal_move( mv ) && self.board.legal_move( mv )
  }

  pub fn current_turn( &self ) -> Player
  {
    self.board.turn()
  }

  pub fn print_board( &self )
  {
    self.board.pretty_print();
  }

  pub fn print_current_turn( &self )
  {
    self.print_board();
    println!( "Next move: {}", self.current_turn() );
  }
}

/*
cargo test test_game -- --show-output
*/

#[test]
fn test_game()
{
  let mut game = Game::default();
  game.print_current_turn();
  // game.apply_move_u8( 8, 16 );
  // game.print_current_turn();
  // game.apply_move_sq( SQ( 49 ), SQ( 41 ) );
  // game.print_current_turn();
  // println!( "Game status: {:?}", game.status() );
}