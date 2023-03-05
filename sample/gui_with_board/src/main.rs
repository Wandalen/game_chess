#![warn(missing_docs)]

//! The sample which draw a chess board and GUI side panel with combobox.

use bevy::prelude::*;
use bevy::window::WindowResizeConstraints;
use bevy_egui::{ EguiPlugin };

mod draw;
use draw::*;

const DISPLAY_HEIGHT : f32 = 600.0;
const DISPLAY_WIDTH : f32 = 800.0;

// 
#[ derive( Resource ) ]
struct WinDescr ( WindowDescriptor );

//

fn main()
{
  App::new()
  .insert_resource( ClearColor( Color::rgb( 0.0, 0.0, 0.0 ) ) )
  .insert_resource( WinDescr ( WindowDescriptor
  {
    title : "Spawn board".to_string(),
    width : DISPLAY_WIDTH,
    height : DISPLAY_HEIGHT,
    resizable : true,
    resize_constraints : WindowResizeConstraints
    {
      min_width : DISPLAY_WIDTH,
      min_height : DISPLAY_HEIGHT,
      .. Default::default()
    },
    .. Default::default()
  } ) )
  .insert_resource( BoardSegments::default() )
  .add_startup_system( setup )
  .add_startup_stage( "game_setup", SystemStage::single( spawn_board ) )
  .add_system_set_to_stage
  (
    CoreStage::PostUpdate,
    SystemSet::new()
    .with_system( position_translation )
    .with_system( size_scaling ),
  )
  .add_plugins( DefaultPlugins )
  .add_plugin( EguiPlugin )
  .add_system( setup_egui )
  .run();
}
