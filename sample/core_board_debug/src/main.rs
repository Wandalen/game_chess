
/*
How to get board debug info
*/

pub fn main()
{
  /*
  https://docs.rs/pleco/0.5.0/pleco/
  https://docs.rs/pleco/0.5.0/pleco/board/struct.Board.html
  */

  let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2";
  let board: pleco::Board = pleco::Board::from_fen( fen ).unwrap();
  // board.print_debug_info(); //Prints Debug Information.
  // board.pretty_print(); //Prints a prettified representation of the board.
  board.fancy_print(); //Print the board alongside useful information.
}

/*
White Pinners
00000000
00000000
00000000
00000000
00000000
00000000
00000000
00000000

Black Pinners
00000000
00000000
00000000
00000000
00000000
00000000
00000000
00000000

White Blockers
00000000
00000000
00000000
00000000
00000000
00000000
00000000
00000000

Black Blockers
00000000
00000000
00000000
00000000
00000000
00000000
00000000
00000000

Checkers
00000000
00000000
00000000
00000000
00000000
00000000
00000000
00000000

Bishop check sqs
00000000
00010100
00000000
00000000
00000000
00000000
00000000
00000000

Rook check sqs
00010100
00001000
00000000
00000000
00000000
00000000
00000000
00000000

Queen check sqs
00010100
00011100
00000000
00000000
00000000
00000000
00000000
00000000
*/

/*
r n b q k b n r
p p - p p p p p
- - - - - - - -
- - p - - - - -
- - - - P - - -
- - - - - - - -
P P P P - P P P
R N B Q K B N R
*/

/*
r n b q k b n r
p p - p p p p p
- - - - - - - -
- - p - - - - -
- - - - P - - -
- - - - - - - -
P P P P - P P P
R N B Q K B N R

Castling bits: 1111, Rule 50: 0, ep_sq: c6
Total Moves: 2, ply: 0, depth: 0
Zobrist: ae62b1babbb9f866
*/