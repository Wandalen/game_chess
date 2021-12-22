#![warn(missing_docs)]

use bevy::prelude::*;

fn main()
{
  let mut app = App::build()
     .add_plugins( DefaultPlugins )
     .insert_resource( ClearColor( Color::rgb( 0.9, 0.9, 0.9 ) ) )
     .add_startup_system( setup.system() )
     .run();
}

fn setup
(
  mut commands: Commands,
  asset_server: Res< AssetServer >,
  mut materials: ResMut<Assets< ColorMaterial > >,
)
{
    let texture_handle = asset_server.load( "icon.png" );
    commands.spawn_bundle( OrthographicCameraBundle::new_2d() );
    let sprite = SpriteBundle
    {
      material: materials.add(texture_handle.into()),
      ..Default::default()
    };
    commands.spawn_bundle( sprite );
}
