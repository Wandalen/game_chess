#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

//!
//! Piece drawing
//!

use bevy::prelude::*;
use game_chess_core as core;
use crate::core::Piece;

///
/// Piece texture atlas
///

type PieceToTexture = std::collections::HashMap<u8, u8>;

///
/// Pieces drawing system
///

pub fn pieces_setup(
  mut commands : Commands,
  asset_server : Res<AssetServer>,
  mut texture_atlases : ResMut<Assets<TextureAtlas>>,
  game : Res<core::Game>,
)
{

}
