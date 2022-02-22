//!
//! Implements in-memory storage.
//!

use std::{borrow::BorrowMut, collections::HashMap};

#[allow(unused_imports)]
use tonic::async_trait;
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
  chats: HashMap<String, Vec<Chat>>
}

impl MemoryStore
{
  ///
  /// Storage constructor.
  ///
  pub fn new() -> Self { Self { games : Vec::new(), chats: HashMap::new() } }
}

#[tonic::async_trait]
impl GameStore for MemoryStore
{
  ///
  /// Add game to storage.
  ///
  fn add_game(&mut self, game : Game) { self.games.push(game) }

  ///
  /// Get game from storage by string ( slice ) id.
  ///
  fn get_game(&self, game_id : &str) -> &Game { self.games.iter().find(|game| game.game_id == game_id).unwrap() }

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
  fn add_chat(&mut self, game_id: &str, message: Chat) {
    if self.chats.contains_key(game_id) {
      self.chats.get_mut(game_id).unwrap().push(message);
    } else {
      self.chats.insert(game_id.to_owned(), vec![message]);
    }
  }

  ///
  /// Get chat messages from storage by `game_id`.
  /// 
  fn get_chats(&self, game_id: &str, _player_id: &str) -> Vec<Chat> {
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
}
