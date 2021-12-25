#![warn( missing_docs )]

//! Graphical interface for the chess game.

use game_chess_core as core;
use bevy::prelude::*;

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
  app.add_system( bevy::input::system::exit_on_esc_system.system() );
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
  asset_server : Res< AssetServer >,
  mut materials : ResMut< Assets< ColorMaterial > >,
)
{
  /* load image */
  let texture_handle = asset_server.load( "icon.png" );
  /* camera */
  commands.spawn_bundle( OrthographicCameraBundle::new_2d() );
  /* sprie */
  let sprite = SpriteBundle
  {
    material : materials.add( texture_handle.into() ),
    ..Default::default()
  };
  /* go live */
  commands.spawn_bundle( sprite );
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
