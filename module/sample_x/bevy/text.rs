use bevy::prelude::*;
use bevy::render::pass::ClearColor;

const DISPLAY_HEIGHT : f32 = 300.0;
const DISPLAY_WIDTH : f32 = 300.0;

//

fn main()
{
  App::build()
  .insert_resource( ClearColor( Color::rgb( 0.04, 0.04, 0.04 ) ) )
  .insert_resource( WindowDescriptor
  {
    title: "Simple text".to_string(),
    width: DISPLAY_WIDTH,
    height: DISPLAY_HEIGHT,
    resizable: false,
    ..Default::default()
  })
  .add_startup_system( setup.system() )
  .add_plugins(DefaultPlugins)
  .run();
}

//

pub fn setup
(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
)
{
  commands.spawn_bundle( OrthographicCameraBundle::new_2d() );
  commands.spawn_bundle( UiCameraBundle::default() );

  commands.spawn_bundle( TextBundle
  {
    style: Style
    {
      align_self : AlignSelf::FlexEnd,
      position_type : PositionType::Absolute,
      position : Rect
      {
        bottom : Val::Px( 150.0 ),
        right : Val::Px( 25.0 ),
        ..Default::default()
      },
      ..Default::default()
    },

    text : Text
    {
      sections :
      vec![
        TextSection
        {
          value : "The text section".to_string(),
          style : TextStyle
          {
            font : asset_server.load( "fonts/FiraSans-Bold.ttf" ),
            font_size : 40.0,
            color : Color::rgb( 1.0, 1.0, 1.0 ),
          },
        },
      ],
      ..Default::default()
    },
    ..Default::default()
  });
}
