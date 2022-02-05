#![warn(missing_docs)]

//! Sample shows how to make sprite from part of original sprites image.

use bevy::prelude::*;
use bevy::render::pass::ClearColor;

const DISPLAY_HEIGHT : f32 = 300.0;
const DISPLAY_WIDTH : f32 = 300.0;

//

fn main()
{
  App::build()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_resource(WindowDescriptor {
      title : "Draw text".to_string(),
      width : DISPLAY_WIDTH,
      height : DISPLAY_HEIGHT,
      resizable : false,
      ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup.system())
    .run();
}

//

fn setup(mut commands : Commands, asset_server : Res<AssetServer>, mut texture_atlases : ResMut<Assets<TextureAtlas>>)
{
  // load sprite image
  let texture_handle = asset_server.load("icons.png");
  // split original image. Parameters : ( asset image, size for partial sprite, number of columns, number of lines )
  let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(150.0, 150.0), 6, 2);
  // handle sprites atlas as resource
  let texture_atlas_handle = texture_atlases.add(texture_atlas);
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  commands.spawn_bundle(SpriteSheetBundle {
    // choose the sprite
    sprite : TextureAtlasSprite {
      index : 2,
      ..Default::default()
    },
    // full sprites atlas
    texture_atlas : texture_atlas_handle,
    // resize original sprite to 200% ( 300px, 300px )
    transform : Transform::from_scale(Vec3::splat(2.0)),
    ..Default::default()
  });
}
