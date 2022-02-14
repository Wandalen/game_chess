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
#[allow(unused_imports)]
use game_chess_client::*;

///
/// Main. CLI game itself.
///
#[tokio::main]
pub async fn main()
{
  let mut game : Option<Game> = None;
  let mut choice;

  command_help();

  /* Dmytro : please, use it with command for network game */
  // let _chess_client = chess_client::ChessClient::connect("http://[::1]:50051")
  //   .await
  //   .expect("Failed to connect to the Chess server");

  loop
  {
    println!("");

    choice = wca::input::ask("\nPlease enter command");

    match choice.to_lowercase().trim()
    {
      ".game.new" => game = Some(command_game_new()),
      ".game.new.ai" | ".new.ai" => game = command_game_new_ai(),
      ".game.save" => command_game_save(&game),
      ".game.from.fen" => game = Some(command_game_from_fen()),
      ".move" | ".m" => command_move(&mut game),
      ".gg" => command_forfeit(&mut game),
      ".online.new" => command_online_game_new().await,
      ".online.join" => command_online_game_join().await,
      ".moves.list" => command_moves_list(&game),
      ".move.ai" => command_move_ai(&mut game),
      ".status" | ".s" => command_status(&game),
      ".moves.history" | ".m.h" => command_moves_history(&game),
      ".quit" => command_exit(&game),
      ".help" => command_help(),
      ".score" => command_score(&game),
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
  println!(".new.ai    => Create game with ai. Also shortcut for .game.new.ai");
  println!(".game.save => Save game to file");
  println!(".game.from.fen => Load game from FEN");
  println!(".move      => Make a move by providing move in UCI format: \"a2a4\" ");
  println!(".gg        => Forfeit the game ");
  println!(".online.new => Create online multiplayer game ");
  println!(".online.join  => Join online multiplayer game ");
  println!(".moves.list=> Print all available moves in UCI format: \"a2a4\" ");
  println!(".move.ai   => Ask the AI to make a move for the player");
  println!(".status    => Print board, current turn, last move");
  println!(".moves.history => Print moves history");
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
    "yes" | "y" =>
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
/// Command to start new game with AI
///

pub fn command_game_new_ai() -> Option<Game>
{
  let mut algorithm = wca::input::ask("\nPlease select the ai engine algorithm (default = iterative)");
  if algorithm.is_empty() {
    algorithm = String::from("iterative")
  }
  let mut engine = match ai::Engine::new(algorithm) {
    Ok(engine) => engine,
    Err(_) => {
      println!("Unknown engine type, please try again.");
      return None;
    }
  };

  let mut depth= wca::input::ask("\nPlease select the ai engine depth (default = 5)");
  if depth.is_empty() {
    depth = String::from("5");
  }
  match depth.parse::<u16>() {
    Ok(depth) => engine.depth = depth,
    Err(_) => {
      println!("Failed to parse number.");
      return None;
    }
  };

  let mut game = Game::default();
  game.ai = Some(engine);

  println!("");
  game.board_print();
  println!("Turn of {}", game.current_turn());
  Some(game)
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
  if game.make_move(UCI(uci_move.clone())) {
    if game.has_ai() {
      game.make_move_ai();
    }
  } else {
    println!("\n\x1b[93mFailed to apply move: '{}'. Try again!\x1b[0m", uci_move);
  }

  println!("");
  game.board_print();
  println!("Turn of {}", game.current_turn());
}

///
/// Command to forfeit.
///

pub fn command_forfeit(game : &mut Option<Game>)
{
  if game.is_none()
  {
    println!("Create a game first. Use command: .game.new");
    return;
  }

  let uci_exit = wca::input::ask("Do you want to forfeit?");
  match uci_exit.to_lowercase().trim()
  {
    "yes" =>
    {
      let game = game.as_mut().unwrap();
      game.forfeit();

      let player = game.current_turn();
      println!("{:?} lose the game.", player);

      println!("Exiting..");
      std::process::exit(0);
    }
    _ => command_status(&game),
  }
}

///
/// Wrapper and control flow
///

pub fn command_score(game : &Option<Game>)
{
  match game
  {
    Some(g) => println!("{}", g.count_score()),
    None => println!("Game not found"),
  }
}

///
/// Command to print moves history.
///

pub fn command_moves_history(game : &Option<Game>)
{
  println!();
  if game.is_none()
  {
    println!("There is no history yet");
    return;
  }

  game.as_ref().unwrap().history_print();
}

///
/// Command to print available moves
///

pub fn command_moves_list(game : &Option<Game>)
{
  if game.is_none()
  {
    println!("Create a game first. Use command: .game.new");
    return;
  }

  let game = game.as_ref().unwrap();
  let moves_list = game.moves_list();
  for legal_move in moves_list
  {
    println!("{}", legal_move.to_string());
  }
}

///
/// Load game from FEN
///

pub fn command_game_from_fen() -> Game
{
  let line = wca::input::ask("Input FEN");
  let game = Game::from_fen(&line);
  println!("");
  game.board_print();
  println!("Turn of {}", game.current_turn());
  game
}

///
/// Command to ask the AI to make a move
///

pub fn command_move_ai(game : &mut Option<Game>)
{
  if game.is_none()
  {
    println!("Create a game first. Use command: .game.new");
    return;
  }

  let game = game.as_mut().unwrap();
  game.make_move_ai();
  println!("");
  game.board_print();
  println!("Turn of {}", game.current_turn());
}

///
/// Command to start new online game.
///

pub async fn command_online_game_new()
{
  if let Ok(mut chess_client) = chess_client::ChessClient::connect("http://127.0.0.1:1313").await {
    let player_id = wca::input::ask("Input Player ID");
    let player_name = wca::input::ask("Input Player Name");
    println!("");

    let online_game = CreateGame { player: Some(game_chess_client::Player { player_id, player_name })};
    let result = chess_client.push_game_create(online_game).await;
    match result {
      Ok(resp) => { println!("Your sharable game ID: {}", resp.get_ref().game_id); }
      Err(e) => { eprintln!("{}", e); }
    }
  } else {
    println!("Failed to connect gRPC server");
  }
}

///
/// Command to join an online game.
///
///
pub async fn command_online_game_join()
{
  if let Ok(mut chess_client) = chess_client::ChessClient::connect("http://127.0.0.1:1313").await {
    let game_id = wca::input::ask("Input Game ID");
    let player_id = wca::input::ask("Input Your Player ID");
    let player_name = wca::input::ask("Input Your Player Name");
    println!("");

    let online_game = AcceptGame {
      game_id: game_id.to_string(),
      player_id: Some(game_chess_client::Player { player_id, player_name })
    };

    let result = chess_client.push_game_accept(online_game).await;
    match result {
      Ok(resp) => {
        println!("You have joined game ID: {}", resp.get_ref().game_id);
        println!("Games list: {:?}", chess_client.pull_games_list(()).await)
      }
      Err(e) => { eprintln!("{}\nGame ID: {} Not found on server", e, game_id); }
    }
  } else {
    println!("Failed to connect gRPC server");
  }
}


#[cfg(test)]
mod online_multiplayer_game_tests
{
  use super::*;

  // Run following tests with `cargo test --bin cui`

  #[tokio::test]
  async fn online_game_new()
  {
    let online_game = CreateGame {
      player: Some(game_chess_client::Player {
        player_id: "01".to_string(),
        player_name: "John Doe".to_string()
      })
    };

    /* qqq : need to test server before or even start it */
    if let Ok(mut chess_client) = chess_client::ChessClient::connect("http://127.0.0.1:1313").await {
      let resp = chess_client.push_game_create(online_game).await;
      let game_id = resp.unwrap().get_ref().game_id.to_string();

      // `push_game_create` returns a Game ID
      // `game_id` is a random string of length 6
      assert_eq!(game_id.len(), 6);
    } /* else {
      panic!("Failed to connect gRPC server");
    } */
  }

  #[tokio::test]
  async fn online_game_join()
  {
    let online_game = CreateGame {
      player: Some(game_chess_client::Player {
        player_id: "01".to_string(),
        player_name: "John Doe".to_string()
      })
    };

    /* qqq : need to test server before or even start it */
    if let Ok(mut chess_client) = chess_client::ChessClient::connect("http://127.0.0.1:1313").await {
      let resp = chess_client.push_game_create(online_game).await;
      let game_id = resp.unwrap().get_ref().game_id.to_string();

      let online_game = AcceptGame {
        game_id: game_id.clone(),
        player_id: Some(game_chess_client::Player {
          player_id: "02".to_string(),
          player_name: "Jane Doe".to_string()
        })
      };

      let resp = chess_client.push_game_accept(online_game).await;
      let joined_game_id = resp.unwrap().get_ref().game_id.to_string();

      assert_eq!(game_id, joined_game_id);
    } /* else {
      panic!("Failed to connect gRPC server");
    } */
  }
}
