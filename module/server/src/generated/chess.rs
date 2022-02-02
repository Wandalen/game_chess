#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Player {
  #[prost(string, tag = "1")]
  pub player_id: ::prost::alloc::string::String,
  #[prost(string, tag = "2")]
  pub player_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameState {
  #[prost(enumeration = "game_state::State", tag = "1")]
  pub state: i32,
  #[prost(string, tag = "2")]
  pub player_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `GameState`.
pub mod game_state {
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
  #[repr(i32)]
  pub enum State {
    Created = 0,
    Running = 1,
    ProposeDraw = 2,
    Draw = 3,
    Surrender = 4,
    Win = 5,
    Leave = 6,
  }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Game {
  #[prost(string, tag = "1")]
  pub game_id: ::prost::alloc::string::String,
  #[prost(message, repeated, tag = "2")]
  pub players: ::prost::alloc::vec::Vec<Player>,
  #[prost(string, optional, tag = "4")]
  pub winner_id: ::core::option::Option<::prost::alloc::string::String>,
  #[prost(message, optional, tag = "5")]
  pub state: ::core::option::Option<GameState>,
  #[prost(map = "string, enumeration(Color)", tag = "6")]
  pub player_colors: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
  #[prost(string, repeated, tag = "7")]
  pub history: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Games {
  #[prost(message, repeated, tag = "1")]
  pub games: ::prost::alloc::vec::Vec<Game>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChessMove {
  #[prost(string, tag = "1")]
  pub game_id: ::prost::alloc::string::String,
  #[prost(string, tag = "2")]
  pub player_id: ::prost::alloc::string::String,
  #[prost(string, tag = "3")]
  pub r#move: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationResult {
  #[prost(bool, tag = "1")]
  pub success: bool,
  #[prost(string, optional, tag = "2")]
  pub msg: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameRequest {
  #[prost(string, tag = "1")]
  pub game_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGame {
  #[prost(message, optional, tag = "1")]
  pub player_id: ::core::option::Option<Player>,
  #[prost(enumeration = "Color", optional, tag = "2")]
  pub color: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerRequest {
  #[prost(message, optional, tag = "1")]
  pub player_id: ::core::option::Option<Player>,
  #[prost(string, tag = "2")]
  pub game_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Surrender {
  #[prost(string, tag = "1")]
  pub player_id: ::prost::alloc::string::String,
  #[prost(string, tag = "2")]
  pub game_id: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Color {
  White = 0,
  Black = 1,
}
#[doc = r" Generated client implementations."]
pub mod chess_client {
  #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
  use tonic::codegen::*;
  #[derive(Debug, Clone)]
  pub struct ChessClient<T> {
    inner: tonic::client::Grpc<T>,
  }
  impl ChessClient<tonic::transport::Channel> {
    #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
    pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
    where
      D: std::convert::TryInto<tonic::transport::Endpoint>,
      D::Error: Into<StdError>,
    {
      let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
      Ok(Self::new(conn))
    }
  }
  impl<T> ChessClient<T>
  where
    T: tonic::client::GrpcService<tonic::body::BoxBody>,
    T::ResponseBody: Body + Send + 'static,
    T::Error: Into<StdError>,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
  {
    pub fn new(inner: T) -> Self {
      let inner = tonic::client::Grpc::new(inner);
      Self { inner }
    }
    pub fn with_interceptor<F>(inner: T, interceptor: F) -> ChessClient<InterceptedService<T, F>>
    where
      F: tonic::service::Interceptor,
      T: tonic::codegen::Service<
        http::Request<tonic::body::BoxBody>,
        Response = http::Response<<T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody>,
      >,
      <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error: Into<StdError> + Send + Sync,
    {
      ChessClient::new(InterceptedService::new(inner, interceptor))
    }
    #[doc = r" Compress requests with `gzip`."]
    #[doc = r""]
    #[doc = r" This requires the server to support it otherwise it might respond with an"]
    #[doc = r" error."]
    pub fn send_gzip(mut self) -> Self {
      self.inner = self.inner.send_gzip();
      self
    }
    #[doc = r" Enable decompressing responses with `gzip`."]
    pub fn accept_gzip(mut self) -> Self {
      self.inner = self.inner.accept_gzip();
      self
    }
    pub async fn make_move(
      &mut self,
      request: impl tonic::IntoRequest<super::ChessMove>,
    ) -> Result<tonic::Response<super::OperationResult>, tonic::Status> {
      self
        .inner
        .ready()
        .await
        .map_err(|e| tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into())))?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/chess.Chess/make_move");
      self.inner.unary(request.into_request(), path, codec).await
    }
    pub async fn game(
      &mut self,
      request: impl tonic::IntoRequest<super::GameRequest>,
    ) -> Result<tonic::Response<super::Game>, tonic::Status> {
      self
        .inner
        .ready()
        .await
        .map_err(|e| tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into())))?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/chess.Chess/game");
      self.inner.unary(request.into_request(), path, codec).await
    }
    pub async fn games(&mut self, request: impl tonic::IntoRequest<()>) -> Result<tonic::Response<super::Games>, tonic::Status> {
      self
        .inner
        .ready()
        .await
        .map_err(|e| tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into())))?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/chess.Chess/games");
      self.inner.unary(request.into_request(), path, codec).await
    }
    pub async fn create_game(
      &mut self,
      request: impl tonic::IntoRequest<super::CreateGame>,
    ) -> Result<tonic::Response<super::Game>, tonic::Status> {
      self
        .inner
        .ready()
        .await
        .map_err(|e| tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into())))?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/chess.Chess/create_game");
      self.inner.unary(request.into_request(), path, codec).await
    }
    pub async fn join_game(
      &mut self,
      request: impl tonic::IntoRequest<super::PlayerRequest>,
    ) -> Result<tonic::Response<super::Game>, tonic::Status> {
      self
        .inner
        .ready()
        .await
        .map_err(|e| tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into())))?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/chess.Chess/join_game");
      self.inner.unary(request.into_request(), path, codec).await
    }
    pub async fn surrender(
      &mut self,
      request: impl tonic::IntoRequest<super::PlayerRequest>,
    ) -> Result<tonic::Response<super::Game>, tonic::Status> {
      self
        .inner
        .ready()
        .await
        .map_err(|e| tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into())))?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/chess.Chess/surrender");
      self.inner.unary(request.into_request(), path, codec).await
    }
    pub async fn call_draw(
      &mut self,
      request: impl tonic::IntoRequest<super::PlayerRequest>,
    ) -> Result<tonic::Response<super::Game>, tonic::Status> {
      self
        .inner
        .ready()
        .await
        .map_err(|e| tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into())))?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/chess.Chess/call_draw");
      self.inner.unary(request.into_request(), path, codec).await
    }
    pub async fn confirm_draw(
      &mut self,
      request: impl tonic::IntoRequest<super::PlayerRequest>,
    ) -> Result<tonic::Response<super::Game>, tonic::Status> {
      self
        .inner
        .ready()
        .await
        .map_err(|e| tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into())))?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/chess.Chess/confirm_draw");
      self.inner.unary(request.into_request(), path, codec).await
    }
    pub async fn leave(
      &mut self,
      request: impl tonic::IntoRequest<super::PlayerRequest>,
    ) -> Result<tonic::Response<super::Game>, tonic::Status> {
      self
        .inner
        .ready()
        .await
        .map_err(|e| tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into())))?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/chess.Chess/leave");
      self.inner.unary(request.into_request(), path, codec).await
    }
  }
}
#[doc = r" Generated server implementations."]
pub mod chess_server {
  #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
  use tonic::codegen::*;
  #[doc = "Generated trait containing gRPC methods that should be implemented for use with ChessServer."]
  #[async_trait]
  pub trait Chess: Send + Sync + 'static {
    async fn make_move(
      &self,
      request: tonic::Request<super::ChessMove>,
    ) -> Result<tonic::Response<super::OperationResult>, tonic::Status>;
    async fn game(&self, request: tonic::Request<super::GameRequest>) -> Result<tonic::Response<super::Game>, tonic::Status>;
    async fn games(&self, request: tonic::Request<()>) -> Result<tonic::Response<super::Games>, tonic::Status>;
    async fn create_game(
      &self,
      request: tonic::Request<super::CreateGame>,
    ) -> Result<tonic::Response<super::Game>, tonic::Status>;
    async fn join_game(
      &self,
      request: tonic::Request<super::PlayerRequest>,
    ) -> Result<tonic::Response<super::Game>, tonic::Status>;
    async fn surrender(
      &self,
      request: tonic::Request<super::PlayerRequest>,
    ) -> Result<tonic::Response<super::Game>, tonic::Status>;
    async fn call_draw(
      &self,
      request: tonic::Request<super::PlayerRequest>,
    ) -> Result<tonic::Response<super::Game>, tonic::Status>;
    async fn confirm_draw(
      &self,
      request: tonic::Request<super::PlayerRequest>,
    ) -> Result<tonic::Response<super::Game>, tonic::Status>;
    async fn leave(&self, request: tonic::Request<super::PlayerRequest>) -> Result<tonic::Response<super::Game>, tonic::Status>;
  }
  #[derive(Debug)]
  pub struct ChessServer<T: Chess> {
    inner: _Inner<T>,
    accept_compression_encodings: (),
    send_compression_encodings: (),
  }
  struct _Inner<T>(Arc<T>);
  impl<T: Chess> ChessServer<T> {
    pub fn new(inner: T) -> Self {
      let inner = Arc::new(inner);
      let inner = _Inner(inner);
      Self {
        inner,
        accept_compression_encodings: Default::default(),
        send_compression_encodings: Default::default(),
      }
    }
    pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
    where
      F: tonic::service::Interceptor,
    {
      InterceptedService::new(Self::new(inner), interceptor)
    }
  }
  impl<T, B> tonic::codegen::Service<http::Request<B>> for ChessServer<T>
  where
    T: Chess,
    B: Body + Send + 'static,
    B::Error: Into<StdError> + Send + 'static,
  {
    type Response = http::Response<tonic::body::BoxBody>;
    type Error = Never;
    type Future = BoxFuture<Self::Response, Self::Error>;
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
      Poll::Ready(Ok(()))
    }
    fn call(&mut self, req: http::Request<B>) -> Self::Future {
      let inner = self.inner.clone();
      match req.uri().path() {
        "/chess.Chess/make_move" => {
          #[allow(non_camel_case_types)]
          struct make_moveSvc<T: Chess>(pub Arc<T>);
          impl<T: Chess> tonic::server::UnaryService<super::ChessMove> for make_moveSvc<T> {
            type Response = super::OperationResult;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::ChessMove>) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).make_move(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = make_moveSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc =
              tonic::server::Grpc::new(codec).apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/chess.Chess/game" => {
          #[allow(non_camel_case_types)]
          struct gameSvc<T: Chess>(pub Arc<T>);
          impl<T: Chess> tonic::server::UnaryService<super::GameRequest> for gameSvc<T> {
            type Response = super::Game;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::GameRequest>) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).game(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = gameSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc =
              tonic::server::Grpc::new(codec).apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/chess.Chess/games" => {
          #[allow(non_camel_case_types)]
          struct gamesSvc<T: Chess>(pub Arc<T>);
          impl<T: Chess> tonic::server::UnaryService<()> for gamesSvc<T> {
            type Response = super::Games;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).games(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = gamesSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc =
              tonic::server::Grpc::new(codec).apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/chess.Chess/create_game" => {
          #[allow(non_camel_case_types)]
          struct create_gameSvc<T: Chess>(pub Arc<T>);
          impl<T: Chess> tonic::server::UnaryService<super::CreateGame> for create_gameSvc<T> {
            type Response = super::Game;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::CreateGame>) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).create_game(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = create_gameSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc =
              tonic::server::Grpc::new(codec).apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/chess.Chess/join_game" => {
          #[allow(non_camel_case_types)]
          struct join_gameSvc<T: Chess>(pub Arc<T>);
          impl<T: Chess> tonic::server::UnaryService<super::PlayerRequest> for join_gameSvc<T> {
            type Response = super::Game;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::PlayerRequest>) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).join_game(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = join_gameSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc =
              tonic::server::Grpc::new(codec).apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/chess.Chess/surrender" => {
          #[allow(non_camel_case_types)]
          struct surrenderSvc<T: Chess>(pub Arc<T>);
          impl<T: Chess> tonic::server::UnaryService<super::PlayerRequest> for surrenderSvc<T> {
            type Response = super::Game;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::PlayerRequest>) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).surrender(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = surrenderSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc =
              tonic::server::Grpc::new(codec).apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/chess.Chess/call_draw" => {
          #[allow(non_camel_case_types)]
          struct call_drawSvc<T: Chess>(pub Arc<T>);
          impl<T: Chess> tonic::server::UnaryService<super::PlayerRequest> for call_drawSvc<T> {
            type Response = super::Game;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::PlayerRequest>) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).call_draw(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = call_drawSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc =
              tonic::server::Grpc::new(codec).apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/chess.Chess/confirm_draw" => {
          #[allow(non_camel_case_types)]
          struct confirm_drawSvc<T: Chess>(pub Arc<T>);
          impl<T: Chess> tonic::server::UnaryService<super::PlayerRequest> for confirm_drawSvc<T> {
            type Response = super::Game;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::PlayerRequest>) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).confirm_draw(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = confirm_drawSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc =
              tonic::server::Grpc::new(codec).apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/chess.Chess/leave" => {
          #[allow(non_camel_case_types)]
          struct leaveSvc<T: Chess>(pub Arc<T>);
          impl<T: Chess> tonic::server::UnaryService<super::PlayerRequest> for leaveSvc<T> {
            type Response = super::Game;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::PlayerRequest>) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).leave(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = leaveSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc =
              tonic::server::Grpc::new(codec).apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
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
  impl<T: Chess> Clone for ChessServer<T> {
    fn clone(&self) -> Self {
      let inner = self.inner.clone();
      Self {
        inner,
        accept_compression_encodings: self.accept_compression_encodings,
        send_compression_encodings: self.send_compression_encodings,
      }
    }
  }
  impl<T: Chess> Clone for _Inner<T> {
    fn clone(&self) -> Self {
      Self(self.0.clone())
    }
  }
  impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{:?}", self.0)
    }
  }
  impl<T: Chess> tonic::transport::NamedService for ChessServer<T> {
    const NAME: &'static str = "chess.Chess";
  }
}
