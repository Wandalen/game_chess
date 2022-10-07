//!
//! Pause menu.
//!

use bevy::prelude::*;
use bevy_egui::
{
  egui,
  EguiContext
};
use crate::GameState;

///
/// System that draws the pause menu.
///

pub fn setup_pause_menu
(
  mut egui_context : ResMut< EguiContext >,
  mut app_state : ResMut< State< GameState > >,
)
{
  egui::CentralPanel::default().show( egui_context.ctx_mut(), | ui |
  {
    ui.vertical_centered( | ui |
    {
      if ui.button( "Resume" ).clicked()
      {
        app_state.set( GameState::GameStart ).unwrap();
      }
      if ui.button( "Exit" ).clicked()
      {
        app_state.set( GameState::MainMenu ).unwrap();
      }
    });
  });
}
