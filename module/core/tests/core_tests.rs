use std::path::Path;
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
  game.make_move(target_move);
  game.board_print();
  assert_eq!(game.status(), GameStatus::Continuing);
  assert_eq!(game.last_move().unwrap(), target_move);
}

#[test]
fn test_board_to_fen()
{
  let mut board = Board::default();
  board = board.make_move("a2a4").unwrap();
  assert_eq!(
    board.to_fen(),
    "rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq - 0 1".to_string()
  );
}

#[test]
fn test_board_from_fen()
{
  //src is board after "a2a4" move from starting position
  let src = "rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq - 0 1".to_string();
  let board = Board::from_fen(&src);
  assert_eq!(board.to_fen(), src);
}

#[test]
fn test_game_import()
{
   let src = r#"{"board":"rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq - 0 1","history":[{"fen":"rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq - 0 1","uci_move":"a2a4"}],"date":{"secs_since_epoch":1640446438,"nanos_since_epoch":529150000}}"#;  let game : Game = serde_json::from_str(src).unwrap();
  assert_eq!(game.last_move().unwrap(), "a2a4");
}

#[test]
fn test_game_export()
{
  let mut game = Game::default();
  game.make_move("a2a4");
  let serialized = serde_json::to_string(&game);
  assert_eq!(serialized.is_ok(), true);
}

#[test]
fn test_print_board()
{
  let board = Board::default();
  let board_str = board.to_pretty_string();
  assert_eq!(
    board_str,
    "8 | r n b q k b n r \n\
     7 | p p p p p p p p \n\
     6 | - - - - - - - - \n\
     5 | - - - - - - - - \n\
     4 | - - - - - - - - \n\
     3 | - - - - - - - - \n\
     2 | P P P P P P P P \n\
     1 | R N B Q K B N R \n  ------------------\n    a b c d e f g h"
  );
}
