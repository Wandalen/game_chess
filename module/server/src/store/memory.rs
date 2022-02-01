use crate::store::GameStore;
use tonic::async_trait;

pub struct MemoryStore {

}

#[tonic::async_trait]
impl GameStore for MemoryStore {
    async fn create_game(&mut self) {
        todo!()
    }

    async fn get_game(&self, game_id: String) {
        todo!()
    }

    async fn get_games(&self) {
        todo!()
    }

    async fn update_game(&mut self) {
        todo!()
    }
}
