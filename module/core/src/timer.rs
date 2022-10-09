//!
//!  Implement timer for the chess game.
//!

use serde::{ Serialize, Deserialize };

use crate::get_unix_timestamp;

///
///   Timer which count time for all players
/// 

#[ derive( Debug, Default, Serialize, Deserialize ) ]
pub struct Timer
{
  whos_turn : usize,
  timestamp : u64, // Current timestamp
  players_time : [ u64; 2 ], // How much time do players have
}

impl Timer
{
  ///
  /// Construct a new timer with time setting for players
  /// 
  pub fn new( time_for_players : u64 ) -> Self
  {
    Timer
    {
      timestamp : get_unix_timestamp( None ),
      players_time : [ time_for_players, time_for_players ],
      ..Default::default()
    }
  }

  ///
  /// Swtich turn to next player
  /// 
  pub fn switch_turn( &mut self )
  {
    let time_now = get_unix_timestamp( None );
    let diff = time_now - self.timestamp;
    
    self.players_time[ self.whos_turn ] = self.players_time[ self.whos_turn ].checked_sub( diff ).unwrap_or_default();
    self.timestamp = time_now;

    self.whos_turn = if self.whos_turn + 1 == self.players_time.len()
    { 0 } else { self.whos_turn + 1 };
  }

  ///
  /// Gets time which player have by his number
  /// 
  pub fn get_player_time( &self, player_number : usize ) -> u64
  {
    let time_now = get_unix_timestamp( None );
    let diff = 
    if self.whos_turn == player_number
    { time_now - self.timestamp }
    else
    { 0 };
    self.players_time[ player_number ].checked_sub( diff ).unwrap_or_default()
  }

  ///
  /// Returns true if any player ran out of time
  /// 
  pub fn time_is_out( &self ) -> bool
  {
    self.players_time.iter().any( | player_time | player_time == &0 )
  }
}
