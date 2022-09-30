//!
//! Controls.
//!

use bevy::prelude::*;
use bevy::math::Vec4Swizzles;

///
/// Resource which stores selected cell
///

#[ derive( Component, Debug ) ]
pub struct SelectedCell
{
  /// Cell number
  pub pos : Option< ( u8, u8 ) >,
}

///
/// System that updates selected cell
///

pub fn select_cell
(
  windows : Res< Windows >,
  mouse_button_input : Res< Input< MouseButton > >,
  q_camera : Query< &Camera >,
  mut selected_cell : Query< &mut SelectedCell >,
)
{
  if !mouse_button_input.just_released( MouseButton::Left )
  {
    return;
  }

  let cell = cell_number( &windows.get_primary().unwrap(), &q_camera.single() );

  let mut selected_cell = selected_cell.single_mut();
  if let Some( cell ) = cell
  {
    let x = cell.x as u8;
    let y = cell.y as u8;

    if let Some( pos ) = selected_cell.pos
    {
      if pos.0 == x && pos.1 == y
      {
        selected_cell.pos = None;
        return;
      }
    }
    selected_cell.pos = Some( ( x, y ) );
  }
}

///
/// Get cell number
///

pub fn cell_number( window : &Window, camera : &Camera ) -> Option< Vec2 >
{
  let window_size = Vec2::new( window.width(), window.height() );

  if let Some( cursor_pos ) = window.cursor_position()
  {
    return cursor_to_cell( cursor_pos, window_size, camera.projection_matrix() );
  }

  None
}

///
/// Convert cursor position to cell number
/// If cursor is outside of the board, returns None
///

pub fn cursor_to_cell( cursor_pos : Vec2, window_size : Vec2, projection_matrix : Mat4 ) -> Option< Vec2 >
{
  let clip_pos = ( cursor_pos / ( window_size / 2.0 ) ) - Vec2::splat( 1.0 );
  let clip_pos_4 = clip_pos.extend( 0.0 ).extend( 1.0 );
  let world_pos_4 = projection_matrix.inverse() * clip_pos_4;

  let world_pos = world_pos_4.xy() / world_pos_4.w;
  let pos = ( ( world_pos + Vec2::splat( 1.0 ) ) * 4.0 ).floor();
  if pos.x < 8.0 && pos.y < 8.0 && pos.x >= 0.0 && pos.y >= 0.0
  {
    Some( pos )
  }
  else
  {
    None
  }
}
