pub mod memory;

use tonic::async_trait;
use game_chess_core::model::Game;

#[tonic::async_trait]
pub trait GameStore {
  async fn add_game(&mut self, game: Game);
  async fn get_game(&self, game_id: &str) -> &Game;
  async fn get_games(&self) -> &Vec<Game>;
  async fn update_game(&mut self, game_id: &str, new_game: Game);
}
