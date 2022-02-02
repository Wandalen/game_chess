use std::sync::{Arc, Mutex};

use tonic::{Request, Response, Status};

use crate::generated::chess::chess_server::Chess;
use crate::store::GameStore;
use crate::generated::chess::{ChessMove, OperationResult, GameRequest, Game, Games, CreateGame, PlayerRequest};
use crate::store::memory::MemoryStore;

pub struct ChessRpcServer {
  store: Arc<Mutex<Box<dyn GameStore + Send + Sync>>>,
}

impl ChessRpcServer {
  pub fn init() -> Self {
    Self {
      store: Arc::new(Mutex::new(Box::new(MemoryStore::new()))),
    }
  }
}

#[tonic::async_trait]
impl Chess for ChessRpcServer {
  async fn make_move(&self, request: Request<ChessMove>) -> Result<Response<OperationResult>, Status> {
    todo!()
  }

  async fn game(&self, request: Request<GameRequest>) -> Result<Response<Game>, Status> {
    todo!()
  }

  async fn games(&self, request: Request<()>) -> Result<Response<Games>, Status> {
    todo!()
  }

  async fn create_game(&self, request: Request<CreateGame>) -> Result<Response<Game>, Status> {
    todo!()
  }

  async fn join_game(&self, request: Request<PlayerRequest>) -> Result<Response<Game>, Status> {
    todo!()
  }

  async fn surrender(&self, request: Request<PlayerRequest>) -> Result<Response<Game>, Status> {
    todo!()
  }

  async fn call_draw(&self, request: Request<PlayerRequest>) -> Result<Response<Game>, Status> {
    todo!()
  }

  async fn leave(&self, request: Request<PlayerRequest>) -> Result<Response<Game>, Status> {
    todo!()
  }
}
