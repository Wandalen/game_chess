#![ warn( missing_docs ) ]

//! Sample shows how to add sprite in Bevy.

use bevy::prelude::*;

const DISPLAY_HEIGHT : f32 = 300.0;
const DISPLAY_WIDTH : f32 = 300.0;

//

fn main()
{
  App::new()
  .insert_resource( ClearColor( Color::rgb( 0.9, 0.9, 0.9 ) ) )
  .insert_resource( WindowDescriptor
  {
    title : "Draw sprite".to_string(),
    width : DISPLAY_WIDTH,
    height : DISPLAY_HEIGHT,
    resizable : false,
    .. Default::default()
  })
  .add_plugins( DefaultPlugins )
  .add_startup_system( setup )
  .run();
}

//

fn setup( mut commands : Commands, asset_server : Res< AssetServer > )
{
  commands.spawn_bundle( Camera2dBundle::default() );
  // adding sprite
  commands.spawn_bundle( SpriteBundle
  {
    texture : asset_server.load( "icon.png" ),
    .. Default::default()
  });
}
