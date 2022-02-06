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
/// Piece texture atlas
///

#[derive(Debug)]
pub struct PieceTextureAtlas( Handle< TextureAtlas > );

type PieceToTexture = std::collections::HashMap< u8, u8 >;

///
/// Game states
///

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState
{
  Init = 0,
  Game = 1
}

///
///
///

pub fn pieces_setup(mut commands : Commands, asset_server : Res< AssetServer >, mut texture_atlases : ResMut< Assets< TextureAtlas > >)
{
  let texture_handle = asset_server.load( "piece/tileset_64.png" );
  let texture_atlas = TextureAtlas::from_grid( texture_handle, Vec2::new( 64.0, 64.0 ), 6, 2 );
  let texture_atlas_handle = texture_atlases.add( texture_atlas );
  commands.insert_resource( PieceTextureAtlas( texture_atlas_handle ) );

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

  commands.insert_resource( piece_to_texture );
}

///
/// Graphics setup.
///

pub fn graphics_setup(mut commands : Commands, mut materials : ResMut<Assets<ColorMaterial>>, texture_atlas_handle : Res< PieceTextureAtlas >, piece_to_texture : Res< PieceToTexture >, game : Res< core::Game >, mut game_state : ResMut< State < GameState > > )
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

      let cell = commands.spawn_bundle(SpriteBundle {
        sprite,
        material,
        transform,
        ..Default::default()
      }).id();

      let cell_index = size_in_cells.0 * y + x;

      let piece = game.piece_at( cell_index );
      if !piece.is_none()
      {
        let texture_atlas = texture_atlas_handle.0.clone();
        let texture_id = *piece_to_texture.get( &( piece.unwrap() as u8 ) ).unwrap();

        let transform = Transform {
          translation : Vec3::new(0.0, 0.0, 0.1 ),
          scale : Vec3::new( 1.0 / 64.0 * size * 0.75, 1.0/ 64.0 * size * 0.75, 1.0 ),
          ..Default::default()
        };
        let piece = SpriteSheetBundle
        {
          texture_atlas,
          sprite : TextureAtlasSprite::new( texture_id as u32 ),
          transform,
          ..Default::default()
        };

        let piece_entity = commands.spawn_bundle( piece ).id();
        commands.entity(cell).push_children(&[piece_entity]);
      }
    }
  }

  game_state.set( GameState::Game ).unwrap()
}

///
/// Startup system for the game.
///

pub fn core_setup( mut commands : Commands )
{
  let mut game = core::Game::default();
  game.board_print();
  game.make_move("a2a4".into());
  game.board_print();
  commands.insert_resource( game );
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
  /* game state */
  app.add_state( GameState::Init );
  /* setup core */
  app.add_startup_system(core_setup.system());
  /* setup pieces */
  app.add_startup_system(pieces_setup.system());
  /* setup graphics */
  app.add_system_set( SystemSet::on_update( GameState::Init ).with_system( graphics_setup.system() ) );
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
