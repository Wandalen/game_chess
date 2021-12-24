
/*
How to get best move for current state of the board
*/

use pleco::tools::Searcher;

pub fn main()
{
  /*
  https://docs.rs/pleco/0.5.0/pleco/
  https://docs.rs/pleco/0.5.0/pleco/board/struct.Board.html
  https://docs.rs/pleco/latest/pleco/board/struct.Board.html#bitboard-representation
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

  let board: pleco::Board = pleco::Board::from_fen( "7k/1pR2P2/N1pb1K2/2B3N1/1P5p/3q1p2/2p5/7B w - - 0 1").unwrap(); //Board from generated fen
  board.pretty_print();
  let depth = 1;
  let best_move = pleco::bots::MiniMaxSearcher::best_move( board.clone(), depth );// Return best move for current state of the board
  println!( "Best move: {}", best_move );
}
