use game_chess_core::*;

/*
cargo test test_trivial -- --show-output
cargo test test_export_and_import -- --show-output
*/

#[test]
fn test_trivial()
{
  let mut game = Game::default();
  let target_move = "a2a4";
  game.board_print();
  game.make_move( target_move );
  game.board_print();
  assert_eq!( game.status(), GameStatus::Continuing );
  assert_eq!( game.last_move().unwrap(), target_move );

}

#[test]
fn test_export_and_import()
{
  let mut game = Game::default();
  game.make_move( "a2a4" );

  let serialized = serde_json::to_value( &game ).unwrap();
  let deserialized: Game = serde_json::from_value( serialized ).unwrap();

  assert_eq!( deserialized.last_move().unwrap(), game.last_move().unwrap() );
}