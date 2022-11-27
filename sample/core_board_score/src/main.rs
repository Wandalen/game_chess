/*
How to calculate board score
*/

pub fn main() {
  /*
  https://docs.rs/tanton/latest/tanton/
  https://docs.rs/tanton/latest/tanton/board/struct.Board.html
  https://docs.rs/tanton/latest/tanton/board/struct.Board.html#bitboard-representation
  https://docs.rs/tanton/latest/tanton/tools/eval/struct.Eval.html#method.eval_low
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

  let board: tanton::Board = tanton::Board::from_fen("3qkb1r/3ppp2/3r1np1/2Q4p/5P2/1P3B2/P1P1PP1P/R2NK2R b k - 0 22").unwrap();

  board.pretty_print();

  let board_score = tanton::tools::eval::Eval::eval_low(&board); // Evaluates board score

  println!("Board score: {}", board_score);

  /*
    Negative score value - black advantage
    Positive score value - white advantage
    Zero score - equal
  */
}
