pub mod generated;

use time::OffsetDateTime;

pub struct MultiplayerMessage {
    player_id: String,
    text: String,
    timestamp: OffsetDateTime,
}

impl MultiplayerMessage {

}

#[derive(Debug)]
pub struct MultiplayerPlayer {
    id: String,
    name: String,
}

impl MultiplayerPlayer {

}

pub struct MultiplayerMove {
    pub player_id: String,
    pub game_id: String,
}

impl MultiplayerMove {

}

#[derive(Debug)]
pub struct MultiplayerGame {
    id: String,
    players: Vec<MultiplayerPlayer>,
}

impl MultiplayerGame {

}
