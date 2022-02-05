//
// use bevy::prelude::*;
// use bevy_text_mesh::*;
// use bevy::render::pass::ClearColor;
//
// fn setup_text_mesh( mut commands : Commands, asset_server : Res<AssetServer> )
// {
//   // load the font
//   let font: Handle<TextMeshFont> = asset_server.load( "fonts/FiraSans-Bold.ttf" );
//
//   commands.spawn_bundle( TextMeshBundle
//   {
//     text_mesh : TextMesh
//     {
//       text : String::from( "Text as mesh" ),
//       // the style of mesh
//       style : TextMeshStyle
//       {
//         font : font.clone(),
//         font_size : SizeUnit::NonStandard( 36.0 ),
//         color : Color::rgb( 0.0, 1.0, 0.0 ),
//         mesh_quality : Quality::Custom( 255 ),
//         ..Default::default()
//       },
//       ..Default::default()
//     },
//     // position relative to 3d camera
//     transform: Transform
//     {
//       translation : Vec3::new( -1.0, 1.3, 0.0 ),
//       ..Default::default()
//     },
//     ..Default::default()
//   });
// }
//
// //
//
// // setup scene to compare text with another object
// fn setup
// (
//   mut commands : Commands,
//   mut meshes : ResMut<Assets<Mesh>>,
//   mut materials : ResMut<Assets<StandardMaterial>>,
// )
// {
//   // add plane
//   commands.spawn_bundle( PbrBundle
//   {
//     mesh : meshes.add( Mesh::from( shape::Plane { size: 5.0 } ) ),
//     material : materials.add( Color::rgb( 0.3, 0.5, 0.3 ).into() ),
//     ..Default::default()
//   });
//   // add lightning of scene
//   commands.spawn_bundle( LightBundle
//   {
//     transform : Transform::from_xyz( 4.0, 8.0, 4.0 ),
//     ..Default::default()
//   });
//   // add perspective camera
//   commands.spawn_bundle( PerspectiveCameraBundle
//   {
//     transform: Transform::from_xyz( -2.0, 2.5, 5.0 ).looking_at( Vec3::ZERO, Vec3::Y ),
//     ..Default::default()
//   });
// }

fn main()
{
  println!( "Hello, world!" );
}
