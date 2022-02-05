//!
//! Implements in-memory storage.
//!

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
  async fn add_game(&mut self, _game : Game) { todo!() }

  ///
  /// Get game from storage by string ( slice ) id.
  ///
  async fn get_game(&self, _game_id : &str) -> &Game { todo!() }

  ///
  /// Get all stored games.
  ///
  async fn get_games(&self) -> &Vec<Game> { todo!() }

  ///
  /// Update game in storage using string id and new instance of Game.
  ///
  async fn update_game(&mut self, _game_id : &str, _new_game : Game) { todo!() }
}
