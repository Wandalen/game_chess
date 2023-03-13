//!
//! Common data structures
//!

use bevy::prelude::States;

///
/// Game state enum
///

#[ derive( Debug, Clone, Eq, PartialEq, Hash, Copy, Default, States, ) ]
pub enum GameState
{
  #[default]
  /// Intial state
  Init,
  /// Main menu
  MainMenu,
  /// When we create a new game
  GameNew,
  /// When we are playing the game
  GamePlaying,
  /// Multiplayer game
  // MultiplayerGame( Multiplayer ),
  /// When game is on pause
  Pause,
  /// Settings menu
  Settings,
}

///
/// Multiplayer game's states
///

#[ derive( Debug, Copy, Clone, Eq, PartialEq, Hash ) ]
pub enum Multiplayer
{
  /// Connection setup
  ConnectingToServer,
  /// List available games on server
  ListGames,
  /// Connecting to a game on a server
  LoadingGame,
  /// Waiting in a created lobby
  WaitingForOpponent,
  /// Game in progress
  Playing,
}
