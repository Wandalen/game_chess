#[allow(non_camel_case_types)]
pub mod generated;
use generated::chess::Player;

use time::OffsetDateTime;

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

impl MultiplayerMessage {}

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

impl MultiplayerPlayer {
  pub fn new(id: String, name: String) -> Self {
    Self { id, name }
  }

  pub fn into_player(&self) -> Player {
    Player {
      player_id: self.id.to_string(),
      player_name: self.name.to_string()
    }
  }
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

///
/// Multiplayer game.
///

#[allow(dead_code)]
#[derive(Debug)]
pub struct MultiplayerGame
{
  pub id : String,
  players : Vec<MultiplayerPlayer>,
}

impl MultiplayerGame {
  pub fn new(id: String, player: MultiplayerPlayer) -> Self {
    Self { id, players: Vec::from([player]) }
  }

  pub fn add_opponent(&mut self, player: MultiplayerPlayer) {
    self.players.push(player)
  }

  pub fn get_first_player(&self) -> MultiplayerPlayer {
    MultiplayerPlayer {
      id: self.players[0].id.to_string(),
      name: self.players[0].name.to_string()
    }
  }

  pub fn get_players(&self) -> &Vec<MultiplayerPlayer> { self.players.as_ref() }
}
