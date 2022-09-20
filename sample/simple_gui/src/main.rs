#![warn(missing_docs)]

//! Sample shows how to use Egui into Bevy window.

use bevy::prelude::*;
use bevy_egui::{ egui, EguiContext, EguiPlugin };

const DISPLAY_HEIGHT : f32 = 300.0;
const DISPLAY_WIDTH : f32 = 300.0;

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
  // add Egui plugin
  .add_plugin( EguiPlugin )
  .add_system( setup )
  .run();
}

//

fn setup( mut egui_context : ResMut< EguiContext > )
{
  // initialize Egui window
  egui::Window::new( "Menu" )
  .show( egui_context.ctx_mut(), | ui |
  {
    // add labels inside Egui window
    ui.label( "Game" );
    ui.label( "Options" );
  });
}
