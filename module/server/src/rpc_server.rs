//!
//! RPC server that provides game API.
//!

use std::collections::HashMap;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use futures::Stream;

use tonic::{Request, Response, Status};
use tokio::sync::mpsc;

use multiplayer::generated::chess::chess_server::Chess;
use crate::store::GameStore;
use multiplayer::generated::chess::{self, Board, GameState, multiplayer_game::GameStatus, Games, CreateGame, GameId, AcceptGame, GameMove, GamePlayer, Msg, Msgs};
use crate::store::memory::MemoryStore;

type ResponseStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;

///
/// Shared sever.
///
#[allow(missing_debug_implementations, dead_code)]
pub struct ChessRpcServer
{
  store : Arc<Mutex<dyn GameStore + Send + Sync>>,
  streams : Arc<Mutex<HashMap<SocketAddr, mpsc::UnboundedSender<Result<chess::GameUpdate, Status>>>>>,
}

impl ChessRpcServer
{
  ///
  /// Server constructor.
  ///
  pub fn init() -> Self
  {
    Self {
      store : Arc::new(Mutex::new(MemoryStore::new())),
      streams : Arc::new(Mutex::new(HashMap::new())),
    }
  }
}

impl ChessRpcServer
{
  /// Broadcasts `GameUpdate` to all clients.
  pub async fn push_game_update(&self, game_update : chess::game_update::GameUpdate)
  {
    let streams_lock = self.streams.lock().expect("Failed to lock the streams mutex");
    for (client_addr, stream) in streams_lock.iter()
    {
      let game_update = chess::GameUpdate {
        game_update : Some(game_update.clone()),
      };
      if let Err(err) = stream.send(Ok(game_update))
      {
        eprintln!("Failed to send GameUpdate to {}: {}", client_addr, err);
      }
    }
  }
}

#[tonic::async_trait]
impl Chess for ChessRpcServer
{
  #[allow(non_camel_case_types)]
  type pull_game_updatesStream = ResponseStream<chess::GameUpdate>;

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
  async fn pull_board_state(&self, _request : Request<GameId>) -> Result<Response<Board>, Status> { todo!() }

  ///
  /// Get info about current game state - positions and history.
  ///
  async fn pull_game_state(&self, _request : Request<GameId>) -> Result<Response<GameState>, Status> { todo!() }

  ///
  /// Get list of games.
  ///
  async fn pull_games_list(&self, _request : Request<()>) -> Result<Response<Games>, Status> { todo!() }

  ///
  /// Send request to forfeit.
  ///
  async fn push_game_gg(&self, _request : Request<GamePlayer>) -> Result<Response<()>, Status> {
    let message = _request.into_inner();
    let game_id = message.game_id;
    let player_id = message.player_id;

    let winner =
    {
      let mut memory_store = self.store.lock().unwrap();
      let mut winner = None;
      let mut current_game = memory_store.get_game(&game_id).clone();

      memory_store.update_game(&game_id, current_game.clone());
      for player in &current_game.players {
        if player.player_id != player_id {
            winner = Some(player.clone());
            break;
        }
      }

      current_game.status = GameStatus::Givenup as i32;
      winner
    };
    // Will be moved from function push_mgs
    //
    // let request;
    // if let Some(winner) = winner {
    //   let msg = format!("The player {} gave up. The player {} is the winner!", player_id, winner.player_id);
    //   request = Request::new(Msg {
    //     player: Some(message),
    //     text: msg.into(),
    //   });
    // }

    // return self.push_mgs(request).await;

    Ok(Response::new(()))
  }

  ///
  /// Send message to game chat.
  ///
  async fn push_mgs(&self, _request : Request<Msg>) -> Result<Response<()>, Status> {
    let message = _request.into_inner();
    let game_id = message.game_id;
    let mut msg_store = self.store.lock().unwrap();

    msg_store.send_msg(&game_id, message);

    Ok(Response::new(()))
  }

  async fn pull_game_updates(&self, request : Request<GameId>) -> Result<Response<Self::pull_game_updatesStream>, Status>
  {
    let mut streams = self.streams.lock().expect("Failed to lock the streams mutex");
    let (tx, rx) = mpsc::unbounded_channel();
    streams.insert(request.remote_addr().expect("Expected a client address"), tx);

    Ok(Response::new(
      Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(rx)) as Self::pull_game_updatesStream,
    ))
  }
}
