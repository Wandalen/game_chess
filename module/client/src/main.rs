#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

//!
//! Chess game client implemented for educational purpose.
//!

use multiplayer::generated::chess::chess_client::ChessClient;

#[tokio::main]
async fn main()
{
  println!("Simple rpc client");

  let /* mut */ _chess_client = ChessClient::connect("http://[::1]:50051").await.unwrap();
}

