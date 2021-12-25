#![warn( missing_docs )]

//! Simpler drawing of chess board.

use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy::input::system::exit_on_esc_system;

///
/// Main.
///

fn main()
{
  App::build()
  .add_plugins( DefaultPlugins )
  .insert_resource( ClearColor( Color::rgb( 0.0, 0.0, 0.0 ) ) )
  .add_startup_system( board_setup.system() )
  .add_system( exit_on_esc_system.system() )
  .run();
}

///
/// Board as 64 sprites.
///

fn board_setup
(
  mut commands : Commands,
  mut materials : ResMut< Assets< ColorMaterial > >,
  windows : Res< Windows >,
)
{

  commands.spawn_bundle( OrthographicCameraBundle::new_2d() );

  let black = materials.add( Color::rgb( 0.30, 0.05, 0.0 ).into() );
  let white = materials.add( Color::rgb( 1.0, 1.0, 1.0 ).into() );

  let board_dim = ( 8, 8 );
  let board_margin = ( 1, 1 );
  let board_dim_f = ( board_dim.0 as f32, board_dim.1 as f32 );
  let board_margin_f = ( board_margin.0 as f32, board_margin.1 as f32 );

  let window = windows.get_primary().unwrap();
  let size_in_pixels = ( window.width(), window.height() );
  let side = if size_in_pixels.0 < size_in_pixels.1
  { size_in_pixels.0 / ( board_dim_f.0 + ( board_margin_f.0 )*2.0 ) }
  else
  { size_in_pixels.1 / ( board_dim_f.1 + ( board_margin_f.1 )*2.0 ) };

  for x in 0..board_dim.0
  {
    for y in 0..board_dim.1
    {
      let material = if ( x + y ) % 2 == 0
      { &white }
      else
      { &black };

      let transform = Transform
      {
        translation : Vec3::new
        (
          ( x as f32 ) * side - side * board_dim_f.0 / 2.0 + side / 2.0,
          ( y as f32 ) * side - side * board_dim_f.1 / 2.0 + side / 2.0,
          0.0,
        ),
        .. Default::default()
      };

      commands.spawn_bundle( SpriteBundle
      {
        material : material.clone(),
        sprite : Sprite::new( Vec2::new( side, side ) ),
        transform,
        .. Default::default()
      });
    }
  }
}
