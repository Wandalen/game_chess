#![ warn( missing_docs ) ]

//! Sample shows how mouse coordinates translates to board coordinates.

use bevy::{ prelude::*, window::CursorMoved };

const DISPLAY_HEIGHT : f32 = 300.0;
const DISPLAY_WIDTH : f32 = 300.0;

const DESK_HEIGHT : u8 = 8;
const DESK_WIDTH : u8 = 8;

//

fn main()
{
  App::new()
  .insert_resource( ClearColor( Color::rgb( 0.04, 0.04, 0.04 ) ) )
  .insert_resource( WindowDescriptor
  {
    title : "Draw text".to_string(),
    width : DISPLAY_WIDTH,
    height : DISPLAY_HEIGHT,
    resizable : false,
    .. Default::default()
  })
  .add_plugins( DefaultPlugins )
  .add_startup_system( setup )
  .add_system_set_to_stage
  (
    CoreStage::PostUpdate,
    SystemSet::new().with_system( cursor_system )
  )
  .run();
}

//

fn setup( mut commands : Commands )
{
  commands.spawn()
  .insert_bundle( Camera2dBundle::default() );
}

//

fn cursor_system( window_resource : Res<Windows>, mut cursor_moved_events : EventReader< CursorMoved > )
{
  let window = window_resource.get_primary().unwrap();
  let width = window.width();
  let height = window.height();

  // number of pixels per square
  let x_multiplier = width / DESK_WIDTH as f32;
  let y_multiplier = height / DESK_HEIGHT as f32;

  if let Some( _position ) = window.cursor_position()
  {
    for event in cursor_moved_events.iter()
    {
      // translate current position to position of square
      let x = ( event.position[ 0 ] / x_multiplier ) as u8;
      let y = ( event.position[ 1 ] / y_multiplier ) as u8;
      eprintln!( "Desk coords : x::{} y::{}", x, y );
    }
  }
}
