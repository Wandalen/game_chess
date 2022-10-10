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
  time_bonuses : u64, // Time bonuses
  players_time : [ u64; 2 ], // How much time do players have
}

impl Timer
{
  ///
  /// Construct a new timer with time setting for players
  /// 
  pub fn new( time_for_players : u64, time_bonuses : u64 ) -> Self
  {
    Timer
    {
      timestamp : get_unix_timestamp( None ),
      players_time : [ time_for_players, time_for_players ],
      time_bonuses,
      ..Default::default()
    }
  }

  ///
  /// Resets current time
  /// 
  pub fn reset_time( &mut self )
  {
    self.timestamp = get_unix_timestamp( None )
  }

  ///
  /// Sets time bonuses
  /// 
  pub fn set_time_bonuses( &mut self, bonus : u64 )
  {
    self.time_bonuses = bonus
  }

  ///
  /// Swtich turn to next player
  /// Returns spent time for a move
  /// 
  pub fn switch_turn( &mut self ) -> u64
  {
    let time_now = get_unix_timestamp( None );
    let diff = time_now - self.timestamp;
    
    self.players_time[ self.whos_turn ] = self.players_time[ self.whos_turn ]
    // if None - player loss
    .checked_sub( diff )
    // if player not lose give bonuses
    .and_then( | time | Some( time + self.time_bonuses ) )
    .unwrap_or_default();
    self.timestamp = time_now;

    self.whos_turn = if self.whos_turn + 1 == self.players_time.len()
    { 0 } else { self.whos_turn + 1 };

    diff
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
    self.players_time.iter().enumerate().any( |( number, _ )| self.get_player_time( number ) == 0 )
  }
}


#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn basic()
  {
    let mut timer = Timer::new( 100, 0 );

    // init statement
    assert_eq!( timer.whos_turn, 0 );
    assert_eq!( timer.players_time, [ 100, 100 ] );

    assert!( !timer.time_is_out() );
    assert!( timer.get_player_time( 0 ) <= 100 );
    assert_eq!( timer.get_player_time( 1 ), 100 );

    // 50 seconds later for first player
    timer.timestamp -= 50;

    assert!( timer.get_player_time( 0 ) <= 50 );
    assert_eq!( timer.get_player_time( 1 ), 100 );

    assert!( timer.switch_turn() >= 50 );

    assert_eq!( timer.whos_turn, 1 );
    assert!( timer.get_player_time( 0 ) <= 50 );
    assert!( timer.get_player_time( 1 ) <= 100 );

    // 70 seconds later for second player
    timer.timestamp -= 70;

    assert!( timer.get_player_time( 0 ) <= 50 );
    assert!( timer.get_player_time( 1 ) <= 30 );

    assert!( timer.switch_turn() >= 70 );

    timer.timestamp -= 70;

    assert_eq!( timer.whos_turn, 0 );
    assert_eq!( timer.get_player_time( 0 ), 0 );

    // end game
    assert!( timer.time_is_out() );
  }

  #[ test ]
  fn with_bonuses()
  {
    let mut timer = Timer::new( 100, 50 );

    assert!( timer.get_player_time( 0 ) <= 100 );
    assert_eq!( timer.get_player_time( 1 ), 100 );

    timer.switch_turn();

    // first player got bonuses on end of move
    let first_player_time = timer.get_player_time( 0 );
    assert!( first_player_time > 100 && first_player_time <= 150 );
    assert_eq!( timer.get_player_time( 1 ), 100 );

    timer.switch_turn();

    // second player got bonuses on end of move
    let first_player_time = timer.get_player_time( 0 );
    let second_player_time = timer.get_player_time( 1 );
    assert!( first_player_time > 100 && first_player_time <= 150 );
    assert!( second_player_time > 100 && second_player_time <= 150 );

    // player have no time
    timer.players_time[ 0 ] = 0;
    // and move lasted, at least a second
    timer.timestamp -= 1;

    timer.switch_turn();

    // The player must lose
    assert_eq!( timer.get_player_time( 0 ), 0 );
    assert!( timer.time_is_out() );

  }
}