#![warn(missing_docs)]

//! Sample shows how start of mouse coordinates to change to center of the window.

use bevy::prelude::*;

const DISPLAY_HEIGHT : f32 = 300.0;
const DISPLAY_WIDTH : f32 = 300.0;

//

struct MainCamera;

fn main()
{
  App::build()
  .insert_resource( ClearColor( Color::rgb( 0.04, 0.04, 0.04 ) ) )
  .insert_resource( WindowDescriptor
  {
    title : "Draw text".to_string(),
    width : DISPLAY_WIDTH,
    height : DISPLAY_HEIGHT,
    resizable : false,
    ..Default::default()
  })
  .add_plugins( DefaultPlugins )
  .add_startup_system( setup.system() )
  .add_system_set_to_stage
  (
    CoreStage::PostUpdate,
    SystemSet::new()
    .with_system( cursor_system.system() )
  )
  .run();
}

//

fn setup( mut commands : Commands)
{
  commands.spawn()
  .insert_bundle( OrthographicCameraBundle::new_2d() )
  .insert( MainCamera );
}

//

fn cursor_system
(
  window : Res<Windows>,
  q_camera : Query<&Transform, With<MainCamera>>
)
{
  let primary_window = window.get_primary().unwrap();

  if let Some( pos ) = primary_window.cursor_position()
  {
    let size = Vec2::new( primary_window.width() as f32, primary_window.height() as f32 );

    // mouse coordinates related to center
    let p = pos - size / 2.0;

    let camera_transform = q_camera.single().unwrap();

    // compute the coordinates by multiplying the camera matrix on vector of mouse coordinates
    // related to center
    let pos_from_center = camera_transform.compute_matrix() * p.extend( 0.0 ).extend( 1.0 );
    eprintln!( "World coords: {}/{}", pos_from_center.x, pos_from_center.y );
  }
}
