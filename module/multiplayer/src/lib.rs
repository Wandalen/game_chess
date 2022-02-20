#[allow(non_camel_case_types)]
pub mod generated;
use generated::chess::GamePlayer;

use time::{OffsetDateTime, format_description};

use game_chess_core::GameStatus;
pub use generated::chess::MultiplayerGame;

///
/// Message.
///

#[allow(dead_code)]
pub struct MultiplayerMessage
{
  player_id : String,
  text : String,
  timestamp : OffsetDateTime,
}

impl MultiplayerMessage {
  pub fn new(player_id: String, text: String) -> Self {
    Self { player_id, text, timestamp: OffsetDateTime::now_utc() }
  }

  pub fn pretty_print(&self) -> String {
    let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
    format!("[{}][{}]>> {}", self.player_id, self.timestamp.format(&format).unwrap(), self.text)
  }
}

///
/// Player.
///

#[allow(dead_code)]
#[derive(Debug)]
pub struct MultiplayerPlayer
{
  id : String,
  name : String,
}

///
/// Move.
///

pub struct MultiplayerMove
{
  /// Player id.
  pub player_id : String,
  /// Game id.
  pub game_id : String,
}

impl MultiplayerMove {}

pub enum MultiplayerStatus {
  NotStarted = 0,
  Started = 1,
  Ended = 2
}

impl MultiplayerGame
{
  pub fn new(id: String, player: GamePlayer, status: i32) -> Self
  {
    Self { game_id: id, players: Vec::from([player]), status }
  }

  pub fn add_opponent(&mut self, player: GamePlayer) { self.players.push(player) }
}
