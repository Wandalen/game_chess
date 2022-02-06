#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

//!
//! Chess game implemented on Bevy for educational purpose.
//!

use bevy::render::RenderSystem;
use bevy::render::camera::camera_system;
use game_chess_core as core;
use bevy::prelude::*;
use bevy::input::system::exit_on_esc_system;

pub mod camera;

///
/// Graphics setup.
///

pub fn graphics_setup(mut commands : Commands, mut materials : ResMut<Assets<ColorMaterial>>)
{
  /* camera */
  commands.spawn_bundle(camera::ChessCameraBundle::new());

  let size_in_cells = (8, 8);

  let white = materials.add(ColorMaterial::color(Color::rgb(0.9, 0.9, 0.7)));
  let black = materials.add(ColorMaterial::color(Color::rgb(0.2, 0.2, 0.1)));

  let size = 2.0 / 8.0;
  let delta = 1.0 - size / 2.0;

  for x in 0 .. size_in_cells.0
  {
    for y in 0 .. size_in_cells.1
    {
      let material = if (x + y) % 2 == 0 { black.clone() } else { white.clone() };

      let sprite = Sprite {
        size : Vec2::new(size, size),
        ..Default::default()
      };

      let transform = Transform {
        translation : Vec3::new((x as f32) * size - delta, (y as f32) * size - delta, 0.0),
        ..Default::default()
      };

      commands.spawn_bundle(SpriteBundle {
        sprite,
        material,
        transform,
        ..Default::default()
      });
    }
  }
}

///
/// Startup system for the game.
///

pub fn core_setup()
{
  let mut game = core::Game::default();
  game.board_print();
  game.make_move("a2a4".into());
  game.board_print();
}

///
/// Main
///

fn main()
{
  let mut app = App::build();
  /* default plugins */
  app.add_plugins(DefaultPlugins);
  /* background */
  app.insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)));
  /* setup core */
  app.add_startup_system(core_setup.system());
  /* setup graphics */
  app.add_startup_system(graphics_setup.system());
  /* escape on exit */
  app.add_system(exit_on_esc_system.system());
  app.add_system_to_stage(
    CoreStage::PostUpdate,
    camera_system::<camera::ChessProjection>
      .system()
      .before(RenderSystem::VisibleEntities),
  );
  /* for web target */
  #[cfg(target_arch = "wasm32")]
  app.add_plugin(bevy_webgl2::WebGL2Plugin);
  /* run */
  app.run();
}
