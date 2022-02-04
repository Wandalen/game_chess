fn main() -> Result<(), Box<dyn std::error::Error>>
{
  tonic_build::configure()
  .out_dir("src/generated")
  .compile(&["../proto/chess_game.proto"], &["../proto"])
  .unwrap();
  Ok(())

}
