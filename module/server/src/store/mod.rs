pub mod memory;

use tonic::async_trait;
use multiplayer::MultiplayerGame;

#[tonic::async_trait]
pub trait GameStore
{
  async fn add_game(&mut self, game : MultiplayerGame);
  async fn get_game(&self, game_id : &str) -> &MultiplayerGame;
  async fn get_games(&self) -> &Vec<MultiplayerGame>;
  async fn update_game(&mut self, game_id : &str, new_game : MultiplayerGame);
}
