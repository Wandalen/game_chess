#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Chess game implemented on Bevy for educational purpose.
//!

use bevy::prelude::*;
#[ cfg( not( target_arch = "wasm32" ) ) ]
use bevy::audio::AudioPlugin;
use bevy::math::Vec4Swizzles;
#[ allow( unused_imports ) ] // qqq : remove with Timer implementation
use bevy::render::camera::{ camera_system, Camera };
use bevy::window::close_on_esc;
use bevy_egui::{ egui, EguiContext, EguiPlugin };
use game_chess_core as core;

pub mod camera;
pub mod common;
#[ cfg( not( target_arch = "wasm32" ) ) ]
pub mod highlight;
pub mod piece;

use common::GameState;

///
/// Color materials handles
///

#[ derive( Debug ) ]
pub struct Materials
{
  /// Black color
  pub black : Handle< ColorMaterial >,
  /// White color
  pub white : Handle< ColorMaterial >,
}

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

pub fn setup( mut commands : Commands, mut materials : ResMut< Assets< ColorMaterial > > )
{
  #[ allow( unused_mut, unused_variables ) ] // qqq : remove with Timer implementation
  let mut camera = commands.spawn_bundle( camera::ChessCameraBundle::new() );
  #[ cfg( not( target_arch = "wasm32" ) ) ]
  camera.insert( bevy_interact_2d::InteractionSource::default() );
  commands.spawn().insert( SelectedCell { pos : None } );
  // camera.insert( Timer::from_seconds( 2.0, false ) );
  commands.insert_resource( Materials
  {
    white : materials.add( ColorMaterial::from( Color::rgb( 0.9, 0.9, 0.7 ) ) ),
    black : materials.add( ColorMaterial::from( Color::rgb( 0.2, 0.2, 0.1 ) ) ),
  });
}

///
/// Board setup.
///

pub fn board_setup
(
  mut commands : Commands,
  #[ cfg( feature = "diagnostic" ) ]
  mut materials : ResMut< Assets< ColorMaterial > >,
  #[ cfg( not( feature = "diagnostic" ) ) ]
  materials : Res< Assets< ColorMaterial > >,
  materials_handles : Res< Materials >
)
{
  let size_in_cells = ( 8, 8 );

  let size = 2.0 / 8.0;
  let delta = 1.0 - size / 2.0;

  let black = materials.get( &materials_handles.black ).unwrap();
  let white = materials.get( &materials_handles.white ).unwrap();

  for x in 0 .. size_in_cells.0
  {
    for y in 0 .. size_in_cells.1
    {
      let is_black = ( x + y ) % 2 == 0;
      let material = if is_black
      {
        black.clone()
      }
      else
      {
        white.clone()
      };

      let sprite = Sprite
      {
        custom_size : Some( Vec2::new( size, size ) ),
        color : material.color,
        .. Default::default()
      };

      let transform = Transform
      {
        translation : Vec3::new( ( x as f32 ) * size - delta, ( y as f32 ) * size - delta, 0.0 ),
        .. Default::default()
      };

      commands.spawn_bundle( SpriteBundle
      {
        sprite,
        transform,
        .. Default::default()
      });
    }
  }

  #[ cfg( feature = "diagnostic" ) ]
  diagnostics_rect( &mut commands, &mut materials );
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
  });
}

///
/// Startup system for the game.
///

pub fn core_setup( mut commands : Commands, mut game_state : ResMut< State< GameState > > )
{
  let mut game = core::Game::default();
  game.board_print();
  game.make_move( "a2a4".into() );
  game.board_print();
  commands.insert_resource( game );

  game_state.set( GameState::GameStart ).unwrap();
}

// fn timer_system( time : Res< Time >, mut query : Query< &mut Timer >, mut game_state : ResMut< State< GameState > > )
// {
//   let mut timer = query.single_mut().unwrap();
//   timer.tick( time.delta() );
//   if timer.finished()
//   {
//     game_state.set( GameState::GameNew ).unwrap();
//   }
// }

fn init_system( mut game_state : ResMut< State< GameState > > )
{
  game_state.set( GameState::GameNew ).unwrap();
}

//Sounds
#[ cfg( not( target_arch = "wasm32" ) ) ]
fn loss( asset_server : Res< AssetServer >, audio_output : Res< Audio > )
{
  let music = asset_server.load( "sound/horror.mp3" );
  audio_output.play( music );
}
#[ allow( dead_code ) ]
#[ cfg( not( target_arch = "wasm32" ) ) ]
fn win( asset_server : Res< AssetServer >, audio_output : Res< Audio > )
{
  let music = asset_server.load( "sound/Windless Slopes.ogg" );
  audio_output.play( music );
}
#[ allow( dead_code ) ]
#[ cfg( not( target_arch = "wasm32" ) ) ]
fn draw( asset_server : Res< AssetServer >, audio_output : Res< Audio > )
{
  let music = asset_server.load( "sound/sad_trombone.mp3" );
  audio_output.play( music );
}
#[ allow( dead_code ) ]
#[ cfg( not( target_arch = "wasm32" ) ) ]
fn movement( asset_server : Res< AssetServer >, audio_output : Res< Audio > )
{
  let music = asset_server.load( "sound/hit.mp3" );
  audio_output.play( music );
}

///
/// GUI setup
///

pub fn egui_setup
(
  mut egui_context : ResMut< EguiContext >,
  mut materials : ResMut< Assets< ColorMaterial > >,
  materials_handles : Res< Materials >,
)
{
  egui::Window::new( "Timer" ).show( egui_context.ctx_mut(), | ui |
  {
    // add labels inside Egui window
    ui.label( "Time: 00:00.00" );
  });

  egui::SidePanel::left( "Menu" )
  .resizable( false )
  //.default_width( SIDE_PANEL_WIDTH )
  .show( egui_context.ctx_mut(), | ui |
  {
    ui.heading( "\"White\" cells color" );
    let material = materials.get_mut( &materials_handles.white ).unwrap();
    let mut color_schema = [ material.color.r(), material.color.g(), material.color.b(), 1.0 ];
    ui.horizontal( | ui |
    {
      if ui.color_edit_button_rgba_unmultiplied( &mut color_schema ).changed()
      {
        material.color = Color::rgb( color_schema[ 0 ],color_schema[ 1 ], color_schema[ 2 ] );
      }
    });
  });
}

///
/// System that highlights cells
///

#[ cfg( not( target_arch = "wasm32" ) ) ]
fn highlight_cells
(
  windows : Res< Windows >,
  interaction : Res< bevy_interact_2d::InteractionState >,
  q_camera : Query< &Camera >,
  mut highlight : ResMut< highlight::Highlight >,
  selected_cell : Query< &SelectedCell >,
  game : Res< core::Game >,
)
{
  let window = windows.get_primary().unwrap();
  let window_size = Vec2::new( window.width(), window.height() );

  let camera = q_camera.single();
  let cell = cursor_to_cell( interaction.last_cursor_position, window_size, camera.projection_matrix() );

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

  if let Some( pos ) = selected_cell.single().pos
  {
    highlight.highlight( pos, Color::rgba( 0.0, 1.0, 0.0, 1.0 ) );
  }
}

///
/// System that updates selected cell
///

#[ cfg( not( target_arch = "wasm32" ) ) ]
fn select_cell
(
  windows : Res< Windows >,
  mouse_button_input : Res< Input< MouseButton > >,
  interaction : Res< bevy_interact_2d::InteractionState >,
  q_camera : Query< &Camera >,
  mut selected_cell : Query< &mut SelectedCell >,
)
{
  if !mouse_button_input.just_released( MouseButton::Left )
  {
    return;
  }

  let window = windows.get_primary().unwrap();
  let window_size = Vec2::new( window.width(), window.height() );

  let camera = q_camera.single();
  let cell = cursor_to_cell( interaction.last_cursor_position, window_size, camera.projection_matrix() );

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

#[ derive( Component ) ]
struct SelectedCell
{
  pos : Option< ( u8, u8 ) >,
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

fn main()
{
  let mut app = App::new();
  /* background */
  app.insert_resource( ClearColor( Color::rgb( 0.9, 0.9, 0.9 ) ) );
  /* default plugins */
  app.add_plugins( DefaultPlugins );
  // app.insert_resource( CellColorSchema::default() );
  /* timer gui */
  app.insert_resource( WindowDescriptor
  {
    title : "Timer GUI".to_string(),
    width : 100.,
    height : 20.,
    resizable : true,
    .. Default::default()
  } );
  app.add_plugin( EguiPlugin );
  app.add_system( egui_setup );
  app.add_state( GameState::Init );
  // /* timer */
  // app.add_system_set( SystemSet::on_update( GameState::Init ).with_system( timer_system ) );
  app.add_system_set( SystemSet::on_update( GameState::Init ).with_system( init_system ) ); // qqq use system with timer
  /* setup core */
  app.add_system_set( SystemSet::on_update( GameState::GameNew ).with_system( core_setup ) );
  app.add_system_set( SystemSet::on_update( GameState::GameStart ).with_system( piece::pieces_setup ) );
  /* setup board */
  app.add_startup_system( setup );
  app.add_startup_stage( "board_setup", SystemStage::single( board_setup ) );

  /* sound */

  #[ cfg( not( target_arch = "wasm32" ) ) ]
  app.add_plugin( AudioPlugin ); // qqq : migrate to bevy_kira_audio https://github.com/NiklasEi/bevy_kira_audio
  #[ cfg( not( target_arch = "wasm32" ) ) ]
  app.add_startup_stage( "loss", SystemStage::single( loss ) );

  #[ cfg( not( target_arch = "wasm32" ) ) ]
  app.add_plugin( bevy_interact_2d::InteractionPlugin );

  /* highlighting */
  #[ cfg( not( target_arch = "wasm32" ) ) ]
  app.add_system_set
  (
    SystemSet::on_update( GameState::GameStart )
    .with_system( select_cell )
    .with_system( highlight_cells )
  );
  #[ cfg( not( target_arch = "wasm32" ) ) ]
  app.add_plugin( highlight::HighlightPlugin
  {
    clear_on_each_frame : true,
  });

  /* escape on exit */
  app.add_system( close_on_esc );

  // app.add_system( color_change );

  app.add_system_to_stage
  (
    CoreStage::PostUpdate,
    camera_system::< camera::ChessProjection >,
  );

  app.run();
}
