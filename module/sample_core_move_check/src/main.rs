
/*
How to get check if move is valid
*/

pub fn main()
{
  /*
  https://docs.rs/pleco/0.5.0/pleco/
  https://docs.rs/pleco/0.5.0/pleco/board/struct.Board.html
  https://docs.rs/pleco/0.5.0/pleco/board/struct.Board.html#method.legal_move
  */

  let mut board: pleco::Board = pleco::Board::default();
  let moves = board.generate_moves();
  let move_str = moves[ 0 ].stringify();

  println!( "Move: {} is valid: {}", move_str, board.legal_move( moves[ 0 ] ) );

  println!( "\nApply move {} to the board\n", move_str );

  board.apply_move( moves[ 0 ] );

  board.pretty_print(); //Print the board

  println!( "Move: {} is valid: {}", move_str, board.legal_move( moves[ 0 ] ) );


}
