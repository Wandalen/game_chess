#![warn( missing_docs )]
#![ warn( missing_debug_implementations ) ]

//!
//! Chess game implemented on Bevy for educational purpose.
//!

use game_chess_core as core;
use bevy::prelude::*;
use bevy::input::system::exit_on_esc_system;

///
/// Main.
///

pub fn main()
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

pub fn graphics_setup
(
  mut commands : Commands,
  windows : Res< Windows >,
  mut materials : ResMut< Assets< ColorMaterial > >,
)
{
  /* camera */
  commands.spawn_bundle( OrthographicCameraBundle::new_2d() );

  let window = windows.get_primary().unwrap();
  let size_in_pixels = ( window.width(), window.height() );
  let size_in_cells = ( 8, 8 );
  let size_in_cells_f = ( size_in_cells.0 as f32, size_in_cells.1 as f32 );
  let side_size = if size_in_pixels.0 < size_in_pixels.1
  {
    size_in_pixels.0 / size_in_cells_f.0
  }
  else
  {
    size_in_pixels.1 / size_in_cells_f.1
  };

  let white = materials.add( ColorMaterial::color( Color::rgb( 0.9, 0.9, 0.7 ) ) );
  let black = materials.add( ColorMaterial::color( Color::rgb( 0.2, 0.2, 0.1 ) ) );

  for x in 0 .. size_in_cells.0
  {
    for y in 0 .. size_in_cells.1
    {

      let material = if ( x + y ) % 2 == 0
      {
        white.clone()
      }
      else
      {
        black.clone()
      };

      let sprite = Sprite
      {
        size : Vec2::new( side_size, side_size ),
        .. Default::default()
      };

      let transform = Transform
      {
        translation : Vec3::new
        (
          ( x as f32 - size_in_cells_f.0 / 2. + 0.5 ) * side_size,
          ( y as f32 - size_in_cells_f.0 / 2. + 0.5 ) * side_size,
          0.
        ),
        .. Default::default()
      };

      commands.spawn_bundle( SpriteBundle
      {
        sprite,
        material,
        transform,
        .. Default::default()
      });

    }
  }

}

///
/// Startup system for the game.
///

pub fn core_setup()
{
  let mut game = core::Game::default();
  game.board_print();
  game.make_move( "a2a4" );
  game.board_print();
}
