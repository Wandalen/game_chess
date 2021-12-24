#![warn( missing_docs )]

//! Graphical interface

use game_chess_core as core;
use bevy::prelude::*;

//

#[allow( dead_code )]
fn main()
{
  let mut app = App::build();
  app.add_plugins( DefaultPlugins );
  app.insert_resource( ClearColor( Color::rgb( 0.9, 0.9, 0.9 ) ) );
  #[cfg(target_arch = "wasm32")]
  app.add_plugin( bevy_webgl2::WebGL2Plugin );
  app.add_startup_system( setup.system() );
  app.add_startup_system( core_setup.system() );
  app.run();
}

//

/// Startup system example
#[allow( dead_code )]
fn setup
(
  mut commands : Commands,
  asset_server : Res< AssetServer >,
  mut materials : ResMut<Assets< ColorMaterial > >,
)
{
  let texture_handle = asset_server.load( "icon.png" );
  commands.spawn_bundle( OrthographicCameraBundle::new_2d() );
  let sprite = SpriteBundle
  {
    material : materials.add(texture_handle.into()),
    ..Default::default()
  };
  commands.spawn_bundle( sprite );
}

//

/// Startup system for the game
#[allow( dead_code )]
fn core_setup()
{
  let mut game = core::Game::default();

  game.board_print();
  game.make_move( "a2a4" );
  game.board_print();
}
