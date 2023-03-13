//!
//! Main menu implementation.
//!

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::
{
  egui,
  EguiContext
};
use crate::common::Multiplayer;
use crate::GameState;
use bevy::prelude::Resource;

///
/// System that draws the main menu.
///

#[ derive( Resource ) ]
struct ResEguiContext ( EguiContext );

pub fn setup_main_menu
(
  mut egui_context : ResMut< ResEguiContext >,
<<<<<<< HEAD
  mut app_state : ResMut< NextState< GameState > >,
=======
  mut app_state : ResMut< State< GameState > >,
>>>>>>> 1316e2094a4d24e4a8ca2babef6eaf416851e451
  mut exit : EventWriter< AppExit >,
)
{
  egui::CentralPanel::default().show( egui_context.ctx_mut(), | ui |
  {
    ui.vertical_centered( |ui|
    {
      if ui.button( "New game with AI" ).clicked()
      {
        app_state.set( GameState::GameNew );
      }
      if ui.button( "New game" ).clicked()
      {
        // TODO: Implement new game.
      }
      if ui.button( "Multiplayer" ).clicked()
      {
        app_state.push( GameState::MultiplayerGame( Multiplayer::ConnectingToServer ) ).unwrap();
      }
      if ui.button( "Options" ).clicked()
      {
        app_state.push( GameState::Settings ).unwrap();
      }
      if ui.button( "Quit" ).clicked()
      {
        exit.send( AppExit );
      }
    });
  });
}

