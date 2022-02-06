#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

//!
//! Piece drawing
//!

use bevy::prelude::*;
use game_chess_core as core;

///
/// Piece texture atlas
///

#[derive(Debug)]
pub struct PieceTextureAtlas(Handle<TextureAtlas>);

type PieceToTexture = std::collections::HashMap<u8, u8>;

///
/// Pieces drawing system
///

pub fn pieces_setup(mut commands : Commands, asset_server : Res< AssetServer >, mut texture_atlases : ResMut< Assets< TextureAtlas > >, game : Res< core::Game > )
{
  let texture_handle = asset_server.load( "piece/tileset_64.png" );
  let texture_atlas = TextureAtlas::from_grid( texture_handle, Vec2::new( 64.0, 64.0 ), 6, 2 );
  let texture_atlas_handle = texture_atlases.add( texture_atlas );
  // commands.insert_resource( PieceTextureAtlas( texture_atlas_handle ) );

  let piece_to_texture : PieceToTexture = std::collections::HashMap::from
  ([
    ( core::Piece::BlackRook as u8, 0 ),
    ( core::Piece::BlackKnight as u8, 1 ),
    ( core::Piece::BlackBishop as u8, 2 ),
    ( core::Piece::BlackQueen as u8, 3 ),
    ( core::Piece::BlackKing as u8, 4 ),
    ( core::Piece::BlackPawn as u8, 5 ),
    ( core::Piece::WhiteRook as u8, 6 ),
    ( core::Piece::WhiteKnight as u8, 7 ),
    ( core::Piece::WhiteBishop as u8, 8 ),
    ( core::Piece::WhiteQueen as u8, 9 ),
    ( core::Piece::WhiteKing as u8, 10 ),
    ( core::Piece::WhitePawn as u8, 11 ),
  ]);

  let size_in_cells = (8, 8);
  let size = 2.0 / 8.0;
  let delta = 1.0 - size / 2.0;

  let piece_scale = 1.0 / 64.0 * size * 0.75;

  for x in 0 .. size_in_cells.0
  {
    for y in 0 .. size_in_cells.1
    {
      let cell_index = size_in_cells.0 * y + x;

      let piece = game.piece_at(cell_index);
      if !piece.is_none()
      {
        let texture_atlas = texture_atlas_handle.clone();
        let texture_id = piece_to_texture.get( &( piece.unwrap() as u8 ) ).unwrap();

        let transform = Transform {
          translation : Vec3::new((x as f32) * size - delta, (y as f32) * size - delta, 0.0),
          scale : Vec3::new(piece_scale, piece_scale, 1.0),
          ..Default::default()
        };

        let piece = SpriteSheetBundle {
          texture_atlas,
          sprite : TextureAtlasSprite::new( *texture_id as u32 ),
          transform,
          ..Default::default()
        };

        commands.spawn_bundle(piece);
      }
    }
  }
}
