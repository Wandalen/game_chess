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
    title : "Draw text".to_string(),
    width : DISPLAY_WIDTH,
    height : DISPLAY_HEIGHT,
    resizable : false,
    ..Default::default()
  })
  .add_plugins( DefaultPlugins )
  .add_startup_system( setup.system() )
  .run();
}

//

fn setup
(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
)
{
  let texture_handle = asset_server.load( "icons.png" );
  let texture_atlas = TextureAtlas::from_grid( texture_handle, Vec2::new( 150.0, 150.0 ), 6, 2 );
  let texture_atlas_handle = texture_atlases.add( texture_atlas );
  commands.spawn_bundle( OrthographicCameraBundle::new_2d() );
  commands.spawn_bundle( SpriteSheetBundle
  {
    sprite : TextureAtlasSprite
    {
      index : 2,
      ..Default::default()
    },
    texture_atlas : texture_atlas_handle,
    transform : Transform::from_scale( Vec3::splat( 2.0 ) ),
    ..Default::default()
  });
}
