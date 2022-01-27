#![warn(missing_docs)]

use bevy::prelude::*;
use bevy_egui::{ egui, EguiContext };

const DESK_HEIGHT: u8 = 8;
const DESK_WIDTH: u8 = 8;

const SIDE_PANEL_WIDTH : f32 = 300.0;

//

#[ derive( Clone, Eq, PartialEq ) ]
pub enum Side
{
  Black,
  White,
}

#[ derive( Clone, Eq, PartialEq ) ]
pub struct CurrentSide( Side );

//

// start setup, adding main resources
pub fn setup
(
  mut commands: Commands,
  mut materials: ResMut<Assets<ColorMaterial>>,
)
{
  commands.spawn_bundle( OrthographicCameraBundle::new_2d() );
  commands.spawn_bundle( UiCameraBundle::default() );
  // add resource with materials for chess board
  commands.insert_resource( Materials
  {
    black : materials.add( Color::rgb( 0.30, 0.05, 0.0 ).into() ),
    white : materials.add( Color::rgb( 1.0, 1.0, 1.0 ).into() ),
  });
  // add resource for combobox
  commands.insert_resource( CurrentSide( Side::White ) );
}

//

pub fn setup_egui( egui_context : Res<EguiContext>, mut side : ResMut<CurrentSide> )
{
  // add fixated panel
  egui::SidePanel::left( "Menu" )
  .resizable( false )
  .default_width( SIDE_PANEL_WIDTH )
  .show( egui_context.ctx(), | ui |
  {
    let choose = match &side.0
    {
        Side::White => "White",
        Side::Black => "Black"
    };

    // add combobox
    egui::ComboBox::from_label( "Select side!" )
    .selected_text( choose )
    .show_ui(ui, | ui |
    {
      ui.selectable_value( &mut side.0, Side::White, "White" );
      ui.selectable_value( &mut side.0, Side::Black, "Black" );
    })
  });
}

//

// struct for board position declaration
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position
{
  pub x: i32,
  pub y: i32,
}

// a vector of board sprites
pub struct BoardSegments( pub Vec<Entity> );
impl Default for BoardSegments
{
  fn default() -> Self
  {
    BoardSegments(Vec::with_capacity( ( DESK_WIDTH * DESK_HEIGHT ) as usize ) )
  }
}

// a struct to define size of chess board square
pub struct Size
{
  width: f32,
  height: f32,
}

impl Size
{
  pub fn square( x: f32 ) -> Self
  {
    Self
    {
      width: x,
      height: x,
    }
  }
}

//

// a struct to handle game matereals
pub struct Materials
{
  pub black: Handle<ColorMaterial>,
  pub white: Handle<ColorMaterial>,
}

//

pub fn spawn_board
(
    mut commands: Commands,
    materials: Res<Materials>,
    mut segments: ResMut<BoardSegments>,
)
{
  for x in 0..DESK_WIDTH
  {
    for y in 0..DESK_HEIGHT
    {

      let material = if ( x + y + 1 ) % 2 == 0
      {
        &materials.white
      }
      else
      {
        &materials.black
      };
      let seg = segment_spawn
      (
        &mut commands,
        material,
        Position
        {
          x: x as i32,
          y: y as i32,
        },
        0.95,
      );
      segments.0.push( seg );
    }
  }
}

//

fn segment_spawn
(
  commands : &mut Commands,
  material : &Handle<ColorMaterial>,
  position: Position,
  size: f32,
) -> Entity
{
  commands.spawn_bundle( SpriteBundle
  {
    material: material.clone(),
    sprite: Sprite::new( Vec2::new( 10.0, 10.0 ) ),
    ..Default::default()
  })
  .insert( position )
  .insert( Size::square( size ) )
  .id()
}

//

// post system which resizes board squares
pub fn size_scaling( windows : Res<Windows>, mut q : Query<( &Size, &mut Sprite )> )
{
  let window = windows.get_primary().unwrap();
  let mut width = window.width() - SIDE_PANEL_WIDTH;
  let mut height = window.height();
  if width > height
  {
    width = height;
  }
  if width < height
  {
    height = width;
  }

  for ( sprite_size, mut sprite ) in q.iter_mut()
  {
    sprite.size = Vec2::new
    (
      ( sprite_size.width / DESK_WIDTH as f32 * width as f32 ) * 0.9,
      ( sprite_size.height / DESK_HEIGHT as f32 * height as f32 ) * 0.9,
    );
  }
}

//

// post system which sets board squares positions
pub fn position_translation
(
  windows: Res<Windows>,
  mut q: Query<( &Position, &mut Transform ) >,
)
{
  fn convert( pos : f32, bound_window : f32, bound_game : f32 ) -> f32
  {
    let tile_size = bound_window / bound_game;
    pos / bound_game * bound_window - ( bound_window / 2.0 ) + ( tile_size / 2.0 )
  }

  let window = windows.get_primary().unwrap();
  let mut width = window.width() - SIDE_PANEL_WIDTH;
  let mut height = window.height();
  if width > height
  {
    width = height;
  }
  if width < height
  {
    height = width;
  }
  for ( pos, mut transform ) in q.iter_mut()
  {
    transform.translation = Vec3::new
    (
      0.1 * width + ( convert( pos.x as f32, width as f32, DESK_WIDTH as f32 ) - pos.x as f32 * 0.02 * width ),
      0.1 * height + ( convert( pos.y as f32, height as f32, DESK_HEIGHT as f32 ) - pos.y as f32 * 0.02 * height ),
      0.0,
    );
  }
}
