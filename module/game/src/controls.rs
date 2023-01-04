//!
//! Controls.
//!

use bevy::prelude::*;
use bevy::math::Vec4Swizzles;
use game_chess_core::
{
  Piece,
  UCI,
  Game,
};
use crate::GameState;

///
/// Resource which stores selected cell
///

#[ derive( Component, Debug ) ]
pub enum Selection
{
  /// Empty cell
  EmptyCell( u8, u8 ),
  /// Cell with a piece
  Piece( u8, u8 ),
  /// No selection
  None,
}

///
/// System that handles mouse click
///

pub fn handle_click
(
  windows : Res< Windows >,
  mouse_button_input : Res< Input< MouseButton > >,
  q_camera : Query< &Camera >,
  mut selected_cell : Query< &mut Selection >,
  mut game : ResMut< Game >,
)
{
  if !mouse_button_input.just_released( MouseButton::Left )
  {
    return;
  }

  let cell = cell_number( &windows.get_primary().unwrap(), &q_camera.single() );

  let mut selected_cell = selected_cell.single_mut();
  let selected_cell = selected_cell.as_mut();
  cell.map( | c | select_cell( &c, selected_cell, game ) );
}

fn select_cell( cell : &Vec2, selected_cell : &mut Selection, mut game : ResMut< Game > )
{
  let ( x, y ) = ( cell.x as u8, cell.y as u8 );
  match selected_cell
  {
    // if piece selected
    Selection::Piece( selected_x, selected_y ) =>
    {
      if let Some( uci ) = uci( ( *selected_x, *selected_y ), ( x, y ) )
      {
        // try to make move to cell
        if game.make_move( uci )
        {
          game.make_move_ai();
          *selected_cell = Selection::None;
          return;
        }
      }
    },
    _ => {},
  }
  // add to selection current cell
  *selected_cell = if game.piece_at( calc_square( x, y ) ) == Piece::None
  {
    Selection::EmptyCell( x, y )
  }
  else
  {
    Selection::Piece( x, y )
  };
}

///
/// System that handles keyboard input.
///
pub fn handle_keyboard
(
  mut keys : ResMut< Input< KeyCode > >,
  mut app_state : ResMut< State< GameState > >,
)
{
  if keys.just_pressed( KeyCode::Escape )
  {
    app_state.set( GameState::Pause ).unwrap();
    // Not doing this can cause issues https://github.com/bevyengine/bevy/issues/1700.
    keys.reset( KeyCode::Escape );
  }
}

///
/// Convert move to the UCI format.
///

pub fn uci( from : ( u8, u8 ), to : ( u8, u8 ) ) -> Option< UCI >
{
  let coords =
  (
    to_x_uci_coord( from.0 ),
    to_y_uci_coord( from.1 ),
    to_x_uci_coord( to.0 ),
    to_y_uci_coord( to.1 ),
  );
  if let ( Some( from_x ), Some( from_y ), Some( to_x ), Some( to_y ) ) = coords
  {
    let uci = String::from_iter( [ from_x, from_y, to_x, to_y ] );
    return Some( UCI( uci ) );
  }

  None
}

///
/// Get x cell coordinate in the UCI format.
/// Return None if the conversion isn't possible.
///

fn to_x_uci_coord( x : u8 ) -> Option< char >
{
  Some
  (
    match x
    {
      0 => 'a',
      1 => 'b',
      2 => 'c',
      3 => 'd',
      4 => 'e',
      5 => 'f',
      6 => 'g',
      7 => 'h',
      _ => return None,
    }
  )
}

///
/// Get y cell coordinate in the UCI format.
/// Return None if the conversion isn't possible.
///

fn to_y_uci_coord( mut y : u8 ) -> Option< char >
{
  y += 1;
  if y > 8
  {
    return None;
  }
  char::from_digit( y as u32, 10 )
}

///
/// Calculate square index from cell number
///

pub fn calc_square( x : u8, y : u8 ) -> u8
{
  8 * y + x
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
