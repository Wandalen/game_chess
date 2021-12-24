
/*
How to calculate profitability of the next move
*/

use pleco::tools::Searcher;

pub fn main()
{
  /*
  https://docs.rs/pleco/0.5.0/pleco/
  https://docs.rs/pleco/0.5.0/pleco/board/struct.Board.html
  https://docs.rs/pleco/latest/pleco/board/struct.Board.html#bitboard-representation
  https://docs.rs/pleco/0.5.0/pleco/tools/eval/struct.Eval.html#method.eval_low
  https://docs.rs/pleco/latest/pleco/tools/trait.Searcher.html#tymethod.best_move
  */

  /*

  8 | 56 57 58 59 60 61 62 63
  7 | 48 49 50 51 52 53 54 55
  6 | 40 41 42 43 44 45 46 47
  5 | 32 33 34 35 36 37 38 39
  4 | 24 25 26 27 28 29 30 31
  3 | 16 17 18 19 20 21 22 23
  2 | 8  9  10 11 12 13 14 15
  1 | 0  1  2  3  4  5  6  7
    -------------------------
      a  b  c  d  e  f  g  h
  */

  let mut board: pleco::Board = pleco::Board::from_fen("3qkb1r/3ppp2/3r1np1/2Q4p/5P2/1P3B2/P1P1PP1P/R2NK2R b k - 0 22").unwrap();

  board.pretty_print();

  println!( "Turn of {}\n", board.turn() );// Returns current player

  board.apply_uci_move( "e7e6" ); //Apply desired turn to the board

  let desired_move_value = pleco::tools::eval::Eval::eval_low( &board );// Evaluates board score
  board.undo_move(); // Undo the desired move in order to apply best move on the original board

  let depth = 1;
  let best_move = pleco::bots::MiniMaxSearcher::best_move( board.clone(), depth );// Return best move for current state of the board
  board.apply_move( best_move ); //Apply best move to the board
  let best_move_value = pleco::tools::eval::Eval::eval_low( &board ); // Evaluates board score

  println!( "Desired move value: {}", desired_move_value );
  println!( "Best move value: {}", best_move_value );

  /*
    Negative score value - black advantage
    Positive score value - white advantage
    Zero score - equal
  */

  let profit = desired_move_value as f32 / best_move_value as f32 * 100.0; //Calculate profit in comparison to best move
  println!( "Profitability in comparison to best move: {:.0}%", profit );
}
