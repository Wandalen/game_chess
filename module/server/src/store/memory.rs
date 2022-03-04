//!
//! Implements in-memory storage.
//!

use std::{borrow::BorrowMut, collections::HashMap};

#[allow(unused_imports)]
use tonic::async_trait;
use game_chess_core::{Game as GameInstance, UCI, Player, MoveList};
use multiplayer::{MultiplayerGame as Game, MultiplayerMessage as Chat};

use crate::store::GameStore;

///
/// Storage structure.
///

#[derive(Debug)]
pub struct MemoryStore
{
  #[allow(dead_code)]
  games : Vec<Game>,
  chats: HashMap<String, Vec<Chat>>,
  game_instances: HashMap<String, GameInstance>
}

impl MemoryStore
{
  ///
  /// Storage constructor.
  ///
  pub fn new() -> Self
  {
    Self { games : Vec::new(), chats : HashMap::new(), game_instances : HashMap::new() }
  }
}

#[tonic::async_trait]
impl GameStore for MemoryStore
{
  ///
  /// Add game to storage.
  ///
  fn add_game(&mut self, game : Game) -> Result<(), String>
  {
    if self.game_instances.contains_key(&game.game_id) {
      Err(format!("Game ID: {} already exists. Try different ID!", &game.game_id))
    } else {
      self.game_instances.insert(game.game_id.to_string(), GameInstance::default());
      self.games.push(game);

      Ok(())
    }
  }

  ///
  /// Get game from storage by string ( slice ) id.
  ///
  fn get_game(&self, game_id : &str) -> Option<&Game>
  {
    self.games.iter().find(|game| game.game_id == game_id)
  }

  ///
  /// Get all stored games.
  ///
  fn get_games(&self) -> &Vec<Game> { &self.games }

  ///
  /// Update game in storage using string id and new instance of Game.
  ///
  fn update_game(&mut self, game_id : &str, new_game : Game)
  {
    if let Some(g) = self.games.iter_mut().find(|item| item.game_id == game_id)
    {
      *g = new_game;
    }
  }

  ///
  /// Add chat messages to storage.
  /// 
  fn add_chat(&mut self, game_id: &str, message: Chat)
  {
    if self.chats.contains_key(game_id) {
      self.chats.get_mut(game_id).unwrap().push(message);
    } else {
      self.chats.insert(game_id.to_owned(), vec![message]);
    }
  }

  ///
  /// Get chat messages from storage by `game_id`.
  /// 
  fn get_chats(&self, game_id: &str, _player_id: &str) -> Vec<Chat>
  {
    let mut chats = Vec::new();

    if self.chats.contains_key(game_id) {
      let messages = self.chats.get(game_id).unwrap();
      for msg in messages {
        // Toggle comment between following lines only if
        // Opponent and spectator (if implemented) chats are desired.

        chats.push(msg.clone())
        // if msg.player_id != _player_id { chats.push(msg.clone()) }
      }
    }
    chats
  }

  ///
  /// Get board state from storage by `game_id`.
  /// 
  fn get_board_state(&self, game_id: &str) -> Option<String>
  {
    if self.game_instances.contains_key(game_id) {
      Some(self.game_instances.get(game_id).unwrap().board_state_printable())
    } else {
      None
    }
  }

  ///
  /// Return the `Player` e.g. [Black/White] whose turn it is to move.
  /// 
  fn current_turn(&self, game_id: &str) -> Player
  {
    // Assumes `game_id` has already been checked! 
    self.game_instances.get(game_id).unwrap().current_turn()
  }

  ///
  /// Return the last move played, if any.
  ///
  fn last_move(&self, game_id: &str) -> Option<UCI>
  {
    // Assumes `game_id` has already been checked! 
    self.game_instances.get(game_id).unwrap().last_move()
  }

  ///
  /// Checks the validity of a given move.
  /// 
  fn move_validity(&self, game_id: &str, r#move: &str) -> bool {
    let uci_move = UCI::from(r#move);

    // Assumes `game_id` has already been checked!
    self.game_instances.get(game_id).unwrap().move_is_valid(uci_move)
  }

  ///
  /// Makes a move on the board.
  /// 
  fn make_move(&mut self, game_id: &str, r#move: &str) -> bool {
    let uci_move = UCI::from(r#move);
    
    // Assumes `game_id` has already been checked!
    self.game_instances.get_mut(game_id).unwrap().make_move(uci_move)
  }

  ///
  /// Returns available moves on the board
  /// 
  fn moves_list(&self, game_id: &str) -> MoveList {
    // Assumes `game_id` has already been checked!
    self.game_instances.get(game_id).unwrap().moves_list()
  }
}
