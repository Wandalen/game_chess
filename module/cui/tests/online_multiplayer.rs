/*
  cargo test --test online_multiplayer 
  cargo test --test online_multiplayer -- --nocapture
*/

use tonic::transport::Server;
use game_chess_server::rpc_server::ChessRpcServer;
use game_chess_client::*;


async fn run_test_server(addr: &str)
{
  let chess_grpc_server = ChessRpcServer::init();

  let addr = addr.parse().unwrap();
  println!("Test Server listening on {}", addr);

  tokio::spawn(async move {
    Server::builder()
      .add_service(chess_server::ChessServer::new(chess_grpc_server))
      .serve(addr)
      .await.unwrap();
  });
}


#[tokio::test]
async fn online_game_new()
{
  run_test_server("0.0.0.0:3001").await;

  let online_game = CreateGame {
    player: Some(game_chess_client::Player {
      player_id: "01".to_string(),
      player_name: "John Doe".to_string()
    })
  };

  let mut chess_client = chess_client::ChessClient::connect("http://localhost:3001").await.unwrap();
  let resp = chess_client.push_game_create(online_game).await;
  let game_id = resp.unwrap().get_ref().game_id.to_string();

  // `push_game_create` returns a Game ID
  // `game_id` is a random string of length 6
  assert_eq!(game_id.len(), 6);
}

#[tokio::test]
async fn online_game_join()
{
  run_test_server("0.0.0.0:3002").await;

  let online_game = CreateGame {
    player: Some(game_chess_client::Player {
      player_id: "01".to_string(),
      player_name: "John Doe".to_string()
    })
  };

  let mut chess_client = chess_client::ChessClient::connect("http://localhost:3002").await.unwrap();
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
}
