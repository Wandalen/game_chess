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
  saved_time : Option< u64 >, // Player time before pause
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
  /// Resets current time with using saved time
  ///
  pub fn resume( &mut self )
  {
    // if already resume - second one will be ignored
    if let Some( time ) = self.saved_time
    {
      self.timestamp = get_unix_timestamp( None ) - time;
      self.saved_time = None;
    }
  }

  ///
  /// Pause with saving current player time
  ///
  pub fn pause( &mut self )
  {
    // if already paused - second one will be ignored
    if let None = self.saved_time
    {
      let time_now = get_unix_timestamp( None );
      self.saved_time = Some( time_now - self.timestamp );
    }
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
    // if game is paused - ignore switching
    //? may be it must returns a result
    //? like "Can not switch turn on pause"(As Error type idk)
    if let Some( _ ) = self.saved_time{ return 0; }

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
    // freeze time on pause
    if let Some( time ) = self.saved_time
    {
      if self.whos_turn == player_number
      { return self.players_time[ player_number ] - time; }
      else
      { return self.players_time[ player_number ] }
    }
    let time_now = get_unix_timestamp( None );
    let diff = if self.whos_turn == player_number
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

  #[ test ]
  fn pauses_and_resumes()
  {
    let mut timer = Timer::new( 100, 50 );

    // spent time before pause
    timer.timestamp -= 20;

    assert!( timer.get_player_time( 0 ) <= 80 );
    assert_eq!( timer.get_player_time( 1 ), 100 );

    timer.pause();

    // spent time on pause
    timer.timestamp -= 50;

    // time for players must be the same as before pause
    assert!( timer.get_player_time( 0 ) <= 80 );
    assert_eq!( timer.get_player_time( 1 ), 100 );

    timer.resume();

    // time for players must be the same as before pause
    assert!( timer.get_player_time( 0 ) <= 80 );
    assert_eq!( timer.get_player_time( 1 ), 100 );

    // double resume must be ignored
    timer.resume();

    assert!( timer.get_player_time( 0 ) <= 80 );
    assert_eq!( timer.get_player_time( 1 ), 100 );

    timer.timestamp -= 20;

    // double pause saves the first state
    timer.pause();

    timer.timestamp -= 40;

    timer.pause();

    assert!( timer.get_player_time( 0 ) <= 60 );
    assert_eq!( timer.get_player_time( 1 ), 100 );

    assert_eq!( timer.whos_turn, 0 );
    // must be ignored by the pause
    timer.switch_turn();
    assert_eq!( timer.whos_turn, 0 );

    // and after resume too
    timer.resume();
    assert_eq!( timer.whos_turn, 0 );
  }
}
