use async_compat::Compat;
use multiplayer::generated::chess::chess_client::ChessClient;

///
/// Client for the chess multiplayer
///

#[ derive( Debug ) ]
pub struct Client
{
  _grpc_client : ChessClient< tonic::transport::Channel >,
}

///
/// Client errors
///

#[ derive( thiserror::Error, Debug ) ]
pub enum ClientError
{
  /// Grpc error
  #[ error( "grpc error" ) ]
  Grpc( #[ from ] tonic::transport::Error ),
}

impl Client
{
  /// Connect to 'destination'
  pub async fn connect( destination : impl Into< String > ) -> Result< Self, ClientError >
  {
    Compat::new
    (
      async
      {
        let grpc_client = ChessClient::connect( destination.into() ).await?;
        Ok( Client { _grpc_client : grpc_client } )
      }
    )
    .await
  }
}

