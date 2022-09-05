use super::chess_client::ChessClient;
use super::{GameId, CreateGame, AcceptGame, GamePlayer, Msg, GameMove};
use tonic::transport::channel::Channel;

///
/// Simple Session for Online MultiplayerGame
///
#[derive(Debug)]
pub struct ToySession
{
  player_id : Option<String>,
  game_id : Option<String>,
}

impl ToySession
{
  pub fn init() -> Self
  {
    ToySession {
      player_id : None,
      game_id : None,
    }
  }

  fn create(&mut self, player_id : &str, game_id : &str)
  {
    self.player_id = Some(player_id.to_string());
    self.game_id = Some(game_id.to_string());
  }

  fn get(&self) -> (String, String) { (self.player_id.clone().unwrap(), self.game_id.clone().unwrap()) }

  fn expired(&self) -> bool
  {
    if self.player_id == None || self.game_id == None
    {
      true
    }
    else
    {
      false
    }
  }

  fn update(&mut self)
  {
    self.player_id = Some(wca::input::ask("Input Player ID"));
    self.game_id = Some(wca::input::ask("Input Game ID"));
  }
}


///
/// Handler of multiplayer command `.help`.
///
pub fn command_help()
{
  println!("\nMultiplayer Commands:\n");

  println!(".online.new         => Create online multiplayer game");
  println!(".online.join        => Join online multiplayer game");
  println!(".online.move        => Make a move by providing move in UCI format: \"a2a4\"");
  println!(".online.moves.list  => Print all available moves in UCI format: \"a2a4\"");
  println!(".online.msg         => Send message to opponent");
  println!(".online.msg.read    => Read messages from opponent");
  println!(".online.status      => Print multiplayer board, current turn, last move");
}

///
/// Command to start new online game.
///
pub async fn command_game_new(session : &mut ToySession, rpc_server : &mut Option<ChessClient<Channel>>)
{
  if let Some(rpc_server) = rpc_server
  {
    let player_id = wca::input::ask("Input Player ID");
    let game_id = wca::input::ask("Input Game ID");
    println!("");

    let online_game = CreateGame {
      player : Some(game_chess_client::GamePlayer {
        player_id : player_id.clone(),
        game_id : game_id.clone(),
      }),
    };

    let result = rpc_server.push_game_create(online_game).await;
    match result
    {
      Ok(resp) =>
      {
        // Initiates ToySession
        session.create(&player_id, &game_id);

        clear_screen();
        println!("Invite others by sharing this Game ID: {}", resp.get_ref().game_id);

        let result = rpc_server.pull_board_state(GameId { game_id }).await;
        print_board_status(result);
      }
      Err(e) =>
      {
        eprintln!("{}", e.message());
      }
    }
  }
  else
  {
    eprintln!("Failed to connect gRPC server");
  }
}

///
/// Command to join an online game.
///
pub async fn command_game_join(session : &mut ToySession, rpc_server : &mut Option<ChessClient<Channel>>)
{
  if let Some(rpc_server) = rpc_server
  {
    let player_id = wca::input::ask("Input Player ID");
    let game_id = wca::input::ask("Input Game ID");
    println!("");

    let online_game = AcceptGame {
      game_id : game_id.clone(),
      player_id : Some(game_chess_client::GamePlayer {
        player_id : player_id.clone(),
        game_id : game_id.clone(),
      }),
    };

    let result = rpc_server.push_game_accept(online_game).await;
    match result
    {
      Ok(resp) =>
      {
        // Initiates ToySession
        session.create(&player_id, &game_id);

        clear_screen();
        println!("You have successfully joined Game ID: {}", resp.get_ref().game_id);

        let result = rpc_server.pull_board_state(GameId { game_id }).await;
        print_board_status(result);
      }
      Err(e) =>
      {
        eprintln!("{}", e.message());
      }
    }
  }
  else
  {
    eprintln!("Failed to connect gRPC server");
  }
}

///
/// Command to make a move on the board.
///
pub async fn command_game_move(session : &mut ToySession, rpc_server : &mut Option<ChessClient<Channel>>)
{
  if let Some(rpc_server) = rpc_server
  {
    // Session guard for ToySession
    if session.expired()
    {
      session.update();
    }

    let (player_id, game_id) = session.get();

    let r#move = wca::input::ask("Provide move in UCI format, for example 'a2a4'");

    let result = rpc_server
      .push_move(GameMove {
        player_id,
        game_id,
        r#move,
      })
      .await
      .ok();

    clear_screen();
    println!("\n{}", result.unwrap().into_inner().board_state);
  }
  else
  {
    eprintln!("Failed to connect gRPC server");
  }
}

///
/// Command to print available moves
///
pub async fn command_game_moves_list(session : &mut ToySession, rpc_server : &mut Option<ChessClient<Channel>>)
{
  if let Some(rpc_server) = rpc_server
  {
    // Session guard for ToySession
    if session.expired()
    {
      session.update();
    }

    let (_, game_id) = session.get();
    let moves_list = rpc_server.pull_moves(GameId { game_id }).await.ok();
    let moves_list = moves_list.unwrap().into_inner().moves_list;

    println!("********[ Available Moves ]********");
    for r#move in moves_list
    {
      println!("{}", r#move);
    }
  }
  else
  {
    eprintln!("Failed to connect gRPC server");
  }
}

///
/// Command to send message to opponent.
///
pub async fn command_game_send_msg(session : &mut ToySession, rpc_server : &mut Option<ChessClient<Channel>>)
{
  if let Some(rpc_server) = rpc_server
  {
    // Session guard for ToySession
    if session.expired()
    {
      session.update();
    }

    let (player_id, game_id) = session.get();

    let text = wca::input::ask("Write Your Message");

    let player = GamePlayer { player_id, game_id };
    rpc_server
      .push_msg(Msg {
        player : Some(player),
        text,
      })
      .await
      .ok();
    println!("Your message has been sent");
  }
  else
  {
    eprintln!("Failed to connect gRPC server");
  }
}

///
/// Command to read messages from opponent.
///
pub async fn command_game_read_msgs(session : &mut ToySession, rpc_server : &mut Option<ChessClient<Channel>>)
{
  if let Some(rpc_server) = rpc_server
  {
    // Session guard for ToySession
    if session.expired()
    {
      session.update();
    }

    let (player_id, game_id) = session.get();

    let player = GamePlayer { player_id, game_id };

    let result = rpc_server.read_msgs(player).await;
    let chats = result.unwrap().into_inner().messages;

    println!("********[ Chat Messages ]********");
    for chat in chats
    {
      println!("{}", chat);
    }
  }
  else
  {
    eprintln!("Failed to connect gRPC server");
  }
}

///
/// Command to print status of the multiplayer game.
///
pub async fn command_game_status(session : &mut ToySession, rpc_server : &mut Option<ChessClient<Channel>>)
{
  if let Some(rpc_server) = rpc_server
  {
    // Session guard for ToySession
    if session.expired()
    {
      session.update();
    }

    let (_, game_id) = session.get();
    let result = rpc_server.pull_board_state(GameId { game_id }).await;

    print_board_status(result);
  }
  else
  {
    eprintln!("Failed to connect gRPC server");
  }
}


fn print_board_status(board : Result<tonic::Response<game_chess_client::Board>, tonic::Status>)
{
  match board
  {
    Ok(board) =>
    {
      println!("\n{}", board.into_inner().board_state);
    }
    Err(e) =>
    {
      eprintln!("\n{}", e.message());
    }
  }
}

fn clear_screen()
{
  if let Err(_) = std::process::Command::new("clear").status()
  {
    std::process::Command::new("cmd").args(&["/c", "cls"]).status().ok();
  }
}
