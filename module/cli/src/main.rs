#![warn( missing_debug_implementations, missing_docs )]

//! Command interface for chess game

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

use game_chess_core::*;

//

fn main()
{
  let mut game: Option< Game > = None;
  let mut choice;

  command_help();

  loop
  {
    println!( "" );

    choice = wca::input::ask( "\nPlease enter command" );

    match choice.to_lowercase().trim()
    {
      ".game.new" => { game = Some( command_game_new() ) },
      ".move" => command_move( &mut game ),
      ".status" => command_status( &game ),
      ".quit" => command_exit(),
      ".help" => command_help(),
      command => println!( "Unknown command : {}\n", command ),
    }

  }

}

/// Handler of command `.help`.
fn command_help()
{
  println!( "" );

  println!( "Commands:" );

  println!( "" );

  println!( ".game.new => Create game with default board" );
  println!( ".move     => Make a move by providing move in UCI format: \"a2a4\" " );
  println!( ".status   => Print board, current turn, last move" );
  println!( ".quit     => Exit from the game" );
  println!( ".help     => Print this help" );
}

/// Handler of command `.exit`.
fn command_exit()
{
  println!( "Exiting.." );
  std::process::exit( 0 );
}

/// Handler of command `.game.new`.
fn command_game_new() -> Game
{
  let game = Game::default();
  println!( "" );
  game.board_print();
  println!( "Turn of {}", game.current_turn() );
  game
}

/// Handler of command `.status`.
fn command_status( game : &Option<Game> )
{
  if game.is_none()
  {
    println!( "Create a game first. Use command: .game.new" );
    return;
  }

  let game = game.as_ref().unwrap();

  println!( "" );

  game.board_print();

  println!( "Current turn: {}", game.current_turn() );

  match game.last_move()
  {
    Some( m ) => println!( "Last move: {}", m ),
    _ => println!( "Last move: None" ),
  }
}

/// Handler of command `.move`.
fn command_move( game : &mut Option<Game>  )
{
  if game.is_none()
  {
    println!( "Create a game first. Use command: .game.new" );
    return;
  }

  let game = game.as_mut().unwrap();

  let uci_move = wca::input::ask( "Provide move in UCI format, for example 'a2a4'" );
  if !game.make_move( uci_move.as_str() )
  {
    println!( "\n\x1b[93mFailed to apply move: '{}'. Try again!\x1b[0m", uci_move );
  }
  println!( "" );
  game.board_print();
  println!( "Turn of {}", game.current_turn() );
}
