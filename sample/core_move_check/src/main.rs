/*
How to get check if move is valid
*/

pub fn main() {
  /*
  https://docs.rs/tanton/latest/tanton/
  https://docs.rs/tanton/latest/tanton/board/struct.Board.html
  https://docs.rs/tanton/latest/tanton/board/struct.Board.html#bitboard-representation
  https://docs.rs/tanton/latest/tanton/board/struct.Board.html#method.legal_move
  https://docs.rs/tanton/latest/tanton/board/struct.Board.html#method.pseudo_legal_move
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

  let board: tanton::Board = tanton::Board::default();
  let invalid_move = tanton::BitMove::make_quiet(tanton::SQ(8), tanton::SQ(9)); //Creates move from 8 square to square 9
  let valid_move = tanton::BitMove::make_quiet(tanton::SQ(8), tanton::SQ(16)); //Creates move from 8 square to square 16

  println!(
    "Move {} is valid: {}",
    invalid_move,
    board.pseudo_legal_move(invalid_move) && board.legal_move(invalid_move)
  );
  println!(
    "Move {} is valid: {}",
    valid_move,
    board.pseudo_legal_move(valid_move) && board.legal_move(valid_move)
  );
}
