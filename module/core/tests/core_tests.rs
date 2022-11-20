use game_chess_core::*;

/*
cargo test test_trivial -- --show-output
cargo test test_export_and_import -- --show-output
*/

#[ test ]
fn test_trivial()
{
  let mut game = Game::default();
  let target_move = "a2a4";
  game.board_print();
  game.make_move( target_move.into() );
  game.board_print();
  assert_eq!( game.status(), GameStatus::Continuing );
  assert_eq!( game.last_move().unwrap().0, target_move );
}

#[ test ]
fn test_board_to_fen()
{
  let mut board = Board::default();
  board = board.make_move( "a2a4".into() ).unwrap();
  assert_eq!
  (
    *board.to_fen(),
    "rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq - 0 1".to_string()
  );
}

#[ test ]
fn test_board_from_fen()
{
  //src is board after "a2a4" move from starting position
  let src = "rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq - 0 1".to_string();
  let board = Board::from_fen( &Fen::from( src.clone() ) );
  assert_eq!( *board.to_fen(), src );
}

#[ test ]
fn test_game_import()
{
  let src = r#"{"board":"rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq - 0 1","history":[{"fen":"rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq - 0 1","last_move":5640}],"date":{"secs_since_epoch":1643988263,"nanos_since_epoch":27317000},"is_forfeited":false}"#;
  let game : Game = serde_json::from_str( src ).unwrap();
  assert_eq!( game.last_move().unwrap().0, "a2a4" );
  assert_eq!( game.last_move_raw().unwrap().get_raw(), 5640 );
}

#[ test ]
fn test_game_export()
{
  let mut game = Game::default();
  game.make_move( "a2a4".into() );
  let serialized = serde_json::to_string( &game );
  assert_eq!( serialized.is_ok(), true );
}

#[ test ]
fn test_print_board()
{
  let board = Board::default();
  let board_str = board.to_pretty_string();
  assert_eq!
  (
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

#[ test ]
fn test_make_random_move()
{
  let mut game = Game::default();
  game.board_print();
  let turn_befor = game.current_turn();
  assert!( game.make_random_move() );
  game.board_print();
  assert_eq!( game.status(), GameStatus::Continuing );
  let turn_after = game.current_turn();
  assert_ne!( turn_befor, turn_after );
}

#[ test ]
fn test_list_saved_games()
{
  use std::fs; 
  let mut game = Game::default();
  assert_eq!( list_saved_games(), None );
  game.save();
  assert!( list_saved_games().is_some() );
  let list = list_saved_games().unwrap();
  println!( "{}", list[ 0 ].display() );
  assert!( list[ 0 ].display().to_string().contains( ".save" ) );
  fs::remove_dir_all( "saves" ).unwrap();
} 

#[ test ]
fn test_move_undo()
{
  let mut game = Game::default();
  assert!( game.get_history_idx().is_none() );
  assert!( game.make_random_move() );
  assert!( game.make_random_move() );
  assert!( game.make_random_move() );
  game.move_undo();
  let idx_1 = game.get_history_idx();
  assert!( game.get_history_idx().is_some());
  game.move_undo();
  let idx_2 = game.get_history_idx();
  assert!( idx_1 > idx_2 );
} 
