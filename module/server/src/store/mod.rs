pub mod memory;

use game_chess_core::{UCI, Player, MoveList};
use multiplayer::{MultiplayerGame, MultiplayerMessage};

///
/// Implements methods for server storage.
///

#[tonic::async_trait]
pub trait GameStore
{
  /// Add game to storage.
  fn add_game(&mut self, game : MultiplayerGame) -> Result<(), String>;
  /// Get game from storage by string ( slice ) id.
  fn get_game(&self, game_id : &str) -> &MultiplayerGame;
  /// Get all stored games.
  fn get_games(&self) -> &Vec<MultiplayerGame>;
  /// Update game in storage using string id and new instance of Game.
  fn update_game(&mut self, game_id : &str, new_game : MultiplayerGame);
  /// Add chat messages to storage
  fn add_chat(&mut self, game_id: &str, message: MultiplayerMessage);
  /// Get chat messages from storage by `game_id`.
  fn get_chats(&self, game_id: &str, player_id: &str) -> Vec<MultiplayerMessage>;
  /// Get board state from storage by `game_id`.
  fn get_board_state(&self, game_id: &str) -> Option<String>;
  /// Return the `Player` e.g. [Black/White] whose turn it is to move.
  fn current_turn(&self, game_id: &str) -> Player;
  /// Return the last move played, if any.
  fn last_move(&self, game_id: &str) -> Option<UCI>;
  /// Checks the validity of a given move.
  fn move_validity(&self, game_id: &str, r#move: &str) -> bool;
  /// Makes a move on the board.
  fn make_move(&mut self, game_id: &str, r#move: &str) -> bool;
  /// Returns available moves on the board.
  fn moves_list(&self, game_id: &str) -> MoveList;
}
