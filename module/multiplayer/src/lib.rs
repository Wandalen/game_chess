#[allow(non_camel_case_types)]
pub mod generated;

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

impl MultiplayerPlayer {}

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
  id : String,
  players : Vec<MultiplayerPlayer>,
}

impl MultiplayerGame {}
