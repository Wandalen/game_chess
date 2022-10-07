//!
//! Main menu implementation.
//!

use bevy::prelude::*;
use bevy_egui::
{
  egui,
  EguiContext
};
use crate::GameState;

///
/// System that draws the main menu.
///

pub fn setup_main_menu
(
  mut egui_context : ResMut< EguiContext >,
  mut app_state : ResMut< State< GameState > >,
)
{
  egui::CentralPanel::default().show( egui_context.ctx_mut(), | ui |
  {
    ui.vertical_centered( |ui|
    {
      if ui.button( "New game with AI" ).clicked()
      {
        app_state.set( GameState::GameNew ).unwrap();
      }
      if ui.button( "New game" ).clicked()
      {
        // TODO: Implement new game.
      }
      if ui.button( "Multiplayer" ).clicked()
      {
        // TODO: Implement multiplayer.
      }
      if ui.button( "Options" ).clicked()
      {
        // TODO: Implement game settings.
      }
      if ui.button( "Quit" ).clicked()
      {
        // TODO: Implement game exit.
      }
    });
  });
}

