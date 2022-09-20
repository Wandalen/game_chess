/*
  cargo test --test online_multiplayer
  cargo test --test online_multiplayer -- --nocapture
*/

use tonic::transport::Server;
use game_chess_server::rpc_server::ChessRpcServer;
use game_chess_client::*;

async fn run_test_server(addr : &str)
{
  let chess_grpc_server = ChessRpcServer::init();

  let addr = addr.parse().unwrap();
  println!("Test Server listening on {}", addr);

  tokio::spawn(async move {
    Server::builder()
      .add_service(chess_server::ChessServer::new(chess_grpc_server))
      .serve(addr)
      .await
      .unwrap();
  });
}

#[tokio::test]
async fn online_game_new()
{
  run_test_server("0.0.0.0:3001").await;

  let online_game = CreateGame {
    player : Some(game_chess_client::GamePlayer {
      player_id : "01".to_string(),
      game_id : "01".to_string(),
    }),
  };

  let mut chess_client = chess_client::ChessClient::connect("http://localhost:3001").await.unwrap();
  let resp = chess_client.push_game_create(online_game).await;
  let game_id = resp.unwrap().get_ref().game_id.to_string();

  assert_eq!(game_id, "01".to_owned());
}

#[tokio::test]
async fn online_game_join()
{
  run_test_server("0.0.0.0:3002").await;

  let online_game = CreateGame {
    player : Some(game_chess_client::GamePlayer {
      player_id : "01".to_string(),
      game_id : "01".to_string(),
    }),
  };

  let mut chess_client = chess_client::ChessClient::connect("http://localhost:3002").await.unwrap();
  let resp = chess_client.push_game_create(online_game).await;
  let game_id = resp.unwrap().get_ref().game_id.to_string();

  let online_game = AcceptGame {
    game_id : "01".to_string(),
    player_id : Some(game_chess_client::GamePlayer {
      player_id : "02".to_string(),
      game_id : "01".to_string(),
    }),
  };

  let resp = chess_client.push_game_accept(online_game).await;
  let joined_game_id = resp.unwrap().get_ref().game_id.to_string();

  assert_eq!(game_id, joined_game_id);
}

#[tokio::test]
async fn online_game_send_receive_msg()
{
  run_test_server("0.0.0.0:3003").await;

  let mut chess_client = chess_client::ChessClient::connect("http://localhost:3003").await.unwrap();
  let player = Some(GamePlayer {
    player_id : "01".to_owned(),
    game_id : "01".to_owned(),
  });
  chess_client
    .push_msg(Msg {
      player,
      text : "Hello, Player!".to_owned(),
    })
    .await
    .ok();

  let player = GamePlayer {
    player_id : "02".to_owned(),
    game_id : "01".to_owned(),
  };
  let resp = chess_client.read_msgs(player).await;
  let chats = resp.unwrap().into_inner().messages;
  let chat : Vec<&str> = chats[0].split(">> ").collect();

  assert_eq!(chat[1], "Hello, Player!");
}
