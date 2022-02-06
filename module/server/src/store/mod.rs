pub mod memory;

use multiplayer::MultiplayerGame;

///
/// Implements methods for server storage.
///

#[tonic::async_trait]
pub trait GameStore
{
  /// Add game to storage.
  fn add_game(&mut self, game : MultiplayerGame);
  /// Get game from storage by string ( slice ) id.
  fn get_game(&self, game_id : &str) -> &MultiplayerGame;
  /// Get all stored games.
  fn get_games(&self) -> &Vec<MultiplayerGame>;
  /// Update game in storage using string id and new instance of Game.
  fn update_game(&mut self, game_id : &str, new_game : MultiplayerGame);
}
