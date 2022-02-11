//!
//! RPC server that provides game API.
//!

use std::collections::HashMap;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use futures::Stream;

use multiplayer::MultiplayerPlayer;
use tonic::{Request, Response, Status};
use tokio::sync::mpsc;

use multiplayer::generated::chess::chess_server::Chess;
use crate::store::{GameStore, self};
use multiplayer::generated::chess::{self, Board, GameState, Games, CreateGame, GameId, AcceptGame, GameMove, GamePlayer, Msg};
use crate::store::memory::MemoryStore;

type ResponseStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;

///
/// Shared sever.
///
#[allow(missing_debug_implementations, dead_code)]
pub struct ChessRpcServer
{
  store : Arc<tokio::sync::Mutex<dyn GameStore + Send + Sync>>,
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
      store : Arc::new(tokio::sync::Mutex::new(MemoryStore::new())),
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
  async fn push_game_create(&self, request : Request<CreateGame>) -> Result<Response<GameId>, Status>
  {
    let mut store = self.store.lock().await;

    if let Some(player) = request.into_inner().player {
      // Generates random game id each time!
      use rand::{distributions::Alphanumeric, Rng};
      let game_id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();

      store.add_game(
        multiplayer::MultiplayerGame::new(
          game_id.clone(),
          MultiplayerPlayer::new(player.player_id, player.player_name)
        )
      ).await;

      Ok(Response::new(GameId { game_id }))
    } else {
      Err(Status::invalid_argument("No player found!"))
    }
  }

  ///
  /// Accept request to join game.
  ///
  async fn push_game_accept(&self, request : Request<AcceptGame>) -> Result<Response<GameId>, Status>
  {
    let game_req = request.into_inner();
    let mut store = self.store.lock().await;

    if let Some(player) = game_req.player_id {
      let game_id = game_req.game_id;

      let game = store.get_game(&game_id).await;
      let game_player = game.get_first_player();

      let mut game = multiplayer::MultiplayerGame::new(game_id.clone(), game_player);
      game.add_opponent(
        MultiplayerPlayer::new(player.player_id, player.player_name)
      );

      store.update_game(&game_id, game).await;

      Ok(Response::new(GameId { game_id }))
    } else {
      Err(Status::invalid_argument("No player found!"))
    }
  }

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
  async fn pull_games_list(&self, _request : Request<()>) -> Result<Response<Games>, Status>
  { 
    let store = self.store.lock().await;
    let games = store.get_games().await;

    if games.len() > 0 {
      let games_info: Vec<chess::GameInfo> = games.iter().map(|game| {
        chess::GameInfo {
          game_id: game.id.to_owned(),
          players: game.get_players().iter().map(|item| { item.into_player() }).collect()
        }}
      )
      .collect();

      Ok(Response::new(Games { games_info: games_info }))
    } else {
      Err(Status::invalid_argument("No game found!"))
    }
  }

  ///
  /// Send request to forfeit.
  ///
  async fn push_game_gg(&self, _request : Request<GamePlayer>) -> Result<Response<()>, Status> { todo!() }

  ///
  /// Send message to game chat.
  ///
  async fn push_mgs(&self, _request : Request<Msg>) -> Result<Response<()>, Status> { todo!() }

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
