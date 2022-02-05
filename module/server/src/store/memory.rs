//!
//! Implements in-memory storage.
//!

use std::borrow::BorrowMut;

#[allow(unused_imports)]
use tonic::async_trait;
use multiplayer::MultiplayerGame as Game;

use crate::store::GameStore;

///
/// Storage structure.
///

#[derive(Debug)]
pub struct MemoryStore
{
  #[allow(dead_code)]
  games : Vec<Game>,
}

impl MemoryStore
{
  ///
  /// Storage constructor.
  ///
  pub fn new() -> Self { Self { games : Vec::new() } }
}

#[tonic::async_trait]
impl GameStore for MemoryStore
{
  ///
  /// Add game to storage.
  ///
  async fn add_game(&mut self, game : Game) { self.games.push(game) }

  ///
  /// Get game from storage by string ( slice ) id.
  ///
  async fn get_game(&self, game_id : &str) -> &Game { self.games.iter().find(|game| game.id == game_id).unwrap() }

  ///
  /// Get all stored games.
  ///
  async fn get_games(&self) -> &Vec<Game> { &self.games }

  ///
  /// Update game in storage using string id and new instance of Game.
  ///
  async fn update_game(&mut self, game_id : &str, new_game : Game)
  {
    if let Some(g) = self.games.iter_mut().find(|item| item.id == game_id)
    {
      *g = new_game;
    }
  }
}
