#![warn(missing_docs)]

//! Sample shows how to detect mouse events on the window

use bevy::
{
  input::mouse::{ MouseButtonInput, MouseMotion, MouseWheel },
  prelude::*,
  window::CursorMoved,
};

//

fn main()
{
  App::build()
  .add_plugins( DefaultPlugins )
  .add_system( print_mouse_events.system() )
  .run();
}

//

fn print_mouse_events
(
  mut mouse_button_input_events : EventReader<MouseButtonInput>,
  mut mouse_motion_events : EventReader<MouseMotion>,
  mut cursor_moved_events : EventReader<CursorMoved>,
  mut mouse_wheel_events : EventReader<MouseWheel>,
)
{
  // detect mouse buttons events
  for event in mouse_button_input_events.iter()
  {
    info!( "{:?}", event );
  }

  // detect mouse motion
  for event in mouse_motion_events.iter()
  {
    info!( "{:?}", event );
  }

  // detect cursor motion
  for event in cursor_moved_events.iter()
  {
    info!( "{:?}", event );
    info!( "x : {}, y : {}", event.position[ 0 ], event.position[ 1 ] );
  }

  // detect mouse wheel events
  for event in mouse_wheel_events.iter()
  {
    info!( "{:?}", event );
  }
}
