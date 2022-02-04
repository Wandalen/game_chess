//!
//! RPC server that provides game API.
//!

use std::sync::{Arc, Mutex};

use tonic::{Request, Response, Status};

use multiplayer::generated::chess::chess_server::Chess;
use crate::store::GameStore;
use multiplayer::generated::chess::{Board, GameState, Games, CreateGame, GameId, AcceptGame, GameMove, GamePlayer, Msg, Msgs};
use crate::store::memory::MemoryStore;

///
/// Shared sever.
///

#[allow(missing_debug_implementations, dead_code)]
pub struct ChessRpcServer
{
  store : Arc<Mutex<Box<dyn GameStore + Send + Sync>>>,
}

impl ChessRpcServer
{
  ///
  /// Server constructor.
  ///
  pub fn init() -> Self
  {
    Self {
      store : Arc::new(Mutex::new(Box::new(MemoryStore::new()))),
    }
  }
}

#[tonic::async_trait]
impl Chess for ChessRpcServer
{
  ///
  /// Apply request to create new game.
  ///
  async fn push_game_create(&self, _request : Request<CreateGame>) -> Result<Response<GameId>, Status> { todo!() }

  ///
  /// Accept request to join game.
  ///
  async fn push_game_accept(&self, _request : Request<AcceptGame>) -> Result<Response<GameId>, Status> { todo!() }

  ///
  /// Apply move.
  ///
  async fn push_move(&self, _request : Request<GameMove>) -> Result<Response<GameId>, Status> { todo!() }

  ///
  /// Get info about current board state. There are only positions.
  ///
  async fn read_board_state(&self, _request : Request<GameId>) -> Result<Response<Board>, Status> { todo!() }

  ///
  /// Get info about current game state - positions and history.
  ///
  async fn read_game_state(&self, _request : Request<GameId>) -> Result<Response<GameState>, Status> { todo!() }

  ///
  /// Get list of games.
  ///
  async fn read_games_list(&self, _request : Request<()>) -> Result<Response<Games>, Status> { todo!() }

  ///
  /// Send request to forfeit.
  ///
  async fn push_game_gg(&self, _request : Request<GamePlayer>) -> Result<Response<()>, Status> { todo!() }

  ///
  /// Send message to game chat.
  ///
  async fn push_mgs(&self, _request : Request<Msg>) -> Result<Response<()>, Status> { todo!() }

  ///
  /// Get messages from chat.
  ///
  async fn read_msgs(&self, _request : Request<GameId>) -> Result<Response<Msgs>, Status> { todo!() }
}
