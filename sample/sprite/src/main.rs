#![warn(missing_docs)]

//! Sample shows how to add sprite in Bevy.

use bevy::prelude::*;
use bevy::render::pass::ClearColor;

const DISPLAY_HEIGHT : f32 = 300.0;
const DISPLAY_WIDTH : f32 = 300.0;

//

fn main()
{
  App::build()
  .insert_resource( ClearColor( Color::rgb( 0.9, 0.9, 0.9 ) ) )
  .insert_resource( WindowDescriptor
  {
    title : "Draw sprite".to_string(),
    width : DISPLAY_WIDTH,
    height : DISPLAY_HEIGHT,
    resizable : false,
    ..Default::default()
  })
  .add_plugins( DefaultPlugins )
  .add_startup_system( setup.system() )
  .run();
}

//

fn setup
(
  mut commands : Commands,
  asset_server : Res<AssetServer>,
  mut materials : ResMut<Assets<ColorMaterial>>,
)
{
  // loading sprite from image
  let texture_handle = asset_server.load( "icon.png" );
  commands.spawn_bundle( OrthographicCameraBundle::new_2d() );
  // adding sprite
  commands.spawn_bundle( SpriteBundle
  {
    material: materials.add( texture_handle.into() ),
    ..Default::default()
  });
}
