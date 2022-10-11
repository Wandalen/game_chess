//!
//! Game settings manipulation.
//!

use bevy::prelude::*;
use bevy_egui::
{
  egui,
  EguiContext,
};
use crate::GameState;

///
/// Game settings
///

#[ derive( Debug ) ]
pub struct Settings
{
  /// Color material handles
  pub color_handles : Materials,
  /// Should game play audio
  pub enable_sound : bool,
}

///
/// Color materials handles
///

#[ derive( Debug ) ]
pub struct Materials
{
  /// Black color
  pub black : Handle< ColorMaterial >,
  /// White color
  pub white : Handle< ColorMaterial >,
}

///
/// System that draws the settings menu.
///

pub fn settings_menu
(
  mut egui_context : ResMut< EguiContext >,
  mut keys : ResMut< Input< KeyCode > >,
  mut app_state : ResMut< State< GameState > >,
  mut materials : ResMut< Assets< ColorMaterial > >,
  mut settings : ResMut< Settings >,
)
{
  if keys.just_pressed( KeyCode::Escape )
  {
    app_state.pop().unwrap();
    // Not doing this can cause issues https://github.com/bevyengine/bevy/issues/1700.
    keys.reset( KeyCode::Escape );
  }

  egui::CentralPanel::default().show( egui_context.ctx_mut(), | ui |
    {
      ui.vertical_centered( |ui|
        {
          ui.heading( "\"White\" cells color" );
          let material = materials.get_mut( &settings.color_handles.white ).unwrap();
          let mut color_schema = [ material.color.r(), material.color.g(), material.color.b(), 1.0 ];
          if ui.color_edit_button_rgba_unmultiplied( &mut color_schema ).changed()
          {
            material.color = Color::rgb( color_schema[ 0 ],color_schema[ 1 ], color_schema[ 2 ] );
          }
          ui.heading( "\"Black\" cells color" );
          let material = materials.get_mut( &settings.color_handles.black ).unwrap();
          let mut color_schema = [ material.color.r(), material.color.g(), material.color.b(), 1.0 ];
          if ui.color_edit_button_rgba_unmultiplied( &mut color_schema ).changed()
          {
            material.color = Color::rgb( color_schema[ 0 ],color_schema[ 1 ], color_schema[ 2 ] );
          }

          ui.checkbox( &mut settings.enable_sound, "Enable sound" );

          if ui.button( "Back" ).clicked()
          {
            app_state.pop().unwrap();
          }
        });
    });
}

impl FromWorld for Settings
{
  fn from_world( world : &mut World ) -> Self
  {
    let mut materials = world.get_resource_mut::< Assets< ColorMaterial > >().unwrap();
    Settings
    {
      color_handles : Materials
      {
        black: materials.add( ColorMaterial::from( Color::rgb( 0.9, 0.9, 0.7 ) ) ),
        white: materials.add( ColorMaterial::from( Color::rgb( 0.2, 0.2, 0.1 ) ) ),
      },
      enable_sound : true,
    }
  }
}
