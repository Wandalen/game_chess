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
fn test_board_to_fen()
{
  let mut board = Board::default();
  board = board.make_move( "a2a4" ).unwrap();
  assert_eq!( board.to_fen(), "rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq - 0 1".to_string() );
}

// #[test]
// fn test_board_from_fen()
// {
//   //src is board after "a2a4" move from starting position
//   let src = "rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq - 0 1".to_string();
//   let board = Board::from_fen( &src );
//   assert_eq!( board.to_fen(), src );

// }

// #[test]
// fn test_game_import()
// {
//   return;
//   let src = r#"{"board":"rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq - 0 1","history":[{"fen":"rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq - 0 1","uci_move":"a2a4"}]}"#;
//   let game: Game = serde_json::from_str( src ).unwrap();
//   assert_eq!( game.last_move().unwrap(), "a2a4" );
// }

// #[test]
// fn test_game_export()
// {
//   let mut game = Game::default();
//   game.make_move( "a2a4" );
//   let serialized = serde_json::to_string( &game );
//   assert_eq!( serialized.is_ok(), true );
// }