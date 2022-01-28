#![warn( missing_docs )]

//!
//! Chess game implemented on Bevy for educational purpose.
//!

use bevy::render::RenderSystem;
use bevy::render::camera::Camera;
use bevy::render::camera::CameraProjection;
use bevy::render::camera::DepthCalculation;
use bevy::render::camera::VisibleEntities;
use bevy::render::camera::camera_system;
use bevy::render::render_graph::base::camera::CAMERA_2D;
use game_chess_core as core;
use bevy::prelude::*;
use bevy::input::system::exit_on_esc_system;

///
/// Main.
///


#[derive(Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct ChessProjection
{
  pub left: f32,
  pub right: f32,
  pub bottom: f32,
  pub top: f32,
  pub near: f32,
  pub far: f32,
}

impl CameraProjection for ChessProjection {
  fn get_projection_matrix(&self) -> Mat4 {
      Mat4::orthographic_rh(
          self.left,
          self.right,
          self.bottom,
          self.top,
          self.near,
          self.far,
      )
  }

  fn update(&mut self, width: f32, height: f32) {
          if width> height
          {
              let aspect_ratio = width / height;
              self.left = (7.0 - 8.0*aspect_ratio)/2.0;
              self.right = (7.0 + 8.0*aspect_ratio)/2.0;
              self.top = 7.5;
              self.bottom = -0.5;
          } else {
              let aspect_ratio = height / width;
              self.left = -0.5;
              self.right = 7.5;
              self.top = (7.0 + 8.0*aspect_ratio)/2.0;
              self.bottom = (7.0 - 8.0*aspect_ratio)/2.0;
          }
  }

  fn depth_calculation(&self) -> DepthCalculation {
    DepthCalculation::Distance
  }
}

impl Default for ChessProjection {
  fn default() -> Self {
    ChessProjection {
          left: -1.0,
          right: 1.0,
          bottom: -1.0,
          top: 1.0,
          near: 0.0,
          far: 1000.0,
      }
  }
}

#[derive(Bundle)]
pub struct ChessCameraBundle
{
    pub camera: Camera,
    pub chess_projection: ChessProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl ChessCameraBundle {
  pub fn new() -> Self {
      let far = 1000.0;
      ChessCameraBundle {
          camera: Camera {
              name: Some(CAMERA_2D.to_string()),
              ..Default::default()
          },
          chess_projection: ChessProjection {
              far,
              ..Default::default()
          },
          visible_entities: Default::default(),
          transform: Transform::from_xyz(0.0, 0.0, far - 0.1),
          global_transform: Default::default(),
      }
  }
}

fn main()
{
  let mut app = App::build();
  /* default plugins */
  app.add_plugins( DefaultPlugins );
  /* background */
  app.insert_resource( ClearColor( Color::rgb( 0.9, 0.9, 0.9 ) ) );
  /* setup core */
  app.add_startup_system( core_setup.system() );
  /* setup graphics */
  app.add_startup_system( graphics_setup.system() );
  /* escape on exit */
  app.add_system( exit_on_esc_system.system() );
  app.add_system_to_stage(
    CoreStage::PostUpdate,
    camera_system::<ChessProjection>.system()
    .before(RenderSystem::VisibleEntities));
  /* for web target */
  #[cfg(target_arch = "wasm32")]
  app.add_plugin( bevy_webgl2::WebGL2Plugin );
  /* run */
  app.run();
}

///
/// Graphics setup.
///

fn graphics_setup
(
  mut commands : Commands,
  mut materials : ResMut< Assets< ColorMaterial > >,
)
{
  /* camera */
  commands.spawn_bundle( ChessCameraBundle::new() );

  let size_in_cells = ( 8, 8 );

  let white = materials.add( ColorMaterial::color( Color::rgb( 0.9, 0.9, 0.7 ) ) );
  let black = materials.add( ColorMaterial::color( Color::rgb( 0.2, 0.2, 0.1 ) ) );

  for x in 0 .. size_in_cells.0
  {
    for y in 0 .. size_in_cells.1
    {

      let material = if ( x + y ) % 2 == 0
      {
        black.clone()
      }
      else
      {
        white.clone()
      };

      let sprite = Sprite
      {
        size : Vec2::new( 1.0, 1.0 ),
        .. Default::default()
      };

      let transform = Transform
      {
        translation : Vec3::new
        (
          x as f32,
          y as f32,
          0.
        ),
        .. Default::default()
      };

      commands.spawn_bundle( SpriteBundle
      {
        sprite,
        material,
        transform,
        .. Default::default()
      });

    }
  }

}

///
/// Startup system for the game.
///

fn core_setup()
{
  let mut game = core::Game::default();
  game.board_print();
  game.make_move( "a2a4" );
  game.board_print();
}
