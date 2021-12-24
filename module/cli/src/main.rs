use game_chess_core::*;

fn main()
{
  let mut game: Game = Game::default();
  let mut choice;

  loop
  {
    println!( "" );

    game.board_print();

    println!( "Turn of {}", game.current_turn() );

    println!( "" );

    println!( "Commands:" );

    println!( "" );

    println!( ".game.new => Create game with default board" );
    println!( ".move     => Make a move by providing move in UCI format: \"a2a4\" " );
    println!( ".status   => Print board, current turn, last move" );
    println!( ".quit     => Exit from the game" );

    choice = wca::input::ask( "\nPlease enter command" );

    match choice.to_lowercase().trim()
    {
      ".game.new" => command_game_new( &mut game ),
      ".move" => command_move( &mut game ),
      ".status" => command_status( &game ),
      ".quit" => command_exit(),
      command => println!( "Unknown command : {}\n", command ),
    }

  }

}

fn command_exit()
{
  println!( "Exiting.." );
  std::process::exit( 0 );
}

fn command_game_new( game : &mut Game )
{
  *game = Game::default();
}

fn command_status( game : &Game )
{
  game.board_print();

  println!( "Current turn: {}", game.current_turn() );

  match game.last_move()
  {
    Some( m ) => println!( "Last move: {}", m ),
    _ => println!( "Last move: None" ),
  }
}

fn command_move( game : &mut Game )
{
  let uci_move = wca::input::ask( "Provide move in UCI format:" );
  game.make_move( uci_move.as_str() );
}