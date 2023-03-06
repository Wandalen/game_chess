#![ warn( missing_docs ) ]

//! Simpler drawing of chess board.

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::close_on_esc;

///
/// Main.
///

fn main()
{
  App::new()
  .add_plugins( DefaultPlugins )
  .insert_resource( ClearColor( Color::rgb( 0.0, 0.0, 0.0 ) ) )
  .add_startup_system( board_setup )
  .add_system( close_on_esc )
  .run();
}

///
/// Board as 64 meshes.
///

fn board_setup
(
  mut commands : Commands,
  mut meshes : ResMut< Assets< Mesh > >,
  mut materials : ResMut< Assets< ColorMaterial > >,
  windows : Res< Windows >
)
{
  commands.spawn( Camera2dBundle::default() );

  let black = materials.add( Color::rgb( 0.30, 0.05, 0.0 ).into() );
  let white = materials.add( Color::rgb( 1.0, 1.0, 1.0 ).into() );

  let board_dim = ( 8, 8 );
  let board_margin = ( 1, 1 );
  let board_dim_f = ( board_dim.0 as f32, board_dim.1 as f32 );
  let board_margin_f = ( board_margin.0 as f32, board_margin.1 as f32 );

  let window = windows.primary();
  let size_in_pixels = ( window.width(), window.height() );
  let side = if size_in_pixels.0 < size_in_pixels.1
  {
    size_in_pixels.0 / ( board_dim_f.0 + ( board_margin_f.0 ) * 2.0 )
  }
  else
  {
    size_in_pixels.1 / ( board_dim_f.1 + ( board_margin_f.1 ) * 2.0 )
  };

  for x in 0 .. board_dim.0
  {
    for y in 0 .. board_dim.1
    {
      let material = if ( x + y ) % 2 == 0 { white.clone() } else { black.clone() };

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

      commands.spawn( MaterialMesh2dBundle
      {
        material,
        mesh : meshes.add( shape::Quad::new( Vec2::new( side, side ) ).into() ).into(),
        transform,
        ..Default::default()
      } );
    }
  }
}
