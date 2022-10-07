//!
//! Common data structures
//!

///
/// Game state enum
///

#[ derive( Debug, Clone, Eq, PartialEq, Hash ) ]
pub enum GameState
{
  /// Intial state
  Init,
  /// Main menu
  MainMenu,
  /// When we create a new game
  GameNew,
  /// When we start a new game
  GameStart,
  /// When we are playing the game
  GamePlaying,
}
