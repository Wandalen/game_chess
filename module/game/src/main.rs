#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

//!
//! Chess game implemented on Bevy for educational purpose.
//!

use bevy::math::Vec4Swizzles;
use bevy::render::RenderSystem;
use bevy::render::camera::{camera_system, Camera};
use game_chess_core as core;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_egui::{EguiPlugin};
use bevy::input::system::exit_on_esc_system;
#[cfg(not(target_arch = "wasm32"))]
use bevy::audio::AudioPlugin;

pub mod camera;
pub mod common;
#[cfg(not(target_arch = "wasm32"))]
pub mod highlight;
pub mod piece;

use common::GameState;

///
/// Color materials handles
///

#[derive(Debug)]
pub struct Materials
{
  /// Black color
  pub black : Handle<ColorMaterial>,
  /// White color
  pub white : Handle<ColorMaterial>,
}

// /// mut material  359
// /// My color change
// ///
//
// pub fn color_change
// (
//   mut materials: ResMut<Assets<ColorMaterial>>,
//    query_white: Query<&Handle<ColorMaterial>, With<CellWhite>>,
//    query_black: Query<&Handle<ColorMaterial>, With<CellBlack>>,
//     color_schema: Res<CellColorSchema>)
//     {
//
//   for handle in query_white.iter() {
//     let mut material = materials.get_mut(handle).unwrap();
//     material.color = Color::rgb(color_schema.white[0],color_schema.white[1], color_schema.white[2]);
//   }
//
//   for handle in query_black.iter() {
//     let mut material = materials.get_mut(handle).unwrap();
//     material.color = Color::rgb(color_schema.black[0],color_schema.black[1], color_schema.black[2]);
//   }
//   /*commands.insert_resource(Materials {
//   _white : materials.add(ColorMaterial::color(Color::rgb(0.9, 0.0, 0.0)));
//   _black : materials.add(ColorMaterial::color(Color::rgb(0.2, 0.8, 0.8)));
//   });*/
// }

///
/// Setup camera and add resources
///

pub fn setup(mut commands : Commands, mut materials : ResMut<Assets<ColorMaterial>>)
{
  commands
    .spawn_bundle(camera::ChessCameraBundle::new())
    .insert(bevy_interact_2d::InteractionSource::default())
    .insert(Timer::from_seconds(2.0, false));
  commands.insert_resource(Materials {
    white : materials.add(ColorMaterial::color(Color::rgb(0.9, 0.9, 0.7))),
    black : materials.add(ColorMaterial::color(Color::rgb(0.2, 0.2, 0.1))),
  });
}
///
/// Board setup.
///

pub fn board_setup(mut commands : Commands, materials : Res<Materials>)
{
  /* camera */
  // commands
  //   .spawn_bundle(camera::ChessCameraBundle::new())
  //   .insert(bevy_interact_2d::InteractionSource::default())
  //   .insert(Timer::from_seconds(2.0, false));

  let size_in_cells = (8, 8);

  // let white = materials.add(ColorMaterial::color(Color::rgb(0.9, 0.9, 0.7)));
  // let black = materials.add(ColorMaterial::color(Color::rgb(0.2, 0.2, 0.1)));

  let size = 2.0 / 8.0;
  let delta = 1.0 - size / 2.0;

  for x in 0 .. size_in_cells.0
  {
    for y in 0 .. size_in_cells.1
    {
      let is_black = (x + y) % 2 == 0;
      let material = if is_black
      {
        materials.black.clone()
      }
      else
      {
        materials.white.clone()
      };

      let sprite = Sprite {
        size : Vec2::new(size, size),
        ..Default::default()
      };

      let transform = Transform {
        translation : Vec3::new((x as f32) * size - delta, (y as f32) * size - delta, 0.0),
        ..Default::default()
      };

      // let mut cell = commands.spawn_bundle(SpriteBundle {
      commands.spawn_bundle(SpriteBundle {
        sprite,
        material,
        transform,
        ..Default::default()
      }); //.insert(Cell)

      // cell.insert(Cell);
      // if is_black
      // {
      //   cell.insert(CellBlack);
      // }
      // else
      // {
      //   cell.insert(CellWhite);
      // }
    }
  }

  // diagnostics_rect( &mut commands, &mut materials );
}


///
/// Convert cursor position to cell number
/// If cursor is outside of the board, may return values below zero or above 7
///

pub fn cursor_to_cell(cursor_pos : Vec2, window_size : Vec2, projection_matrix : Mat4) -> Vec2
{
  let clip_pos = (cursor_pos / (window_size / 2.0)) - Vec2::splat(1.0);
  let clip_pos_4 = clip_pos.extend(0.0).extend(1.0);
  let world_pos_4 = projection_matrix.inverse() * clip_pos_4;

  let world_pos = world_pos_4.xy() / world_pos_4.w;
  ((world_pos + Vec2::splat(1.0)) * 4.0).floor()
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

fn timer_system(time : Res<Time>, mut query : Query<&mut Timer>, mut game_state : ResMut<State<GameState>>)
{
  let mut timer = query.single_mut().unwrap();
  timer.tick(time.delta());
  if timer.finished()
  {
    game_state.set(GameState::GameNew).unwrap();
  }
}
//Sounds
#[cfg(not(target_arch = "wasm32"))]
fn loss(asset_server : Res<AssetServer>, audio_output : Res<Audio>)
{
  let music = asset_server.load("sound/horror.mp3");
  audio_output.play(music);
}
#[allow(dead_code)]
#[cfg(not(target_arch = "wasm32"))]
fn win(asset_server : Res<AssetServer>, audio_output : Res<Audio>)
{
  let music = asset_server.load("sound/Windless Slopes.ogg");
  audio_output.play(music);
}
#[allow(dead_code)]
#[cfg(not(target_arch = "wasm32"))]
fn draw(asset_server : Res<AssetServer>, audio_output : Res<Audio>)
{
  let music = asset_server.load("sound/sad_trombone.mp3");
  audio_output.play(music);
}
#[allow(dead_code)]
#[cfg(not(target_arch = "wasm32"))]
fn movement(asset_server : Res<AssetServer>, audio_output : Res<Audio>)
{
  let music = asset_server.load("sound/hit.mp3");
  audio_output.play(music);
}

///
/// Timer setup
///

pub fn egui_setup(
  egui_context : Res<EguiContext>,
  mut materials : ResMut<Assets<ColorMaterial>>,
  materials_handles : Res<Materials>,
)
{
  egui::Window::new("Timer").show(egui_context.ctx(), |ui| {
    // add labels inside Egui window
    ui.label("Time: 00:00.00");
  });

  egui::SidePanel::left("Menu")
    .resizable(false)
    //.default_width(SIDE_PANEL_WIDTH)
    .show(egui_context.ctx(), |ui|
    {
      ui.heading("\"White\" cells color");
      let material = materials.get_mut(&materials_handles.white).unwrap();
      let mut color_schema = [ material.color.r(), material.color.g(), material.color.b(), 1.0 ];
      ui.horizontal(|ui|{
        if ui.color_edit_button_rgba_unmultiplied(&mut color_schema).changed() {
          material.color = Color::rgb(color_schema[0],color_schema[1], color_schema[2]);
        }
      });
    });
}

///
/// System that highlights cells under the cursor
///

#[cfg(not(target_arch = "wasm32"))]
fn highlight_under_cursor
(
  windows : Res<Windows>,
  interaction : Res<bevy_interact_2d::InteractionState>,
  q_camera : Query<&Camera>,
  mut highlight : ResMut<highlight::Highlight>,
)
{
  //highlight.highlight((1, 2), Color::rgba(1.0, 1.0, 1.0, 0.3));

  let window = windows.get_primary().unwrap();
  let window_size = Vec2::new(window.width(), window.height());

  let camera = q_camera.single().unwrap();
  let cell = cursor_to_cell(interaction.last_cursor_position, window_size, camera.projection_matrix);

  if cell.x < 8.0 && cell.y < 8.0 && cell.x >= 0.0 && cell.y >= 0.0
  {
    highlight.highlight((cell.x as u8, cell.y as u8), Color::rgba(1.0, 1.0, 1.0, 0.2));
  }
}

//
// /// my struct
//
// pub fn setup_egui(egui_context : Res<EguiContext>, mut color_schema: ResMut<CellColorSchema>)
// {
//   // add fixated panel
//   egui::SidePanel::left("Menu")
//     .resizable(false)
//     //.default_width(SIDE_PANEL_WIDTH)
//     .show(egui_context.ctx(), |ui| {
//       ui.heading("1");
//       ui.horizontal(|ui|{
//         //let mut color_white = [0.,0.,0.,0.];
//         if ui.color_edit_button_rgba_unmultiplied(&mut color_schema.white).changed() {
//           //dbg!(color_white);
//         }
//       });
//     });
// }

// ///
// /// Mark cells
// ///
//
// #[derive(Debug)]
// pub struct Cell;
//
// ///
// /// Mark white cells
// ///
//
// #[derive(Debug)]
// pub struct CellWhite;
//
// ///
// /// Mark black cells
// ///
//
// #[derive(Debug)]
// pub struct CellBlack;
//
// ///
// /// Game color schema
// ///
//
// #[derive(Debug)]
// pub struct CellColorSchema
// {
//   /// White color
//   pub white : [f32; 4],
//   /// Black color
//   pub black : [f32; 4],
// }
//
// impl Default for CellColorSchema
// {
//   fn default() -> Self
//   {
//     Self {
//       white : [0.98, 0.94, 1.0, 1.],
//       black : [0.0, 0.2, 0.2, 1.],
//     }
//   }
// }

fn main()
{
  let mut app = App::build();
  /* default plugins */
  app.add_plugins(DefaultPlugins);

  /* background */
  app.insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)));
  // app.insert_resource(CellColorSchema::default());
  /* timer gui */
  app.insert_resource(WindowDescriptor {
    title : "Timer GUI".to_string(),
    width : 100.,
    height : 20.,
    resizable : true,
    ..Default::default()
  });
  app.add_plugin(EguiPlugin);
  app.add_system(egui_setup.system());
  app.add_state(GameState::Init);
  /* timer */
  app.add_system_set(SystemSet::on_update(GameState::Init).with_system(timer_system.system()));
  /* setup core */
  app.add_system_set(SystemSet::on_update(GameState::GameNew).with_system(core_setup.system()));
  app.add_system_set(SystemSet::on_update(GameState::GameStart).with_system(piece::pieces_setup.system()));
  /* setup board */
  app.add_startup_system(setup.system());
  app.add_startup_stage("board_setup", SystemStage::single(board_setup.system()));

  /* sound */

  #[cfg(not(target_arch = "wasm32"))]
  app.add_plugin(AudioPlugin);
  #[cfg(not(target_arch = "wasm32"))]
  app.add_startup_stage("loss", SystemStage::single(loss.system()));

  #[cfg(not(target_arch = "wasm32"))]
  app.add_plugin(bevy_interact_2d::InteractionPlugin);

  /* highlighting */
  #[cfg(not(target_arch = "wasm32"))]
  app.add_system(highlight_under_cursor.system());
  #[cfg(not(target_arch = "wasm32"))]
  app.add_plugin(highlight::HighlightPlugin {
    clear_on_each_frame : true,
  });

  /* escape on exit */
  app.add_system(exit_on_esc_system.system());

  // app.add_system(color_change.system());

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
