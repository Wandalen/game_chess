#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Chess game server implemented for educational purpose.
//!

#[ allow( unused_imports ) ]
use game_chess_server::store::memory::MemoryStore;
use game_chess_server::rpc_server::ChessRpcServer;
use tonic::transport::Server;
use multiplayer::generated::chess::chess_server::ChessServer;

///
/// Main.
///

#[ tokio::main ]
async fn main() -> Result< (), Box< dyn std::error::Error > >
{
  let chess_grpc_server = ChessRpcServer::init();

  let addr = "0.0.0.0:1313".parse()?;
  println!( "Server listening on {}", addr );

  let chess_server = tonic_web::enable( ChessServer::new( chess_grpc_server ) );
  Server::builder()
  .accept_http1( true )
  .add_service( chess_server )
  .serve( addr )
  .await?;

  Ok( () )
}
