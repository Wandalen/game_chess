pub mod memory;

use tonic::async_trait;

#[tonic::async_trait]
pub trait GameStore {
    async fn create_game(&mut self);
    async fn get_game(&self, game_id: String);
    async fn get_games(&self);
    async fn update_game(&mut self);
}
