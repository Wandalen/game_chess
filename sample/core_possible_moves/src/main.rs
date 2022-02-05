/*
How to get list of possible squares to move from a target square
*/

pub fn main()
{
  /*
  https://docs.rs/pleco/0.5.0/pleco/
  https://docs.rs/pleco/0.5.0/pleco/board/struct.Board.html
  https://docs.rs/pleco/0.5.0/pleco/board/struct.Board.html#bitboard-representation
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

  let board : pleco::Board = pleco::Board::default();
  board.pretty_print(); //Prints a prettified representation of the board.
  let legal_moves = board.generate_moves(); //All legal moves
  let from_square = pleco::SQ(8); //Source square
  let legal_moves_for_target = legal_moves.iter().filter(|mv| mv.get_src() == from_square); //Selects legal moves that have target as source
  for legal_move in legal_moves_for_target
  {
    println!("{}", legal_move.to_string());
  }
}
