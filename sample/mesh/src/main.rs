#![ warn( missing_docs ) ]

//! Sample shows how to add mesh in Bevy.

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

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

fn setup( mut commands : Commands, mut meshes : ResMut< Assets< Mesh > >, mut materials : ResMut< Assets< ColorMaterial > > )
{
  commands.spawn_bundle( Camera2dBundle::default() );
  // adding mesh
  commands.spawn_bundle( MaterialMesh2dBundle
  {
    mesh : meshes.add( shape::Quad::new( Vec2::new( 100.0, 100.0 ) ).into() ).into(),
    material : materials.add( ColorMaterial::from( Color::rgb( 0.0, 0.0, 0.0 ) ) ),
    transform : Transform::from_translation( Vec3::new( 0.0, 0.0, 0.0 ) ),
    .. Default::default()
  } );
}
