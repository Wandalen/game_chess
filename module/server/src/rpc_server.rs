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

use multiplayer::{MultiplayerStatus, MultiplayerMessage};
use multiplayer::generated::chess::chess_server::Chess;
use crate::store::GameStore;
use multiplayer::generated::chess::{self, Board, GameState, Games, CreateGame, GameId, AcceptGame, GameMove, GamePlayer, Msg, Msgs};
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
  async fn push_game_create(&self, request : Request<CreateGame>) -> Result<Response<GameId>, Status>
  {
    let mut store = self.store.lock().expect("Failed to lock the store mutex");

    if let Some(player) = request.into_inner().player {
      store.add_game(
        multiplayer::MultiplayerGame::new(
          player.game_id.to_string(),
          GamePlayer { game_id: player.game_id.to_string(), player_id: player.player_id },
          MultiplayerStatus::NotStarted as i32
        )
      );

      Ok(Response::new(GameId { game_id: player.game_id }))
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
    let input_game_id = game_req.game_id;

    let mut store = self.store.lock().expect("Failed to lock the store mutex");

    if let Some(player) = game_req.player_id {
      let input_player_id = player.player_id;

      // This will panic if `input_game_id` not found on he store
      let game = store.get_game(&input_game_id);

      let mut game = multiplayer::MultiplayerGame::new(
        game.game_id.to_string(),
        GamePlayer {
          game_id: game.players[0].game_id.to_string(),
          player_id: game.players[0].player_id.to_string()
        },
        MultiplayerStatus::Started as i32
      );

      game.add_opponent(GamePlayer {
        game_id: input_game_id.to_string(),
        player_id: input_player_id
      });

      store.update_game(&input_game_id.to_string(), game);
      Ok(Response::new(GameId { game_id: input_game_id }))
    } else {
      Err(Status::not_found("No player found on input!"))
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
    let store = self.store.lock().expect("Failed to lock the store mutex");
    let games = store.get_games();

    if games.len() > 0
    {
      Ok(Response::new(Games { games : games.clone() }))
    }
    else
    {
      Err(Status::not_found("No game found on server!"))
    }
  }

  ///
  /// Send request to forfeit.
  ///
  async fn push_game_gg(&self, _request : Request<GamePlayer>) -> Result<Response<()>, Status>
  {
    let message = _request.into_inner();
    let game_id = message.game_id;
    let player_id = message.player_id;

    let winner = {
      let mut memory_store = self.store.lock().unwrap();
      let mut winner = None;
      let mut current_game = memory_store.get_game(&game_id).clone();

      memory_store.update_game(&game_id, current_game.clone());
      for player in &current_game.players
      {
        if player.player_id != player_id
        {
          winner = Some(player.clone());
          break;
        }
      }

      current_game.status = MultiplayerStatus::Ended as i32;
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
  async fn push_msg(&self, request : Request<Msg>) -> Result<Response<()>, Status>
  {
    let msg_req = request.into_inner();
    let player = msg_req.player;

    if let Some(player) = player {
      let msg = MultiplayerMessage::new(player.player_id, msg_req.text);

      let mut store = self.store.lock().expect("Failed to lock the store mutex");
      store.add_chat(&player.game_id, msg);

      Ok(Response::new(()))
    } else {
      Err(Status::not_found("No player found on input!"))
    }
  }

  ///
  /// Read messages from game chat.
  ///
  async fn read_msgs(&self, request : Request<GamePlayer>) -> Result<Response<Msgs>, Status>
  {
    let player = request.into_inner();
    let mut msgs = Msgs { messages: Vec::new() };

    let store = self.store.lock().expect("Failed to lock the store mutex");

    // Does not guarantees ordered message
    let chats = store.get_chats(&player.game_id, &player.player_id);
    for msg in chats.iter() { msgs.messages.push(msg.pretty_print()); }
    
    if msgs.messages.len() == 0 { msgs.messages.push("No Chat Messages!".to_owned()); }
    Ok(Response::new(msgs))
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
