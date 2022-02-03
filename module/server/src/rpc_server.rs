use std::sync::{Arc, Mutex};

use tonic::{Request, Response, Status};

use crate::generated::chess::chess_server::Chess;
use crate::store::GameStore;
use crate::generated::chess::{Game, Games, CreateGame, GameId, AcceptGame, GameMove, GamePlayer, Msg, Msgs};
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
  async fn push_game_create(&self, request: Request<CreateGame>) -> Result<Response<GameId>, Status> {
    todo!()
  }

  async fn push_game_accept(&self, request: Request<AcceptGame>) -> Result<Response<GameId>, Status> {
    todo!()
  }

  async fn push_move(&self, request: Request<GameMove>) -> Result<Response<GameId>, Status> {
    todo!()
  }

  async fn read_board_state(&self, request: Request<GameId>) -> Result<Response<Game>, Status> {
    todo!()
  }

  async fn read_game_state(&self, request: Request<GameId>) -> Result<Response<Game>, Status> {
    todo!()
  }

  async fn read_games_list(&self, request: Request<()>) -> Result<Response<Games>, Status> {
    todo!()
  }

  async fn push_game_gg(&self, request: Request<GamePlayer>) -> Result<Response<Game>, Status> {
    todo!()
  }

  async fn push_mgs(&self, request: Request<Msg>) -> Result<Response<()>, Status> {
    todo!()
  }

  async fn read_msgs(&self, request: Request<GameId>) -> Result<Response<Msgs>, Status> {
    todo!()
  }
}
