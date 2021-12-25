#![warn( missing_docs )]

//!
//! Chess game implemented on Bevy for educational purpose.
//!

use game_chess_core as core;
use bevy::prelude::*;
use bevy::input::system::exit_on_esc_system;

///
/// Main.
///

fn main()
{
  let mut app = App::build();
  /* default plugins */
  app.add_plugins( DefaultPlugins );
  /* background */
  app.insert_resource( ClearColor( Color::rgb( 0.9, 0.9, 0.9 ) ) );
  /* setup core */
  app.add_startup_system( core_setup.system() );
  /* setup graphics */
  app.add_startup_system( graphics_setup.system() );
  /* escape on exit */
  app.add_system( exit_on_esc_system.system() );
  /* for web target */
  #[cfg(target_arch = "wasm32")]
  app.add_plugin( bevy_webgl2::WebGL2Plugin );
  /* run */
  app.run();
}

///
/// Graphics setup.
///

fn graphics_setup
(
  mut commands : Commands,
)
{
  /* camera */
  commands.spawn_bundle( OrthographicCameraBundle::new_2d() );
}


///
/// Startup system for the game.
///

fn core_setup()
{
  let mut game = core::Game::default();
  game.board_print();
  game.make_move( "a2a4" );
  game.board_print();
}
