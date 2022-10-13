use multiplayer::generated::chess::chess_client::ChessClient;
#[ cfg( not( target_arch = "wasm32" ) ) ]
use lazy_static::lazy_static;

#[ cfg( not( target_arch = "wasm32" ) ) ]
lazy_static!
{
  static ref TOKIO_RUNTIME : tokio::runtime::Runtime =
  {
    std::thread::spawn( move || TOKIO_RUNTIME.block_on( std::future::pending::< () >() ) );
    tokio::runtime::Builder::new_current_thread()
    .enable_io()
    .build()
    .expect( "cannot start tokio runtime" )
  };
}

///
/// Client for the chess multiplayer
///

#[ derive( Debug ) ]
pub struct Client
{
  #[ cfg( not( target_arch = "wasm32" ) ) ]
  _grpc_client : ChessClient< tonic::transport::Channel >,
  #[ cfg( target_arch = "wasm32" ) ]
  _grpc_client : ChessClient< tonic_web_wasm_client::Client >,
}

///
/// Client errors
///

#[ derive( thiserror::Error, Debug ) ]
pub enum ClientError
{
  /// Grpc error
  #[ cfg( not(target_arch = "wasm32" ) ) ]
  #[ error( "grpc error" ) ]
  Grpc( #[ from ] tonic::transport::Error ),
}

impl Client
{
  /// Connect to 'destination'
  pub async fn connect( destination : impl Into< String > ) -> Result< Self, ClientError >
  {
    // tonic's transport feature needs tokio runtime.
    #[ cfg( not( target_arch = "wasm32" ) ) ]
    let _guard = TOKIO_RUNTIME.enter();

    // TODO: Add connection check in wasm
    #[ cfg( target_arch = "wasm32" ) ]
    let grpc_client = ChessClient::new( tonic_web_wasm_client::Client::new( destination.into() ) );

    #[ cfg( not( target_arch = "wasm32" ) ) ]
    let grpc_client = ChessClient::new( tonic::transport::Endpoint::new( destination.into() )?.connect().await? );

    Ok( Client { _grpc_client : grpc_client } )
  }
}

