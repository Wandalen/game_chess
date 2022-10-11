#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Chess game implemented on Bevy for educational purpose.
//!

use bevy::
{
  prelude::*,
  core_pipeline::core_2d::graph::NAME,
  render::
  {
    camera::
    {
      Camera,
      CameraRenderGraph,
      CameraProjection,
      DepthCalculation,
      camera_system,
    },
    primitives::Frustum,
    view::VisibleEntities,
  },
  sprite::MaterialMesh2dBundle,
  window::close_on_esc,
};

///
/// Main.
///

#[ derive( Component, Debug, Clone, Reflect ) ]
#[ reflect( Component ) ]
pub struct ChessProjection
{
  /// Offset from left side.
  pub left : f32,
  /// Offset from right side.
  pub right : f32,
  /// Offset from bottom.
  pub bottom : f32,
  /// Offset from top.
  pub top : f32,
  /// Near element for depth sorting.
  pub near : f32,
  /// Far element for depth sorting.
  pub far : f32,
  /// Scale of view.
  pub scale: f32,
  /// Depth calculation.
  pub depth_calculation: DepthCalculation,
}

impl CameraProjection for ChessProjection
{
  /// Transform positions points to projection matrix.
  fn get_projection_matrix( &self ) -> Mat4
  {
    Mat4::orthographic_rh( self.left, self.right, self.bottom, self.top, self.near, self.far )
  }

  /// Setup positions projection taking into account window size.
  fn update( &mut self, width : f32, height : f32 )
  {
    if width > height
    {
      /* if width > height we need to shrink left and right sides by delta */
      let delta = width / height - 1.0;
      self.left = -1.0 - delta;
      self.right = 1.0 + delta;
      self.top = 1.0;
      self.bottom = -1.0;
    }
    else
    {
      /* if width > height we need to shrink bottom and top by delta */
      let delta = height / width - 1.0;
      self.left = -1.0;
      self.right = 1.0;
      self.top = 1.0 + delta;
      self.bottom = -1.0 - delta;
    }
  }

  /// Far getter.
  fn far( &self ) -> f32
  {
    self.far
  }

  /// Sort entities by depth. Not used.
  fn depth_calculation( &self ) -> DepthCalculation
  {
    self.depth_calculation
  }
}

impl Default for ChessProjection
{
  /* Default settings. */
  fn default() -> Self
  {
    ChessProjection
    {
      left : -1.0,
      right : 1.0,
      bottom : -1.0,
      top : 1.0,
      near : 0.0,
      far : 1000.0,
      scale: 1.0,
      depth_calculation : DepthCalculation::Distance,
    }
  }
}

///
/// Alternative camera bundle that show up the game board.
///

#[ derive( Bundle, Default ) ]
pub struct ChessCameraBundle
{
  /// Instance of camera.
  pub camera : Camera,
  /// Instance of 2d camera.
  pub camera_2d : Camera2d,
  /// Render graph.
  pub camera_render_graph : CameraRenderGraph,
  /// Custom projection.
  pub projection : ChessProjection,
  /// Default settings for visible entities.
  pub visible_entities : VisibleEntities,
  /// Local transform.
  pub transform : Transform,
  /// Global transform.bevy::window::close_on_esc
  pub global_transform : GlobalTransform,
  /// Frustum of view.
  pub frustum : Frustum,
}

impl std::fmt::Debug for ChessCameraBundle
{
  fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
  {
    f.debug_struct( "ChessCameraBundle" )
    .field( "camera", &self.camera )
    .field( "camera_render_graph", &*self.camera_render_graph )
    .field( "projection", &self.projection )
    .field( "visible_entities", &self.visible_entities )
    .field( "transform", &self.transform )
    .field( "global_transform", &self.global_transform )
    .field( "frustum", &self.frustum )
    .finish()
  }
}

impl ChessCameraBundle
{
  /// ChessCameraBundle constructor.
  pub fn new() -> Self
  {
    let projection = ChessProjection::default();
    let transform = Transform::from_xyz( 0.0, 0.0, projection.far() - 0.1 );
    let view_projection = projection.get_projection_matrix() * transform.compute_matrix().inverse();
    let frustum = Frustum::from_view_projection
    (
      &view_projection,
      &transform.translation,
      &transform.back(),
      projection.far(),
    );

    ChessCameraBundle
    {
      projection,
      transform,
      frustum,
      camera : Default::default(),
      camera_render_graph : CameraRenderGraph::new( NAME ), // default graph name, required
      camera_2d : Default::default(),
      global_transform : Default::default(),
      visible_entities : Default::default(),
    }
  }
}

fn main()
{
  let mut app = App::new();
  /* default plugins */
  app.add_plugins( DefaultPlugins );
  /* background */
  app.insert_resource( ClearColor( Color::rgb( 0.9, 0.9, 0.9 ) ) );
  /* setup core */
  app.add_startup_system( graphics_setup );
  /* exit on escape */
  app.add_system( close_on_esc );
  app.add_system_to_stage
  (
    CoreStage::PostUpdate,
    camera_system::< ChessProjection >
  );
  /* run */
  app.run();
}

///
/// Graphics setup.
///

pub fn graphics_setup( mut commands : Commands, mut meshes : ResMut< Assets< Mesh > >, mut materials : ResMut< Assets< ColorMaterial > > )
{
  /* camera */
  commands.spawn_bundle( ChessCameraBundle::new() );

  let material = materials.add( ColorMaterial::from( Color::rgb( 0.2, 0.2, 0.1 ) ) );

  let mesh = shape::Quad::new( Vec2::new( 1.0, 1.0 ) );

  let transform = Transform
  {
    translation : Vec3::new( -0.5, -0.5, 0.0 ),
    ..Default::default()
  };

  commands.spawn_bundle( MaterialMesh2dBundle
  {
    mesh : meshes.add( mesh.into() ).into(),
    material : material.clone(),
    transform,
    ..Default::default()
  } );

  //

  let mesh = shape::Quad::new( Vec2::new( 1.0, 1.0 ) );

  let transform = Transform
  {
    translation : Vec3::new( 0.5, 0.5, 0.0 ),
    ..Default::default()
  };

  commands.spawn_bundle( MaterialMesh2dBundle
  {
    mesh : meshes.add( mesh.into() ).into(),
    material,
    transform,
    ..Default::default()
  } );
}
