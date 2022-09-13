#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

//!
//! The rendered content is scaled to maintain its aspect ratio while fitting within the windows.
//!

use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy::render::camera::CameraProjection;
use bevy::render::camera::DepthCalculation;

///
/// The rendered content is scaled to maintain its aspect ratio while fitting within the windows.
///

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct ChessProjection
{
  /// offset from left side
  pub left : f32,
  /// offset from right side
  pub right : f32,
  /// offset from bottom
  pub bottom : f32,
  /// offset from top
  pub top : f32,
  /// near element for depth sorting
  pub near : f32,
  /// far element for depth sorting
  pub far : f32,
}

impl CameraProjection for ChessProjection
{
  ///
  /// Transform positions points to projection matrix.
  ///
  fn get_projection_matrix(&self) -> Mat4
  {
    Mat4::orthographic_rh(self.left, self.right, self.bottom, self.top, self.near, self.far)
  }

  ///
  /// Setup projection points taking into account window size and board bounds.
  ///
  fn update(&mut self, width : f32, height : f32)
  {
    /* Margin size must be equals to cell size. */
    /* Size of board in coordinates is 2.0. */
    const BOARD_SIZE : f32 = 2.0;
    const CELLS_COUNT : f32 = 8.0;
    /* Size of cell in cordinates. */
    const CELL_SIZE : f32 = BOARD_SIZE / CELLS_COUNT;
    /* Board offset including margin. */
    const OFFSET : f32 = 1.0 + CELL_SIZE;

    if width > height
    {
      /* if width > height we need to shrink left and right sides by delta */
      let delta = width / height - 1.0;
      self.left = -OFFSET - delta;
      self.right = OFFSET + delta;
      self.top = OFFSET;
      self.bottom = -OFFSET;
    }
    else
    {
      /* if width > height we need to shrink bottom and top by delta */
      let delta = height / width - 1.0;
      self.left = -OFFSET;
      self.right = OFFSET;
      self.top = OFFSET + delta;
      self.bottom = -OFFSET - delta;
    }
  }

  ///
  /// Sort entities by depth. Not used.
  ///
  fn depth_calculation(&self) -> DepthCalculation { DepthCalculation::Distance }

  /// Far.
  fn far(&self) -> f32 { self.far }
}

impl Default for ChessProjection
{
  /* Default settings. */
  fn default() -> Self
  {
    ChessProjection {
      left : -1.0,
      right : 1.0,
      bottom : -1.0,
      top : 1.0,
      near : 0.0,
      far : 1000.0,
    }
  }
}

///
/// Alternative camera bundle that show up the game board.
///

#[derive(Component, Bundle, Debug)]
pub struct ChessCameraBundle
{
  /// Instance of camera.
  pub camera : Camera,
  /// Custom projection.
  pub chess_projection : ChessProjection,
  /// Local transform.
  pub transform : Transform,
  /// Global transform.
  pub global_transform : GlobalTransform,
}

impl ChessCameraBundle
{
  ///
  /// ChessCameraBundle constructor.
  ///
  pub fn new() -> Self
  {
    let far = 1000.0;
    ChessCameraBundle {
      camera : Camera { ..Default::default() },
      chess_projection : ChessProjection {
        far,
        ..Default::default()
      },
      transform : Transform::from_xyz(0.0, 0.0, far - 0.1),
      global_transform : Default::default(),
    }
  }
}
