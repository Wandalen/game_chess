#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

//!
//! Command user interface ( CLI ) for chess game implemented for educational purpose.
//!

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

///
/// Main. CLI game itself.
///

pub fn main()
{
  let mut game : Option<Game> = None;
  let mut choice;

  command_help();

  loop
  {
    println!("");

    choice = wca::input::ask("\nPlease enter command");

    match choice.to_lowercase().trim()
    {
      ".game.new" => game = Some(command_game_new()),
      ".game.save" => command_game_save(&game),
      ".move" | ".m" => command_move(&mut game),
      ".status" | ".s" => command_status(&game),
      ".quit" => command_exit(&game),
      ".help" => command_help(),
      command => println!("Unknown command : {}\n", command),
    }
  }
}

///
/// Handler of command `.help`.
///

pub fn command_help()
{
  println!("");

  println!("Commands:");

  println!("");

  println!(".game.new  => Create game with default board");
  println!(".game.save => Save game to file");
  println!(".move      => Make a move by providing move in UCI format: \"a2a4\" ");
  println!(".status    => Print board, current turn, last move");
  println!(".quit      => Exit from the game");
  println!(".help      => Print this help");
}

///
/// Command to quit the game.
///

pub fn command_exit(game : &Option<Game>)
{
  let uci_exit = wca::input::ask("Do you want to exit?");
  match uci_exit.to_lowercase().trim()
  {
    "yes" =>
    {
      println!("Exiting..");
      std::process::exit(0);
    }
    _ => command_status(&game),
  }
}

///
/// Command to start new game.
///

pub fn command_game_new() -> Game
{
  let game = Game::default();
  println!("");
  game.board_print();
  println!("Turn of {}", game.current_turn());
  game
}

///
/// Command to print status of the game.
///

pub fn command_status(game : &Option<Game>)
{
  if game.is_none()
  {
    println!("Create a game first. Use command: .game.new");
    return;
  }

  let game = game.as_ref().unwrap();

  println!("");

  game.board_print();

  println!("Current turn: {}", game.current_turn());

  match game.last_move()
  {
    Some(m) => println!("Last move: {}", m.0),
    _ => println!("Last move: None"),
  }
}

///
/// Command to save game to file.
///

pub fn command_game_save(game : &Option<Game>)
{
  if game.is_none()
  {
    println!("Create a game first. Use command: .game.new");
    return;
  }

  let game = game.as_ref().unwrap();

  let save_path = game.save();

  println!("Saved game to file: {}", save_path.unwrap());
}

///
/// Command to make a move.
///

pub fn command_move(game : &mut Option<Game>)
{
  if game.is_none()
  {
    println!("Create a game first. Use command: .game.new");
    return;
  }

  let game = game.as_mut().unwrap();

  let uci_move = wca::input::ask("Provide move in UCI format, for example 'a2a4'");
  if !game.make_move(UCI(uci_move.clone()))
  {
    println!("\n\x1b[93mFailed to apply move: '{}'. Try again!\x1b[0m", uci_move);
  }
  println!("");
  game.board_print();
  println!("Turn of {}", game.current_turn());
}
