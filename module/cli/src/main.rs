use game_chess_core::*;

fn main()
{
  let mut game: Option< Game > = None;
  let mut choice;

  command_help();

  loop
  {
    println!( "" );

    choice = wca::input::ask( "\nPlease enter command" );

    match choice.to_lowercase().trim()
    {
      ".game.new" => { game = Some( command_game_new() ) },
      ".move" => command_move( &mut game ),
      ".status" => command_status( &game ),
      ".quit" => command_exit(),
      ".help" => command_help(),
      command => println!( "Unknown command : {}\n", command ),
    }

  }

}

fn command_help()
{
  println!( "" );

  println!( "Commands:" );

  println!( "" );

  println!( ".game.new => Create game with default board" );
  println!( ".move     => Make a move by providing move in UCI format: \"a2a4\" " );
  println!( ".status   => Print board, current turn, last move" );
  println!( ".quit     => Exit from the game" );
  println!( ".help     => Print this help" );
}

fn command_exit()
{
  println!( "Exiting.." );
  std::process::exit( 0 );
}

fn command_game_new() -> Game
{
  let game = Game::default();
  println!( "" );
  game.board_print();
  println!( "Turn of {}", game.current_turn() );
  game
}

fn command_status( game : &Option<Game> )
{
  if game.is_none()
  {
    println!( "Create a game first. Use command: .game.new" );
    return;
  }

  let game = game.as_ref().unwrap();

  println!( "" );

  game.board_print();

  println!( "Current turn: {}", game.current_turn() );

  match game.last_move()
  {
    Some( m ) => println!( "Last move: {}", m ),
    _ => println!( "Last move: None" ),
  }
}

fn command_move( game : &mut Option<Game>  )
{
  if game.is_none()
  {
    println!( "Create a game first. Use command: .game.new" );
    return;
  }

  let game = game.as_mut().unwrap();

  let uci_move = wca::input::ask( "Provide move in UCI format, for example 'a2a4'" );
  if !game.make_move( uci_move.as_str() )
  {
    println!( "\n\x1b[93mFailed to apply move: '{}'. Try again!\x1b[0m", uci_move );
  }
  println!( "" );
  game.board_print();
  println!( "Turn of {}", game.current_turn() );
}