use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy::window::WindowResizeConstraints;
use bevy_egui::{ EguiPlugin };

mod draw;
use draw::*;

const DISPLAY_HEIGHT : f32 = 600.0;
const DISPLAY_WIDTH : f32 = 800.0;

//

fn main() {
  App::build()
  .insert_resource( ClearColor( Color::rgb( 0.0, 0.0, 0.0 ) ) )
  .insert_resource( WindowDescriptor
  {
    title : "Spawn board".to_string(),
    width : DISPLAY_WIDTH,
    height : DISPLAY_HEIGHT,
    resizable : true,
    resize_constraints : WindowResizeConstraints
    {
      min_width : DISPLAY_WIDTH,
      min_height : DISPLAY_HEIGHT,
      ..Default::default()
    },
    ..Default::default()
  })
  .insert_resource(BoardSegments::default())
  .add_startup_system( setup.system() )
  .add_startup_stage( "game_setup", SystemStage::single( spawn_board.system() ) )
  .add_system_set_to_stage
  (
    CoreStage::PostUpdate,
    SystemSet::new()
    .with_system( position_translation.system() )
    .with_system( size_scaling.system() ),
  )
  .add_plugins( DefaultPlugins )
  .add_plugin( EguiPlugin )
  .add_system( setup_egui.system() )
  .run();
}

