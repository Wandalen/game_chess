#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Chess game implemented on Bevy for educational purpose.
//!

use bevy::prelude::*;
use bevy_kira_audio::{ AudioPlugin, AudioControl };
use bevy::render::camera::{ camera_system, Camera };
use bevy_egui::{ egui, EguiContext, EguiPlugin };
use game_chess_core as core;

pub mod camera;
pub mod common;
pub mod highlight;
pub mod piece;
pub mod controls;
pub mod main_menu;
pub mod pause_menu;
pub mod settings;
pub mod multiplayer;

use common::GameState;
use controls::Selection;
use crate::common::Multiplayer;
use crate::settings::Settings;


// /// mut material  359
// /// My color change
// ///
//
// pub fn color_change
// (
//   mut materials: ResMut< Assets< ColorMaterial > >,
//    query_white: Query< &Handle< ColorMaterial >, With< CellWhite > >,
//    query_black: Query< &Handle< ColorMaterial >, With< CellBlack > >,
//     color_schema: Res< CellColorSchema > )
//     {
//
//   for handle in query_white.iter() {
//     let mut material = materials.get_mut( handle ).unwrap();
//     material.color = Color::rgb( color_schema.white[ 0 ],color_schema.white[ 1 ], color_schema.white[ 2 ] );
//   }
//
//   for handle in query_black.iter() {
//     let mut material = materials.get_mut( handle ).unwrap();
//     material.color = Color::rgb( color_schema.black[ 0 ],color_schema.black[ 1 ], color_schema.black[ 2 ] );
//   }
//   /*commands.insert_resource( Materials {
//   _white : materials.add( ColorMaterial::color( Color::rgb( 0.9, 0.0, 0.0 ) ) );
//   _black : materials.add( ColorMaterial::color( Color::rgb( 0.2, 0.8, 0.8 ) ) );
//   } );*/
// }

///
/// Setup camera and add resources
///

#[ derive( Component ) ]
struct IterSource( bevy_interact_2d::InteractionSource ); 

pub fn setup( mut commands : Commands )
{
  let mut camera = commands.spawn( camera::ChessCameraBundle::new() );
  #[ cfg( not( target_arch = "wasm32" ) ) ]
  camera.insert( IterSource( bevy_interact_2d::InteractionSource::default() ) );
  camera.insert( GameTimer { timer : Timer::from_seconds( 2.0, TimerMode::Once ) } );

  commands.spawn( Selection::None );
  commands.init_resource::< Settings >();
}

///
/// Board setup.
///

pub fn board_setup
(
  mut commands : Commands,
  #[ cfg( feature = "diagnostic" ) ]
  mut materials : ResMut< Assets< ColorMaterial > >,
)
{
  let size_in_cells = ( 8, 8 );

  let size = 2.0 / 8.0;
  let delta = 1.0 - size / 2.0;

  for x in 0 .. size_in_cells.0
  {
    for y in 0 .. size_in_cells.1
    {
      let is_black = ( x + y ) % 2 == 0;

      let sprite = Sprite
      {
        custom_size : Some( Vec2::new( size, size ) ),
        .. Default::default()
      };

      let transform = Transform
      {
        translation : Vec3::new( ( x as f32 ) * size - delta, ( y as f32 ) * size - delta, 0.0 ),
        .. Default::default()
      };

      commands.spawn( SpriteBundle
      {
        sprite,
        transform,
        .. Default::default()
      } )
      .insert( Cell { is_black } );
    }
  }

  #[ cfg( feature = "diagnostic" ) ]
  diagnostics_rect( &mut commands, &mut materials );
}

///
/// Component that holds information about a cell.
///

#[ derive( Component, Debug ) ]
pub struct Cell
{
  is_black : bool,
}

///
/// System that changes color scheme.
///

pub fn gamma_change
(
  materials : ResMut< Assets< ColorMaterial > >,
  settings : Res< Settings >,
  mut query : Query< ( &Cell, &mut Sprite ) >,
)
{
  if settings.is_changed()
  {
    let black = materials.get( &settings.color_handles.black ).unwrap();
    let white = materials.get( &settings.color_handles.white ).unwrap();

    for ( cell, mut sprite ) in query.iter_mut()
    {
      if cell.is_black
      {
        sprite.color = black.color;
      }
      else
      {
        sprite.color = white.color;
      }
    }
  }
}

#[ cfg( feature = "diagnostic" ) ]
/// Add sprite of size 2x2 for diagnostics purpose. The sprite should cover central zone of window.
#[ cfg( feature = "diagnostic" ) ]
pub fn diagnostics_rect( commands : &mut Commands, materials : &mut ResMut< Assets< ColorMaterial > > )
{
  let color = Color::rgb( 0.9, 0.2, 0.2 );
  materials.add( ColorMaterial::from( color ) );

  let sprite = Sprite
  {
    custom_size : Some( Vec2::new( 2., 2. ) ),
    color,
    .. Default::default()
  };

  let transform = Transform
  {
    translation : Vec3::new( 0.0, 0.0, 0.0 ),
    .. Default::default()
  };

  commands.spawn_bundle( SpriteBundle
  {
    sprite,
    transform,
    .. Default::default()
  } );
}

///
/// Startup system for the game.
///

pub fn core_setup
(
  mut commands : Commands,
  mut game_state : ResMut< State< GameState > >,
  server : Res< AssetServer >,
  texture_atlases : ResMut< Assets< TextureAtlas > >,
)
{
  let game = core::Game::default();
  game.board_print();
  piece::pieces_setup( &mut commands, server, texture_atlases, &game );
  commands.insert_resource( game );

  game_state.set( GameState::GamePlaying ).unwrap();
}

fn timer_system( time : Res< Time >, mut query : Query< &mut GameTimer >, mut game_state : ResMut< State< GameState > > )
{
  let timer = &mut query.single_mut().timer;
  timer.tick( time.delta() );
  if timer.finished()
  {
    game_state.set( GameState::GameNew ).unwrap();
  }
}

fn init_system( mut game_state : ResMut< State< GameState > > )
{
  game_state.set( GameState::MainMenu ).unwrap();
}

//Sounds
fn loss( asset_server : Res< AssetServer >, audio_output : Res< bevy_kira_audio::Audio > )
{
  let music = asset_server.load( "sound/horror.mp3" );
  audio_output.play( music );
}
#[ allow( dead_code ) ]
fn win( asset_server : Res< AssetServer >, audio_output : Res< bevy_kira_audio::Audio > )
{
  let music = asset_server.load( "sound/Windless Slopes.ogg" );
  audio_output.play( music );
}
#[ allow( dead_code ) ]
fn draw( asset_server : Res< AssetServer >, audio_output : Res< bevy_kira_audio::Audio > )
{
  let music = asset_server.load( "sound/sad_trombone.mp3" );
  audio_output.play( music );
}
#[ allow( dead_code ) ]
fn movement( asset_server : Res< AssetServer >, audio_output : Res< bevy_kira_audio::Audio > )
{
  let music = asset_server.load( "sound/hit.mp3" );
  audio_output.play( music );
}

///
/// System that enables/disables sound
///
pub fn sound_control( audio_output : Res< bevy_kira_audio::Audio >, settings : Res< Settings > )
{
  if settings.is_changed()
  {
    if settings.enable_sound
    {
      audio_output.resume();
    }
    else
    {
      audio_output.pause();
    }
  }
}

///
/// GUI setup
///

#[ derive( Resource ) ]
struct ResEquiContext ( EguiContext );

pub fn egui_setup
(
  mut egui_context : ResMut< ResEquiContext >,
)
{
  egui::Window::new( "Timer" ).show( egui_context.ctx_mut(), | ui |
  {
    // add labels inside Egui window
    ui.label( "Time: 00:00.00" );
  } );
}

#[ derive( Component ) ]
struct GameTimer
{
  timer : Timer,
}

///
/// System that highlights cells
///

fn highlight_cells
(
  windows : Res< Windows >,
  q_camera : Query< &Camera >,
  mut highlight : ResMut< highlight::Highlight >,
  selected_cell : Query< &Selection >,
  game : Res< core::Game >,
)
{
  let cell = controls::cell_number( windows.primary(), q_camera.single() );

  highlight_legal_moves( &selected_cell, &mut highlight, &game );

  if let Some( cell ) = cell
  {
    let x = cell.x as u8;
    let y = cell.y as u8;
    let color = if game.piece_at( 8 * y + x ) == core::Piece::None
    {
      Color::rgba( 1.0, 0.0, 0.0, 1.0 )
    }
    else
    {
      Color::rgba( 0.0, 0.0, 1.0, 1.0 )
    };
    highlight.highlight( ( x, y ), color );
  }

  match selected_cell.single()
  {
    Selection::EmptyCell( x, y ) | Selection::Piece( x, y ) => highlight.highlight( ( *x, *y ), Color::rgba( 0.0, 1.0, 0.0, 1.0 ) ),
    Selection::None => {}
  }
}

fn index_to_pos( index : u8 ) -> ( u8, u8 )
{
  let y = index / 8;
  ( index - 8 * y, y )
}

///
/// Highlight legal moves
///

fn highlight_legal_moves
(
  selected_cell : &Query< &Selection >,
  highlight : &mut ResMut< highlight::Highlight >,
  game : &Res< core::Game >
)
{
  if let Selection::Piece( x, y ) = selected_cell.single()
  {
    game.moves_list().iter()
    .filter( | mv |  mv.get_src_u8() == 8 * y + x )
    .for_each( | mv |
    {
      let color = if mv.is_capture()
      { Color::rgba( 1.0, 0.5, 0.0, 1.0 ) }
      else
      { Color::rgba( 1.0, 1.0, 0.0, 1.0 ) };

      highlight.highlight( index_to_pos( mv.get_dest_u8() ), color );
    });
  };
}

// ///
// /// Mark cells
// ///
//
// #[ derive( Debug ) ]
// pub struct Cell;
//
// ///
// /// Mark white cells
// ///
//
// #[ derive( Debug ) ]
// pub struct CellWhite;
//
// ///
// /// Mark black cells
// ///
//
// #[ derive( Debug ) ]
// pub struct CellBlack;
//
// ///
// /// Game color schema
// ///
//
// #[ derive( Debug ) ]
// pub struct CellColorSchema
// {
//   /// White color
//   pub white : [ f32; 4 ],
//   /// Black color
//   pub black : [ f32; 4 ],
// }
//
// impl Default for CellColorSchema
// {
//   fn default() -> Self
//   {
//     Self {
//       white : [ 0.98, 0.94, 1.0, 1. ],
//       black : [ 0.0, 0.2, 0.2, 1. ],
//     }
//   }
// }

// 

fn main()
{
  let mut app = App::new();
  /* background */
  app.insert_resource( ClearColor( Color::rgb( 0.9, 0.9, 0.9 ) ) );
  /* default plugins */
  app.add_plugins( DefaultPlugins );
  // app.insert_resource( CellColorSchema::default() );
  /* timer gui */
  app.insert_resource( Window
  {
    title : "Timer GUI".to_string(),
    resolution: ( 100., 20. ).into(),
    resizable : true,
    ..Default::default()
  } );
  app.add_plugin( EguiPlugin );
  app.add_system_set( SystemSet::on_update( GameState::GamePlaying ).with_system( egui_setup ) );
  app.add_system( gamma_change );
  app.add_system( sound_control );
  app.add_state( GameState::Init );
  /* main menu */
  app.add_system_set( SystemSet::on_update( GameState::MainMenu ).with_system( main_menu::setup_main_menu ) );
  /* settings menu */
  app.add_system_set( SystemSet::on_update( GameState::Settings ).with_system( settings::settings_menu ) );
  /* pause menu */
  app.add_system_set( SystemSet::on_update( GameState::Pause ).with_system( pause_menu::setup_pause_menu ) );
  // /* timer */
  app.add_system_set( SystemSet::on_update( GameState::Init ).with_system( timer_system ) );
  app.add_system_set( SystemSet::on_update( GameState::Init ).with_system( init_system ) ); // qqq use system with timer
  /* setup core */
  app.add_system_set( SystemSet::on_update( GameState::GameNew ).with_system( core_setup ) );
  app.add_system_set( SystemSet::on_update( GameState::GamePlaying ).with_system( piece::draw_pieces ) );
  /* setup board */
  app.add_startup_system( setup );
  app.add_startup_stage( "board_setup", SystemStage::single( board_setup ) );

  /* sound */

  app.add_plugin( AudioPlugin );
  app.add_startup_stage( "loss", SystemStage::single( loss ) );

  #[ cfg( not( target_arch = "wasm32" ) ) ]
  app.add_plugin( bevy_interact_2d::InteractionPlugin );

  /* highlighting */
  app.add_system_set
  (
    SystemSet::on_update( GameState::GamePlaying )
    .with_system( controls::handle_click )
    .with_system( controls::handle_keyboard )
    .with_system( highlight_cells )
  );
  app.add_plugin( highlight::HighlightPlugin
  {
    clear_on_each_frame : true,
  } );

  /* Multiplayer */
  app.add_system_set
  (
    SystemSet::on_update( GameState::MultiplayerGame( Multiplayer::ConnectingToServer ) )
      .with_system( multiplayer::menu::connect_menu )
  );
  app.add_system_set
  (
    SystemSet::on_enter( GameState::MultiplayerGame( Multiplayer::ConnectingToServer ) )
      .with_system( multiplayer::menu::setup )
  );

  // app.add_system( color_change );

  app.add_system_to_stage
  (
    CoreSet::PostUpdate,
    camera_system::< camera::ChessProjection >,
  );

  app.run();
}
