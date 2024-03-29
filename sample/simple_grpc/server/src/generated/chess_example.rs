#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameMoveRequest {
  #[prost(int32, tag = "1")]
  pub figure_id: i32,
  #[prost(message, optional, tag = "2")]
  pub to: ::core::option::Option<Position>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveResultResponse {
  #[prost(int32, tag = "1")]
  pub status: i32,
  #[prost(string, tag = "2")]
  pub message: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
  #[prost(int32, tag = "1")]
  pub row: i32,
  #[prost(int32, tag = "2")]
  pub column: i32,
}
#[doc = r" Generated client implementations."]
pub mod chess_state_client {
  #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
  use tonic::codegen::*;
  #[derive(Debug, Clone)]
  pub struct ChessStateClient<T> {
    inner: tonic::client::Grpc<T>,
  }
  impl ChessStateClient<tonic::transport::Channel> {
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
  impl<T> ChessStateClient<T>
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
    pub fn with_interceptor<F>(inner: T, interceptor: F) -> ChessStateClient<InterceptedService<T, F>>
    where
      F: tonic::service::Interceptor,
      T: tonic::codegen::Service<
        http::Request<tonic::body::BoxBody>,
        Response = http::Response<<T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody>,
      >,
      <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error: Into<StdError> + Send + Sync,
    {
      ChessStateClient::new(InterceptedService::new(inner, interceptor))
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
      request: impl tonic::IntoRequest<super::GameMoveRequest>,
    ) -> Result<tonic::Response<super::MoveResultResponse>, tonic::Status> {
      self
        .inner
        .ready()
        .await
        .map_err(|e| tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into())))?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/chess_example.ChessState/make_move");
      self.inner.unary(request.into_request(), path, codec).await
    }
  }
}
#[doc = r" Generated server implementations."]
pub mod chess_state_server {
  #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
  use tonic::codegen::*;
  #[doc = "Generated trait containing gRPC methods that should be implemented for use with ChessStateServer."]
  #[async_trait]
  pub trait ChessState: Send + Sync + 'static {
    async fn make_move(
      &self,
      request: tonic::Request<super::GameMoveRequest>,
    ) -> Result<tonic::Response<super::MoveResultResponse>, tonic::Status>;
  }
  #[derive(Debug)]
  pub struct ChessStateServer<T: ChessState> {
    inner: _Inner<T>,
    accept_compression_encodings: (),
    send_compression_encodings: (),
  }
  struct _Inner<T>(Arc<T>);
  impl<T: ChessState> ChessStateServer<T> {
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
  impl<T, B> tonic::codegen::Service<http::Request<B>> for ChessStateServer<T>
  where
    T: ChessState,
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
        "/chess_example.ChessState/make_move" => {
          #[allow(non_camel_case_types)]
          struct make_moveSvc<T: ChessState>(pub Arc<T>);
          impl<T: ChessState> tonic::server::UnaryService<super::GameMoveRequest> for make_moveSvc<T> {
            type Response = super::MoveResultResponse;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::GameMoveRequest>) -> Self::Future {
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
  impl<T: ChessState> Clone for ChessStateServer<T> {
    fn clone(&self) -> Self {
      let inner = self.inner.clone();
      Self {
        inner,
        accept_compression_encodings: self.accept_compression_encodings,
        send_compression_encodings: self.send_compression_encodings,
      }
    }
  }
  impl<T: ChessState> Clone for _Inner<T> {
    fn clone(&self) -> Self {
      Self(self.0.clone())
    }
  }
  impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{:?}", self.0)
    }
  }
  impl<T: ChessState> tonic::transport::NamedService for ChessStateServer<T> {
    const NAME: &'static str = "chess_example.ChessState";
  }
}
