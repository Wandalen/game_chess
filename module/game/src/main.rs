#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

//!
//! Chess game implemented on Bevy for educational purpose.
//!

use std::process::exit;
use bevy::render::RenderSystem;
use bevy::render::camera::camera_system;
use game_chess_core as core;
use bevy::prelude::*;
use bevy::input::system::exit_on_esc_system;
use bevy_egui::{egui, EguiContext, EguiPlugin};

pub mod camera;
pub mod piece;

///
/// Board setup.
///

pub fn board_setup(mut commands : Commands, mut materials : ResMut<Assets<ColorMaterial>>)
{
  /* camera */
  commands
    .spawn_bundle(camera::ChessCameraBundle::new())
    .insert(Timer::from_seconds(2.0, false));


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

  // diagnostics_rect( &mut commands, &mut materials );
}

///
/// Add sprite of size 2x2 for diagnostics purpose. The sprite should cover central zone of window.
///

pub fn diagnostics_rect(commands : &mut Commands, materials : &mut ResMut<Assets<ColorMaterial>>)
{
  let red = materials.add(ColorMaterial::color(Color::rgb(0.9, 0.2, 0.2)));

  let sprite = Sprite {
    size : Vec2::new(2., 2.),
    ..Default::default()
  };

  let transform = Transform {
    translation : Vec3::new(0.0, 0.0, 0.0),
    ..Default::default()
  };

  commands.spawn_bundle(SpriteBundle {
    sprite,
    material : red,
    transform,
    ..Default::default()
  });
}

///
/// Startup system for the game.
///

pub fn core_setup(mut commands : Commands, mut game_state : ResMut<State<GameState>>)
{
  let mut game = core::Game::default();
  game.board_print();
  game.make_move("a2a4".into());
  game.board_print();
  commands.insert_resource(game);

  game_state.set(GameState::GameStart).unwrap();
}

pub fn menu_setup(mut commands : Commands, mut game_state : ResMut<State<GameState>>, egui_context : ResMut<EguiContext>)
{
  egui::Window::new("Menu").show(egui_context.ctx(), |ui| {
    let start_button = ui.button("Start Game");
    let mut exit_button = ui.button("Exit");

    if start_button.clicked()
    {
      game_state.set(GameState::GameNew);
    }
    if exit_button.clicked()
    {
      exit(0);
    }

  });
}

///
/// Game state enum
///

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState
{
  /// Intial state
  Init,
  /// When we create a new game
  GameNew,
  /// When we start a new game
  GameStart,
}

fn timer_system(time : Res<Time>, mut query : Query<&mut Timer>, mut game_state : ResMut<State<GameState>>)
{
  let mut timer = query.single_mut().unwrap();
  timer.tick(time.delta());
  if timer.finished()
  {
    game_state.set(GameState::GameNew).unwrap();
  }
}

///
/// Main
///

fn main()
{
  let mut app = App::build();
  /* default plugins */
  app.add_plugins(DefaultPlugins);
  app.add_plugin(EguiPlugin);
  /* background */
  app.insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)));
  app.add_state(GameState::Init);

  /* setup core */
  app.add_system_set(SystemSet::on_update(GameState::Init).with_system(menu_setup.system()));
  app.add_system_set(SystemSet::on_update(GameState::GameNew).with_system(core_setup.system()));
  app.add_system_set(SystemSet::on_update(GameState::GameStart).with_system(piece::pieces_setup.system()));
  /* setup board */
  app.add_startup_system(board_setup.system());

  // app.add_system(timer_system.system());
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
