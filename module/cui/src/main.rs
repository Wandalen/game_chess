#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Command user interface ( CLI ) for chess game implemented for educational purpose.
//!

/*
Commands

.game.new - creates game with default board
.game.from.fen - creates game [feature: game from fen]
[issue: implement command game.from.fen]

.games.list - list games [feature: persistence][issue: implement persistency]
.game.open [id] - opens the game from storage [feature: persistence]
.game.save - saves current game state [feature: persistence]

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
//#[ allow( unused_imports ) ]
//use game_chess_client::*;
mod multiplayer;

///
/// Main. CLI game itself.
///
#[ tokio::main ]
pub async fn main()
{
  let mut game : Option< Game > = None;
  let mut choice;

  let mut session = multiplayer::ToySession::init();

  let remote_rpc = game_chess_client::Client::connect( "http://127.0.0.1:1313" ).await;
  let mut remote_rpc = if let Ok( remote ) = remote_rpc { Some( remote ) } else { None };

  command_help();

  /* Dmytro : please, use it with command for network game */
  // let _chess_client = chess_client::ChessClient::connect("http://[::1]:50051")
  //   .await
  //   .expect("Failed to connect to the Chess server");

  loop
  {
    println!();

    choice = wca::input::ask( "\nPlease enter command" );

    match choice.to_lowercase().trim()
    {
      ".game.new" => game = Some( command_game_new() ),
      ".game.new.ai" | ".new.ai" => game = command_game_new_ai(),
      ".game.save" => command_game_save( &game ),
      ".games.list" => command_list_saved_games(),
      ".game.from.fen" => game = Some( command_game_from_fen() ),
      ".move" | ".m" => command_move( &mut game ),
      ".move.undo" => command_move_undo( &mut game ),
      ".move.redo" => command_move_redo( &mut game ),
      ".move.random" => command_random_move( &mut game ),
      ".gg" => command_forfeit( &mut game ),
      ".moves.list" => command_moves_list( &game ),
      ".move.ai" => command_move_ai( &mut game ),
      ".status" | ".s" => command_status( &game ),
      ".moves.history" | ".m.h" => command_moves_history( &game ),
      ".quit" => command_exit( &game ),
      ".resume" => command_resume( &mut game ),
      ".pause" => command_pause( &mut game ),
      ".help" => command_help(),
      ".score" => command_score( &game ),

      ".online.new" => multiplayer::command_game_new( &mut session, &mut remote_rpc ).await,
      ".online.join" => multiplayer::command_game_join( &mut session, &mut remote_rpc ).await,
      ".online.move" => multiplayer::command_game_move( &mut session, &mut remote_rpc ).await,
      ".online.moves.list" => multiplayer::command_game_moves_list( &mut session, &mut remote_rpc ).await,
      ".online.msg" => multiplayer::command_game_send_msg( &mut session, &mut remote_rpc ).await,
      ".online.msg.read" => multiplayer::command_game_read_msgs( &mut session, &mut remote_rpc ).await,
      ".online.status" => multiplayer::command_game_status( &mut session, &mut remote_rpc ).await,

      command => println!( "Unknown command : {}\n", command ),
    }
  }
}

///
/// Handler of command `.help`.
///

pub fn command_help()
{
  println!( "\nCommands:\n" );

  println!( ".game.new      => Create game with default board" );
  println!( ".new.ai        => Create game with ai. Also shortcut for .game.new.ai" );
  println!( ".game.save     => Save game to file" );
  println!( ".games.list    => Print list saved games" );
  println!( ".game.from.fen => Load game from FEN" );
  println!( ".move          => Make a move by providing move in UCI format: \"a2a4\" " );
  println!( ".move.undo     => Move undo" );
  println!( ".move.redo     => Move redo" );
  println!( ".move.random   => Make a random move" );
  println!( ".gg            => Forfeit the game " );
  println!( ".moves.list    => Print all available moves in UCI format: \"a2a4\" " );
  println!( ".move.ai       => Ask the AI to make a move for the player" );
  println!( ".status        => Print board, current turn, last move" );
  println!( ".moves.history => Print moves history" );
  println!( ".pause         => Command to pause the game" );
  println!( ".resume        => Command to resume the game" );
  println!( ".score         => Print score" );
  println!( ".quit          => Exit from the game" );
  println!( ".help          => Print this help" );

  multiplayer::command_help();
}

///
/// Command to quit the game.
///

pub fn command_exit( game : &Option< Game > )
{
  let uci_exit = wca::input::ask( "Do you want to exit?" );
  match uci_exit.to_lowercase().trim()
  {
    "yes" | "y" =>
    {
      println!( "Exiting.." );
      std::process::exit( 0 );
    }
    _ => command_status( game ),
  }
}

///
/// Command to resume the game
///

pub fn command_resume( game : &mut Option< Game > )
{
  if game.is_none()
  {
    println!( "Create a game first. Use command: .game.new" );
    return;
  }

  let game = game.as_mut().unwrap();
  game.resume();
  println!( "Game resumed" );
}

///
/// Command to pause the game
///

pub fn command_pause( game : &mut Option< Game > )
{
  if game.is_none()
  {
    println!( "Create a game first. Use command: .game.new" );
    return;
  }

  let game = game.as_mut().unwrap();
  game.pause();
  println!( "Game paused" );
}

fn timer_setup() -> Option< timer::Timer >
{
  let use_it = wca::input::ask( "\nDo you want to use timer? (default = no)" );
  match use_it.to_lowercase().trim()
  {
    "yes" | "y" =>
    {
      println!();
      println!( "[1] => 10min + 50s" );
      println!( "[2] => 5min + 3s" );
      println!( "[3] => 3min" );
      println!( "[0] => set your value" );
      let settings = wca::input::ask( "\nPlease, select time settings? (default = 1)" );
      match settings.as_str()
      {
        "1" => Some( timer::Timer::new( 10 * 60, 50 ) ),
        "2" => Some( timer::Timer::new( 5 * 60, 3 ) ),
        "3" => Some( timer::Timer::new( 3 * 60, 0 ) ),
        "0" =>
        {
          let value = wca::input::ask( "\nEnter number of seconds for player" );
          let value = match value.parse()
          {
            Ok( value ) => value,
            Err( _ ) => { println!( "Failed to parse number." ); return None; }
          };
          let bonuses = wca::input::ask( "\nEnter number of seconds for bonuses" );
          let bonuses = match bonuses.parse()
          {
            Ok( value ) => value,
            Err( _ ) => { println!( "Failed to parse number." ); return None; }
          };

          Some( timer::Timer::new( value, bonuses ) )
        }
        _ => Some( timer::Timer::new( 10 * 60, 50 ) )
      }

    },
    _ => None
  }
}

///
/// Command to start new game.
///

pub fn command_game_new() -> Game
{
  let mut game = Game::default();
  game.timer = timer_setup();
  println!();
  game.board_print();
  game.timers_print();
  println!( "Turn of {}", game.current_turn() );
  game
}

///
/// Command to start new game with AI
///

pub fn command_game_new_ai() -> Option< Game >
{
  let mut algorithm = wca::input::ask( "\nPlease select the ai engine algorithm (default = iterative)" );
  if algorithm.is_empty()
  {
    algorithm = String::from( "iterative" )
  }
  let mut engine = match ai::Engine::new( algorithm )
  {
    Ok( engine ) => engine,
    Err( _ ) =>
    {
      println!( "Unknown engine type, please try again." );
      return None;
    }
  };

  let mut depth = wca::input::ask( "\nPlease select the ai engine depth (default = 5)" );
  if depth.is_empty()
  {
    depth = String::from( "5" );
  }
  match depth.parse::< u16 >()
  {
    Ok( depth ) => engine.depth = depth,
    Err( _ ) =>
    {
      println!( "Failed to parse number." );
      return None;
    }
  };

  let mut game = Game::default();
  game.ai = Some( engine );
  game.timer = timer_setup();

  println!();
  game.board_print();
  game.timers_print();
  println!( "Turn of {}", game.current_turn() );
  Some( game )
}

///
/// Command to print status of the game.
///

pub fn command_status( game : &Option< Game > )
{
  if game.is_none()
  {
    println!( "Create a game first. Use command: .game.new" );
    return;
  }

  let game = game.as_ref().unwrap();

  println!();

  game.board_print();
  game.timers_print();

  println!( "Current turn: {}", game.current_turn() );

  match game.last_move()
  {
    Some( m ) => println!( "Last move: {}", m.0 ),
    _ => println!( "Last move: None" ),
  }
}

///
/// Command to save game to file.
///

pub fn command_game_save( game : &Option< Game > )
{
  if game.is_none()
  {
    println!( "Create a game first. Use command: .game.new" );
    return;
  }

  let game = game.as_ref().unwrap();

  let save_path = game.save();

  println!( "Saved game to file: {}", save_path.unwrap() );
}

///
/// Command to print list of saved games.
///

pub fn command_list_saved_games()
{
  if let Some( list ) = list_saved_games()
  {
    for path in list
    {
      let filename = path.file_name().unwrap().to_str().unwrap();
      println!( "Game: {}", filename );
    }
  }
  else 
  {
    println!( "No saved games!" );    
  }
}

///
/// Command to make a move.
///

pub fn command_move( game : &mut Option< Game > )
{
  let Some(game) = game else {
    println!( "Create a game first. Use command: .game.new" );
    return;
  };

  let uci_move = wca::input::ask( "Provide move in UCI format, for example 'a2a4'" );
  if game.make_move( UCI( uci_move.clone() ) )
  {
    if game.has_ai()
    {
      game.make_move_ai();
    }
  }
  else
  {
    println!( "\n\x1b[93mFailed to apply move: '{}'. Try again!\x1b[0m", uci_move );
  }

  println!();
  game.board_print();
  game.timers_print();
  println!( "Turn of {}", game.current_turn() );
}

///
/// Command move undo
/// 

pub fn command_move_undo( game : &mut Option< Game > )
{
  if game.is_none()
  {
    println!( "Create a game first. Use command: .game.new" );
    return;
  }

  let game = game.as_mut().unwrap();
  game.move_undo();

  // println!();
  // game.board_print();
  // game.timers_print();
  // println!( "Turn of {}", game.current_turn() );
}

///
/// Command move redo
/// 

pub fn command_move_redo( game : &mut Option< Game > )
{
  if game.is_none()
  {
    println!( "Create a game first. Use command: .game.new" );
    return;
  }

  let game = game.as_mut().unwrap();
  game.move_redo();
}

///
/// Command make a random move
///

pub fn command_random_move( game : &mut Option< Game > )
{
  let Some(game) = game else {
    println!( "Create a game first. Use command: .game.new" );
    return;
  };

  if game.make_random_move() && game.has_ai()
  {
    game.make_move_ai();
  }

  println!();
  game.board_print();
  game.timers_print();
  println!( "Turn of {}", game.current_turn() );
}

///
/// Command to forfeit.
///

pub fn command_forfeit( game : &mut Option< Game > )
{
  if game.is_none()
  {
    println!( "Create a game first. Use command: .game.new" );
    return;
  }

  let uci_exit = wca::input::ask( "Do you want to forfeit?" );
  match uci_exit.to_lowercase().trim()
  {
    "yes" =>
    {
      let game = game.as_mut().unwrap();
      game.forfeit();

      let player = game.current_turn();
      println!( "{:?} lose the game.", player );

      println!( "Exiting.." );
      std::process::exit( 0 );
    }
    _ => command_status( game ),
  }
}

///
/// Wrapper and control flow
///

pub fn command_score( game : &Option< Game > )
{
  match game
  {
    Some( g ) => println!( "{}", g.count_score() ),
    None => println!( "Game not found" ),
  }
}

///
/// Command to print moves history.
///

pub fn command_moves_history( game : &Option< Game > )
{
  println!();
  if game.is_none()
  {
    println!( "There is no history yet" );
    return;
  }

  game.as_ref().unwrap().history_print();
}

///
/// Command to print available moves
///

pub fn command_moves_list( game : &Option< Game > )
{
  if game.is_none()
  {
    println!( "Create a game first. Use command: .game.new" );
    return;
  }

  let game = game.as_ref().unwrap();
  let moves_list = game.moves_list();
  for legal_move in moves_list
  {
    println!( "{}", legal_move );
  }
}

///
/// Load game from FEN
///

pub fn command_game_from_fen() -> Game
{
  let line = wca::input::ask( "Input FEN" );
  let game = Game::from_fen( &line );
  println!();
  game.board_print();
  game.timers_print();
  println!( "Turn of {}", game.current_turn() );
  game
}

///
/// Command to ask the AI to make a move
///

pub fn command_move_ai( game : &mut Option< Game > )
{
  if game.is_none()
  {
    println!( "Create a game first. Use command: .game.new" );
    return;
  }

  let game = game.as_mut().unwrap();
  game.make_move_ai();
  println!();
  game.board_print();
  game.timers_print();
  println!( "Turn of {}", game.current_turn() );
}
