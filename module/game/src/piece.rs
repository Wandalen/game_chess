#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Piece drawing
//!

use bevy::prelude::*;
use game_chess_core::Game;

const SIZE : f32 = 2.0 / 8.0;
const DELTA : f32 = 1.0 - SIZE / 2.0;
const PIECE_SCALE : f32 = 1.0 / 64.0 * SIZE * 0.75;

///
/// Piece id.
///

#[ derive( Component, Debug ) ]
pub struct PieceId
{
  id : u8,
}

///
/// Setup pieces.
///

pub fn pieces_setup
(
  commands : &mut Commands,
  server : Res< AssetServer >,
  mut texture_atlases : ResMut< Assets< TextureAtlas > >,
  game : &Game,
)
{
  let texture_atlas_handle : Handle< Image > = server.load( "piece/tileset_64.png" );
  let texture_atlas = TextureAtlas::from_grid( texture_atlas_handle, Vec2::new( 64.0, 64.0 ), 6, 2, None, None );
  let texture_atlas_handle = texture_atlases.add( texture_atlas );

  for x in 0 .. 8
  {
    for y in 0 .. 8
    {
      let ( id, piece ) = game.piece_at_with_id( x, y );

      if piece != game_chess_core::Piece::None
      {
        let texture_id = piece_to_texture_id( &piece ).unwrap();
        let sprite = TextureAtlasSprite::new( texture_id );

        let transform = Transform
        {
          scale : Vec3::new( PIECE_SCALE, PIECE_SCALE, 1.0 ),
          .. Default::default()
        };

        let piece = SpriteSheetBundle
        {
          texture_atlas : texture_atlas_handle.clone(),
          sprite,
          transform,
          ..Default::default()
        };

        commands.spawn( piece ).insert( PieceId { id } );
      }
    }
  }
}

///
/// Pieces drawing system.
///

pub fn draw_pieces
(
  mut commands : Commands,
  mut query : Query< ( Entity, &PieceId, &mut Transform ) >,
  game : NonSend< Game >,
)
{
  for ( entity, id, mut transform ) in query.iter_mut()
  {
    let ( piece, x, y ) = game.piece_by_id( id.id );
    if piece == game_chess_core::Piece::None
    {
      commands.entity( entity ).despawn();
    }
    else
    {
      transform.translation = Vec3::new( ( x as f32 ) * SIZE - DELTA, ( y as f32 ) * SIZE - DELTA, 1.0 );
    }
  }
}

fn piece_to_texture_id( piece : &game_chess_core::Piece ) -> Option< usize >
{
  Some
    (
      match piece
      {
        game_chess_core::Piece::BlackRook => 0,
        game_chess_core::Piece::BlackKnight => 1,
        game_chess_core::Piece::BlackBishop => 2,
        game_chess_core::Piece::BlackQueen => 3,
        game_chess_core::Piece::BlackKing => 4,
        game_chess_core::Piece::BlackPawn => 5,
        game_chess_core::Piece::WhiteRook => 6,
        game_chess_core::Piece::WhiteKnight => 7,
        game_chess_core::Piece::WhiteBishop => 8,
        game_chess_core::Piece::WhiteQueen => 9,
        game_chess_core::Piece::WhiteKing => 10,
        game_chess_core::Piece::WhitePawn => 11,
        game_chess_core::Piece::None => return None,
      }
    )
}
