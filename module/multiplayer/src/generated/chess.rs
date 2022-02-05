#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameUpdate
{
  #[prost(oneof = "game_update::GameUpdate", tags = "1, 2")]
  pub game_update : ::core::option::Option<game_update::GameUpdate>,
}
/// Nested message and enum types in `GameUpdate`.
pub mod game_update
{
  #[derive(Clone, PartialEq, ::prost::Oneof)]
  pub enum GameUpdate
  {
    #[prost(message, tag = "1")]
    GameMove(super::GameMove),
    /// TODO: refactor chat messages to use streams when they are ready.
    #[prost(message, tag = "2")]
    GameEnd(super::GameEnd),
  }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Blank {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Player
{
  #[prost(string, tag = "1")]
  pub player_id : ::prost::alloc::string::String,
  #[prost(string, tag = "2")]
  pub player_name : ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GamePlayer
{
  #[prost(string, tag = "1")]
  pub player_id : ::prost::alloc::string::String,
  #[prost(string, tag = "2")]
  pub game_id : ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameInfo
{
  #[prost(string, tag = "1")]
  pub game_id : ::prost::alloc::string::String,
  #[prost(message, repeated, tag = "2")]
  pub players : ::prost::alloc::vec::Vec<Player>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameState
{
  #[prost(string, tag = "1")]
  pub game_id : ::prost::alloc::string::String,
  #[prost(message, repeated, tag = "2")]
  pub players : ::prost::alloc::vec::Vec<Player>,
  #[prost(message, optional, tag = "3")]
  pub game_state : ::core::option::Option<Blank>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Board
{
  #[prost(string, tag = "1")]
  pub game_id : ::prost::alloc::string::String,
  #[prost(message, optional, tag = "2")]
  pub board_state : ::core::option::Option<Blank>,
}
/// ?
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Games {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameMove
{
  #[prost(string, tag = "1")]
  pub game_id : ::prost::alloc::string::String,
  #[prost(string, tag = "2")]
  pub player_id : ::prost::alloc::string::String,
  #[prost(message, optional, tag = "3")]
  pub r#move : ::core::option::Option<Blank>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameEnd
{
  /// Can be empty if it's a draw.
  #[prost(string, tag = "1")]
  pub winner_id : ::prost::alloc::string::String,
  /// Draw, Player won, Surrender.
  #[prost(string, tag = "2")]
  pub reason : ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceptGame
{
  #[prost(string, tag = "1")]
  pub game_id : ::prost::alloc::string::String,
  #[prost(message, optional, tag = "2")]
  pub player_id : ::core::option::Option<Player>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGame
{
  #[prost(message, optional, tag = "1")]
  pub player : ::core::option::Option<Player>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameId
{
  #[prost(string, tag = "1")]
  pub game_id : ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Msg
{
  #[prost(message, optional, tag = "1")]
  pub player : ::core::option::Option<GamePlayer>,
  #[prost(string, tag = "2")]
  pub text : ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Msgs
{
  #[prost(message, repeated, tag = "1")]
  pub messages : ::prost::alloc::vec::Vec<Msg>,
}
#[doc = r" Generated client implementations."]
pub mod chess_client
{
  #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
  use tonic::codegen::*;
  #[derive(Debug, Clone)]
  pub struct ChessClient<T>
  {
    inner : tonic::client::Grpc<T>,
  }
  impl ChessClient<tonic::transport::Channel>
  {
    #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
    pub async fn connect<D>(dst : D) -> Result<Self, tonic::transport::Error>
    where
      D : std::convert::TryInto<tonic::transport::Endpoint>,
      D::Error : Into<StdError>,
    {
      let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
      Ok(Self::new(conn))
    }
  }
  impl<T> ChessClient<T>
  where
    T : tonic::client::GrpcService<tonic::body::BoxBody>,
    T::ResponseBody : Body + Send + 'static,
    T::Error : Into<StdError>,
    <T::ResponseBody as Body>::Error : Into<StdError> + Send,
  {
    pub fn new(inner : T) -> Self
    {
      let inner = tonic::client::Grpc::new(inner);
      Self { inner }
    }

    pub fn with_interceptor<F>(inner : T, interceptor : F) -> ChessClient<InterceptedService<T, F>>
    where
      F : tonic::service::Interceptor,
      T : tonic::codegen::Service<
        http::Request<tonic::body::BoxBody>,
        Response = http::Response<<T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody>,
      >,
      <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error : Into<StdError> + Send + Sync,
    {
      ChessClient::new(InterceptedService::new(inner, interceptor))
    }

    #[doc = r" Compress requests with `gzip`."]
    #[doc = r""]
    #[doc = r" This requires the server to support it otherwise it might respond with an"]
    #[doc = r" error."]
    pub fn send_gzip(mut self) -> Self
    {
      self.inner = self.inner.send_gzip();
      self
    }

    #[doc = r" Enable decompressing responses with `gzip`."]
    pub fn accept_gzip(mut self) -> Self
    {
      self.inner = self.inner.accept_gzip();
      self
    }

    pub async fn push_game_create(
      &mut self,
      request : impl tonic::IntoRequest<super::CreateGame>,
    ) -> Result<tonic::Response<super::GameId>, tonic::Status>
    {
      self
        .inner
        .ready()
        .await
        .map_err(|e| tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into())))?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/chess.Chess/push_game_create");
      self.inner.unary(request.into_request(), path, codec).await
    }

    pub async fn push_game_accept(
      &mut self,
      request : impl tonic::IntoRequest<super::AcceptGame>,
    ) -> Result<tonic::Response<super::GameId>, tonic::Status>
    {
      self
        .inner
        .ready()
        .await
        .map_err(|e| tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into())))?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/chess.Chess/push_game_accept");
      self.inner.unary(request.into_request(), path, codec).await
    }

    pub async fn push_move(
      &mut self,
      request : impl tonic::IntoRequest<super::GameMove>,
    ) -> Result<tonic::Response<super::GameId>, tonic::Status>
    {
      self
        .inner
        .ready()
        .await
        .map_err(|e| tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into())))?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/chess.Chess/push_move");
      self.inner.unary(request.into_request(), path, codec).await
    }

    pub async fn pull_board_state(
      &mut self,
      request : impl tonic::IntoRequest<super::GameId>,
    ) -> Result<tonic::Response<super::Board>, tonic::Status>
    {
      self
        .inner
        .ready()
        .await
        .map_err(|e| tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into())))?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/chess.Chess/pull_board_state");
      self.inner.unary(request.into_request(), path, codec).await
    }

    pub async fn pull_game_state(
      &mut self,
      request : impl tonic::IntoRequest<super::GameId>,
    ) -> Result<tonic::Response<super::GameState>, tonic::Status>
    {
      self
        .inner
        .ready()
        .await
        .map_err(|e| tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into())))?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/chess.Chess/pull_game_state");
      self.inner.unary(request.into_request(), path, codec).await
    }

    pub async fn pull_games_list(
      &mut self,
      request : impl tonic::IntoRequest<()>,
    ) -> Result<tonic::Response<super::Games>, tonic::Status>
    {
      self
        .inner
        .ready()
        .await
        .map_err(|e| tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into())))?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/chess.Chess/pull_games_list");
      self.inner.unary(request.into_request(), path, codec).await
    }

    pub async fn push_game_gg(
      &mut self,
      request : impl tonic::IntoRequest<super::GamePlayer>,
    ) -> Result<tonic::Response<()>, tonic::Status>
    {
      self
        .inner
        .ready()
        .await
        .map_err(|e| tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into())))?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/chess.Chess/push_game_gg");
      self.inner.unary(request.into_request(), path, codec).await
    }

    pub async fn push_mgs(&mut self, request : impl tonic::IntoRequest<super::Msg>)
      -> Result<tonic::Response<()>, tonic::Status>
    {
      self
        .inner
        .ready()
        .await
        .map_err(|e| tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into())))?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/chess.Chess/push_mgs");
      self.inner.unary(request.into_request(), path, codec).await
    }

    pub async fn pull_game_updates(
      &mut self,
      request : impl tonic::IntoRequest<super::GameId>,
    ) -> Result<tonic::Response<tonic::codec::Streaming<super::GameUpdate>>, tonic::Status>
    {
      self
        .inner
        .ready()
        .await
        .map_err(|e| tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into())))?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/chess.Chess/pull_game_updates");
      self.inner.server_streaming(request.into_request(), path, codec).await
    }
  }
}
#[doc = r" Generated server implementations."]
pub mod chess_server
{
  #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
  use tonic::codegen::*;
  #[doc = "Generated trait containing gRPC methods that should be implemented for use with ChessServer."]
  #[async_trait]
  pub trait Chess: Send + Sync + 'static
  {
    async fn push_game_create(
      &self,
      request : tonic::Request<super::CreateGame>,
    ) -> Result<tonic::Response<super::GameId>, tonic::Status>;
    async fn push_game_accept(
      &self,
      request : tonic::Request<super::AcceptGame>,
    ) -> Result<tonic::Response<super::GameId>, tonic::Status>;
    async fn push_move(&self, request : tonic::Request<super::GameMove>)
      -> Result<tonic::Response<super::GameId>, tonic::Status>;
    async fn pull_board_state(
      &self,
      request : tonic::Request<super::GameId>,
    ) -> Result<tonic::Response<super::Board>, tonic::Status>;
    async fn pull_game_state(
      &self,
      request : tonic::Request<super::GameId>,
    ) -> Result<tonic::Response<super::GameState>, tonic::Status>;
    async fn pull_games_list(&self, request : tonic::Request<()>) -> Result<tonic::Response<super::Games>, tonic::Status>;
    async fn push_game_gg(&self, request : tonic::Request<super::GamePlayer>) -> Result<tonic::Response<()>, tonic::Status>;
    async fn push_mgs(&self, request : tonic::Request<super::Msg>) -> Result<tonic::Response<()>, tonic::Status>;
    #[doc = "Server streaming response type for the pull_game_updates method."]
    type pull_game_updatesStream: futures_core::Stream<Item = Result<super::GameUpdate, tonic::Status>> + Send + 'static;
    async fn pull_game_updates(
      &self,
      request : tonic::Request<super::GameId>,
    ) -> Result<tonic::Response<Self::pull_game_updatesStream>, tonic::Status>;
  }
  #[derive(Debug)]
  pub struct ChessServer<T : Chess>
  {
    inner : _Inner<T>,
    accept_compression_encodings : (),
    send_compression_encodings : (),
  }
  struct _Inner<T>(Arc<T>);
  impl<T : Chess> ChessServer<T>
  {
    pub fn new(inner : T) -> Self
    {
      let inner = Arc::new(inner);
      let inner = _Inner(inner);
      Self {
        inner,
        accept_compression_encodings : Default::default(),
        send_compression_encodings : Default::default(),
      }
    }

    pub fn with_interceptor<F>(inner : T, interceptor : F) -> InterceptedService<Self, F>
    where
      F : tonic::service::Interceptor,
    {
      InterceptedService::new(Self::new(inner), interceptor)
    }
  }
  impl<T, B> tonic::codegen::Service<http::Request<B>> for ChessServer<T>
  where
    T : Chess,
    B : Body + Send + 'static,
    B::Error : Into<StdError> + Send + 'static,
  {
    type Error = Never;
    type Future = BoxFuture<Self::Response, Self::Error>;
    type Response = http::Response<tonic::body::BoxBody>;

    fn poll_ready(&mut self, _cx : &mut Context<'_>) -> Poll<Result<(), Self::Error>> { Poll::Ready(Ok(())) }

    fn call(&mut self, req : http::Request<B>) -> Self::Future
    {
      let inner = self.inner.clone();
      match req.uri().path()
      {
        "/chess.Chess/push_game_create" =>
        {
          #[allow(non_camel_case_types)]
          struct push_game_createSvc<T : Chess>(pub Arc<T>);
          impl<T : Chess> tonic::server::UnaryService<super::CreateGame> for push_game_createSvc<T>
          {
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            type Response = super::GameId;

            fn call(&mut self, request : tonic::Request<super::CreateGame>) -> Self::Future
            {
              let inner = self.0.clone();
              let fut = async move { (*inner).push_game_create(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = push_game_createSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc =
              tonic::server::Grpc::new(codec).apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/chess.Chess/push_game_accept" =>
        {
          #[allow(non_camel_case_types)]
          struct push_game_acceptSvc<T : Chess>(pub Arc<T>);
          impl<T : Chess> tonic::server::UnaryService<super::AcceptGame> for push_game_acceptSvc<T>
          {
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            type Response = super::GameId;

            fn call(&mut self, request : tonic::Request<super::AcceptGame>) -> Self::Future
            {
              let inner = self.0.clone();
              let fut = async move { (*inner).push_game_accept(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = push_game_acceptSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc =
              tonic::server::Grpc::new(codec).apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/chess.Chess/push_move" =>
        {
          #[allow(non_camel_case_types)]
          struct push_moveSvc<T : Chess>(pub Arc<T>);
          impl<T : Chess> tonic::server::UnaryService<super::GameMove> for push_moveSvc<T>
          {
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            type Response = super::GameId;

            fn call(&mut self, request : tonic::Request<super::GameMove>) -> Self::Future
            {
              let inner = self.0.clone();
              let fut = async move { (*inner).push_move(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = push_moveSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc =
              tonic::server::Grpc::new(codec).apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/chess.Chess/pull_board_state" =>
        {
          #[allow(non_camel_case_types)]
          struct pull_board_stateSvc<T : Chess>(pub Arc<T>);
          impl<T : Chess> tonic::server::UnaryService<super::GameId> for pull_board_stateSvc<T>
          {
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            type Response = super::Board;

            fn call(&mut self, request : tonic::Request<super::GameId>) -> Self::Future
            {
              let inner = self.0.clone();
              let fut = async move { (*inner).pull_board_state(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = pull_board_stateSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc =
              tonic::server::Grpc::new(codec).apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/chess.Chess/pull_game_state" =>
        {
          #[allow(non_camel_case_types)]
          struct pull_game_stateSvc<T : Chess>(pub Arc<T>);
          impl<T : Chess> tonic::server::UnaryService<super::GameId> for pull_game_stateSvc<T>
          {
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            type Response = super::GameState;

            fn call(&mut self, request : tonic::Request<super::GameId>) -> Self::Future
            {
              let inner = self.0.clone();
              let fut = async move { (*inner).pull_game_state(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = pull_game_stateSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc =
              tonic::server::Grpc::new(codec).apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/chess.Chess/pull_games_list" =>
        {
          #[allow(non_camel_case_types)]
          struct pull_games_listSvc<T : Chess>(pub Arc<T>);
          impl<T : Chess> tonic::server::UnaryService<()> for pull_games_listSvc<T>
          {
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            type Response = super::Games;

            fn call(&mut self, request : tonic::Request<()>) -> Self::Future
            {
              let inner = self.0.clone();
              let fut = async move { (*inner).pull_games_list(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = pull_games_listSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc =
              tonic::server::Grpc::new(codec).apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/chess.Chess/push_game_gg" =>
        {
          #[allow(non_camel_case_types)]
          struct push_game_ggSvc<T : Chess>(pub Arc<T>);
          impl<T : Chess> tonic::server::UnaryService<super::GamePlayer> for push_game_ggSvc<T>
          {
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            type Response = ();

            fn call(&mut self, request : tonic::Request<super::GamePlayer>) -> Self::Future
            {
              let inner = self.0.clone();
              let fut = async move { (*inner).push_game_gg(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = push_game_ggSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc =
              tonic::server::Grpc::new(codec).apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/chess.Chess/push_mgs" =>
        {
          #[allow(non_camel_case_types)]
          struct push_mgsSvc<T : Chess>(pub Arc<T>);
          impl<T : Chess> tonic::server::UnaryService<super::Msg> for push_mgsSvc<T>
          {
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            type Response = ();

            fn call(&mut self, request : tonic::Request<super::Msg>) -> Self::Future
            {
              let inner = self.0.clone();
              let fut = async move { (*inner).push_mgs(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = push_mgsSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc =
              tonic::server::Grpc::new(codec).apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/chess.Chess/pull_game_updates" =>
        {
          #[allow(non_camel_case_types)]
          struct pull_game_updatesSvc<T : Chess>(pub Arc<T>);
          impl<T : Chess> tonic::server::ServerStreamingService<super::GameId> for pull_game_updatesSvc<T>
          {
            type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
            type Response = super::GameUpdate;
            type ResponseStream = T::pull_game_updatesStream;

            fn call(&mut self, request : tonic::Request<super::GameId>) -> Self::Future
            {
              let inner = self.0.clone();
              let fut = async move { (*inner).pull_game_updates(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = pull_game_updatesSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc =
              tonic::server::Grpc::new(codec).apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.server_streaming(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        _ => Box::pin(async move {
          Ok(
            http::Response::builder()
              .status(200)
              .header("grpc-status", "12")
              .header("content-type", "application/grpc")
              .body(empty_body())
              .unwrap(),
          )
        }),
      }
    }
  }
  impl<T : Chess> Clone for ChessServer<T>
  {
    fn clone(&self) -> Self
    {
      let inner = self.inner.clone();
      Self {
        inner,
        accept_compression_encodings : self.accept_compression_encodings,
        send_compression_encodings : self.send_compression_encodings,
      }
    }
  }
  impl<T : Chess> Clone for _Inner<T>
  {
    fn clone(&self) -> Self { Self(self.0.clone()) }
  }
  impl<T : std::fmt::Debug> std::fmt::Debug for _Inner<T>
  {
    fn fmt(&self, f : &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", self.0) }
  }
  impl<T : Chess> tonic::transport::NamedService for ChessServer<T>
  {
    const NAME : &'static str = "chess.Chess";
  }
}
