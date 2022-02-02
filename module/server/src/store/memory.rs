use tonic::async_trait;
use game_chess_core::model::Game;

use crate::store::GameStore;

pub struct MemoryStore {
  games: Vec<Game>,
}

impl MemoryStore {
  pub fn new() -> Self {
    Self { games: Vec::new() }
  }
}

#[tonic::async_trait]
impl GameStore for MemoryStore {
  async fn add_game(&mut self, game: Game) {
    todo!()
  }

  async fn get_game(&self, game_id: &str) -> &Game {
    todo!()
  }

  async fn get_games(&self) -> &Vec<Game> {
    todo!()
  }

  async fn update_game(&mut self, game_id: &str, new_game: Game) {
    todo!()
  }
}
