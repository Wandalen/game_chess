#![warn(missing_docs)]

//! Sample shows how to add text to Bevy window.

use bevy::prelude::*;

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
  .add_startup_system( setup )
  .add_plugins( DefaultPlugins )
  .run();
}

//

fn setup( mut commands : Commands, asset_server : Res< AssetServer > )
{
  commands.spawn_bundle( Camera2dBundle::default() );
  // to show text on the screen the UiCameraBundle is required
  // commands.spawn_bundle( UiCameraBundle::default() );

  // the section with text
  let text_section = TextSection
  {
    value : "The text section".to_string(),
    // style of the text
    style : TextStyle
    {
      // user font, if it is not defined, the default font is used
      font : asset_server.load( "fonts/FiraSans-Bold.ttf" ),
      font_size : 40.0,
      color : Color::rgb( 1.0, 1.0, 1.0 ),
    },
  };

  commands.spawn_bundle( TextBundle
  {
    // style of bundle
    style : Style
    {
      // align to right side of the window
      align_self : AlignSelf::FlexEnd,
      // absolute position, fixed size
      position_type : PositionType::Absolute,
      // position relative to right side and bottom of window
      position : bevy::ui::UiRect
      {
        bottom : Val::Px( 150.0 ),
        right : Val::Px( 25.0 ),
        .. Default::default()
      },
      .. Default::default()
    },

    text : Text
    {
      // text values, can contain multiple text sections
      sections : vec![ text_section ],
      .. Default::default()
    },
    .. Default::default()
  });
}
