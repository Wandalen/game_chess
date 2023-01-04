/*
Sample of the chess game.
Player moves are generated.
Press Enter key to make a turn.
*/

pub fn main()
{
  /*
  https://docs.rs/pleco/0.5.0/pleco/
  https://docs.rs/pleco/0.5.0/pleco/board/struct.Board.html
  */

  use std::io::{ stdin, stdout, Read, Write };

  let mut stdout = stdout();
  let mut board : pleco::Board = pleco::Board::start_pos();

  let mut wait_for_enter_key = || 
  {
    stdout.write_all( b"Press Enter to make move for current player...\n" ).unwrap();
    stdout.flush().unwrap();
    stdin().read_exact( &mut [ 0 ] ).unwrap();
  };

  loop
  {
    println!( "\nBoard:" );

    board.pretty_print(); //Prints the board to the terminal

    let current_player = board.turn(); //Returns the Player whose turn it is to move.

    println!( "Current player: {}", current_player );

    /* Comment out call of `wait_for_enter_key` to disable wait for input */
    wait_for_enter_key();

    let current_player_moves = board.generate_moves(); //Generates a list of legal moves for current player
    let current_player_move = current_player_moves[ 0 ];

    println!( "Current player move: {}", current_player_move.stringify() );

    board.apply_move( current_player_move ); //Makes the move

    if board.checkmate()
    {
      println!( "\nCheckmate" );
      break;
    }

    if board.stalemate()
    {
      println!( "\nStalemate" );
      break;
    }
  }
}
