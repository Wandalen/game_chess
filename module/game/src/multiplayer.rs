//!
//! Multiplayer
//!

use bevy::prelude::*;
use bevy_egui::
{
  egui,
  EguiContext
};
use game_chess_client::
{
  Client,
  ClientError
};
use crate::GameState;
use crate::common::Multiplayer;
use crate::multiplayer::loading::AsyncTask;

mod loading;

///
/// Component with a connection address
///

#[ derive( Component, Debug ) ]
pub struct Destination( String );

///
/// One-shot setup system
///

pub fn setup( mut commands : Commands )
{
  commands.spawn().insert( Destination( String::new() ) );
}

///
/// Connection menu
///

pub fn connect_menu
(
  mut commands : Commands,
  mut egui_context : ResMut< EguiContext >,
  mut query : Query< ( Entity, &mut AsyncTask< Result< Client, ClientError > > ) >,
  mut destination : Query< &mut Destination >,
  mut app_state : ResMut< State< GameState > >,
)
{
  let mut destination = destination.single_mut();

  if let Some( ( entity, mut async_task) ) = query.iter_mut().next()
  {
    if let Some( result ) = async_task.result()
    {
      match result
      {
        Ok( client ) =>
        {
          commands.insert_resource( client );
          app_state.replace( GameState::MultiplayerGame( Multiplayer::ListGames ) ).unwrap();
        },
        Err( err ) =>
        {
          handle_error( egui_context.ctx_mut(), err );
        },
      }

      commands.entity( entity ).remove::< AsyncTask< Result< Client, ClientError > > >();
    }
    else
    {
      display_connecting_interface( egui_context.ctx_mut(), &mut destination.0 );
    }

    return;
  }

  egui::CentralPanel::default().show( egui_context.ctx_mut(), | ui |
  {
    ui.vertical_centered( | ui |
    {
      ui.add
      (
        egui::TextEdit::singleline( &mut destination.0 )
          .hint_text( "Enter server address: " )
      );

      if ui.button( "Connect" ).clicked()
      {
        let destination = format!( "http://{}", destination.0 );
        let task = AsyncTask::spawn( Client::connect( destination ) );
        commands.spawn().insert( task );
      }

      ui.separator();
      if ui.button( "Back" ).clicked()
      {
        app_state.pop().unwrap();
      }
    });
  });
}

fn handle_error( _egui_ctx : &egui::Context, error : ClientError )
{
  // TODO: display error in gui.
  println!( "{:?}", error );
}

fn display_connecting_interface( egui_ctx : &egui::Context, destination : &mut String )
{
  egui::CentralPanel::default().show( egui_ctx, | ui |
  {
    ui.vertical_centered( | ui |
    {
      ui.add
      (
        egui::TextEdit::singleline( destination )
          .hint_text( "Enter server address: " )
          .interactive( false )
      );
      ui.add_enabled( false, egui::Button::new( "Connect" ) );

      ui.separator();
      ui.add_enabled( false, egui::Button::new( "Back" ) );
    });
  });
}
