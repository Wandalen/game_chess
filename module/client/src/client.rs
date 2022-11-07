use multiplayer::generated::chess::chess_client::ChessClient;
use multiplayer::generated::chess::{ CreateGame, GameId, Board, AcceptGame, GameMove, GamePlayer, Msg };
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

  /// Create new game.
  pub async fn push_game_create( &mut self, game : CreateGame ) -> Result< tonic::Response< GameId >, tonic::Status >
  {
    self._grpc_client.push_game_create( game ).await
  }

  /// Get board state.
  pub async fn pull_board_state( &mut self, game_id : GameId ) -> Result< tonic::Response< Board >, tonic::Status >
  {
    self._grpc_client.pull_board_state( game_id ).await
  }
  
  /// Accept game  
  pub async fn push_game_accept( &mut self, game_accept : AcceptGame ) -> Result< tonic::Response< GameId >, tonic::Status >
  {
    self._grpc_client.push_game_accept( game_accept ).await
  } 

  /// Game move
  pub async fn push_move( &mut self, game_move : GameMove ) -> Result< tonic::Response< Board >, tonic::Status >
  {
    self._grpc_client.push_move( game_move ).await
  } 

  /// Game player  
  pub async fn push_game_gg( &mut self, game_player : GamePlayer ) -> Result< tonic::Response< () >, tonic::Status >
  {
    self._grpc_client.push_game_gg( game_player ).await
  } 

  /// Push Msg
  pub async fn push_msg( &mut self, msg : Msg ) -> Result< tonic::Response< () >, tonic::Status >
  {
    self._grpc_client.push_msg( msg ).await
  }  
}

