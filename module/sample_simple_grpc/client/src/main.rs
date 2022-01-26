
use client::generated::chess_example::{Position, GameMoveRequest, MoveResultResponse};
use client::generated::chess_example::chess_state_client::ChessStateClient;

#[tokio::main]
async fn main() {
  println!("Simple grpc client");

  let mut chess_client = ChessStateClient::connect("http://[::1]:50051").await.unwrap();
  let move_result = chess_client.make_move(GameMoveRequest {
    figure_id: 2,
    to: Some(Position {
      row: 3,
      column: 2,
    }),
  }).await.unwrap();

  println!("{:?}", move_result.metadata());
  println!("{:?}", move_result.extensions());
  println!("{}", move_result.get_ref());
}
