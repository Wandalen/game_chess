#![warn(missing_docs)]

use bevy::prelude::*;
use bevy_egui::{ egui, EguiContext };

const DESK_HEIGHT: u8 = 8;
const DESK_WIDTH: u8 = 8;

const SIDE_PANEL_WIDTH : f32 = 300.0;

//

#[ derive( Clone, Eq, PartialEq ) ]
pub enum Side
{
  Black,
  White,
}

#[ derive( Clone, Eq, PartialEq ) ]
pub struct CurrentSide( Side );

//



//

pub fn setup_egui( egui_context : Res<EguiContext>, mut side : ResMut<CurrentSide> )
{
  // add fixated panel
  egui::SidePanel::left( "Menu" )
  .resizable( false )
  .default_width( SIDE_PANEL_WIDTH )
  .show( egui_context.ctx(), | ui |
  {
    let choose = match &side.0
    {
        Side::White => "White",
        Side::Black => "Black"
    };

    // add combobox
    egui::ComboBox::from_label( "Select side!" )
    .selected_text( choose )
    .show_ui(ui, | ui |
    {
      ui.selectable_value( &mut side.0, Side::White, "White" );
      ui.selectable_value( &mut side.0, Side::Black, "Black" );
    })
  });
}

//

