//!
//! RPC server that provides game API.
//!

use std::collections::HashMap;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::{ Arc, Mutex };
use futures::Stream;


use tonic::{ Request, Response, Status };
use tokio::sync::mpsc;

use game_chess_core::{ UCI, Player };
use multiplayer::{ MultiplayerStatus, MultiplayerMessage };
use multiplayer::generated::chess::chess_server::Chess;
use crate::store::GameStore;
use multiplayer::generated::chess::
{
  self, Board, GameState, Games, CreateGame, GameId, AcceptGame, GameMove, GamePlayer, Msg, Msgs, GameAvailableMoves,
};
use crate::store::memory::MemoryStore;

type ResponseStream< T > = Pin< Box< dyn Stream< Item = Result< T, Status > > + Send > >;

///
/// Shared sever.
///
#[ allow( missing_debug_implementations, dead_code ) ]
pub struct ChessRpcServer
{
  store : Arc< Mutex< dyn GameStore + Send + Sync > >,
  streams : Arc< Mutex< HashMap< SocketAddr, mpsc::UnboundedSender< Result< chess::GameUpdate, Status > > > > >,
}

impl ChessRpcServer
{
  ///
  /// Server constructor.
  ///
  pub fn init() -> Self
  {
    Self 
    {
      store : Arc::new( Mutex::new( MemoryStore::new() ) ),
      streams : Arc::new( Mutex::new( HashMap::new() ) ),
    }
  }
}

impl ChessRpcServer
{
  /// Broadcasts `GameUpdate` to all clients.
  pub async fn push_game_update( &self, game_update : chess::game_update::GameUpdate )
  {
    let streams_lock = self.streams.lock().expect( "Failed to lock the streams mutex" );
    for ( client_addr, stream ) in streams_lock.iter()
    {
      let game_update = chess::GameUpdate 
      {
        game_update : Some( game_update.clone() ),
      };
      if let Err( err ) = stream.send( Ok( game_update ) )
      {
        eprintln!( "Failed to send GameUpdate to {}: {}", client_addr, err );
      }
    }
  }
}

#[ tonic::async_trait ]
impl Chess for ChessRpcServer
{
  #[ allow( non_camel_case_types ) ]
  type pull_game_updatesStream = ResponseStream< chess::GameUpdate >;

  ///
  /// Apply request to create new game.
  ///
  async fn push_game_create( &self, request : Request< CreateGame > ) -> Result< Response< GameId >, Status >
  {
    let mut store = self.store.lock().expect( "Failed to lock the store mutex" );

    if let Some( player ) = request.into_inner().player
    {
      let new_game = store.add_game( multiplayer::MultiplayerGame::new
      (
        player.game_id.to_string(),
        GamePlayer 
        {
          game_id : player.game_id.to_string(),
          player_id : player.player_id,
        },
        MultiplayerStatus::NotStarted as i32,
      ) );

      if let Err( e ) = new_game
      {
        Err( Status::already_exists( e ) )
      }
      else
      {
        Ok( Response::new( GameId 
        {
          game_id : player.game_id,
        } ) )
      }
    }
    else
    {
      Err( Status::invalid_argument( "No player found!" ) )
    }
  }

  ///
  /// Accept request to join game.
  ///
  async fn push_game_accept( &self, request : Request< AcceptGame > ) -> Result< Response< GameId >, Status >
  {
    let game_req = request.into_inner();
    let input_game_id = game_req.game_id;

    let mut store = self.store.lock().expect( "Failed to lock the store mutex" );

    if let Some( player ) = game_req.player_id
    {
      let input_player_id = player.player_id;

      let mut game = match store.get_game( &input_game_id )
      {
        Some( game ) =>
        {
          if game.players.len() < 2
          {
            multiplayer::MultiplayerGame::new
            (
              game.game_id.clone(),
              GamePlayer 
              {
                game_id : game.players[ 0 ].game_id.clone(),
                player_id : game.players[ 0 ].player_id.clone(),
              },
              MultiplayerStatus::Started as i32,
            )
          }
          else
          {
            let other_player = &game.players[ 1 ].player_id;
            let err_msg = format!( "Oops! Game ID: {input_game_id} is already joined by {other_player}" );
            return Err( Status::not_found( err_msg ) );
          }
        }
        None =>
        {
          let err_msg = format!( "No game found by the Game ID: {input_game_id}" );
          return Err( Status::not_found( err_msg ) );
        }
      };

      game.add_opponent( GamePlayer 
      {
        game_id : input_game_id.clone(),
        player_id : input_player_id,
      } );

      store.update_game( &input_game_id, game );
      Ok( Response::new( GameId { game_id : input_game_id } ) )
    }
    else
    {
      Err( Status::not_found( "No player found on input!" ) )
    }
  }

  ///
  /// Apply move.
  ///
  async fn push_move( &self, request : Request< GameMove > ) -> Result< Response< Board >, Status >
  {
    let move_req = request.into_inner();
    let ( game_id, player_id, r#move ) = ( move_req.game_id, move_req.player_id, move_req.r#move );

    let mut store = self.store.lock().expect( "Failed to lock the store mutex" );
    if store.move_validity( &game_id, &r#move )
    {
      // Check for legal player move
      let current_turn = store.current_turn( &game_id );

      let game = match store.get_game( &game_id )
      {
        Some( game ) => game,
        None =>
        {
          let err_msg = format!( "No game found by the Game ID: {game_id}" );
          return Err( Status::not_found( err_msg ) );
        }
      };

      let mut turn_msg = String::new();
      let illegal_turn_msg = "Illegal move! Now is your opponent's turn";

      match current_turn
      {
        Player::White =>
        {
          if game.players[ 0 ].player_id == player_id
          {
            store.make_move( &game_id, &r#move );
          }
          else
          {
            turn_msg = illegal_turn_msg.to_string()
          }
        }
        Player::Black =>
        {
          if game.players[ 0 ].player_id != player_id
          {
            store.make_move( &game_id, &r#move );
          }
          else
          {
            turn_msg = illegal_turn_msg.to_string()
          }
        }
      }

      let mut board_state = store.get_board_state( &game_id ).unwrap();

      if turn_msg.is_empty()
      {
        let current_turn = store.current_turn( &game_id );
        let last_move = store.last_move( &game_id );
        default_board_view( &mut board_state, current_turn, last_move );

        Ok( Response::new( Board { game_id, board_state } ) )
      }
      else
      {
        Ok( Response::new( Board 
        {
          game_id,
          board_state : turn_msg,
        } ) )
      }
    }
    else
    {
      Ok( Response::new( Board 
      {
        game_id,
        board_state : "Invalid move! For all available moves use command: .moves.list".to_owned(),
      } ) )
    }
  }

  ///
  /// Get available moves.
  ///
  async fn pull_moves( &self, request : Request< GameId > ) -> Result< Response< GameAvailableMoves >, Status >
  {
    let game_id = request.into_inner().game_id;
    let store = self.store.lock().expect( "Failed to lock the store mutex" );

    // Assumes game already exists
    let moves_list = store.moves_list( &game_id) ;
    let moves_list : Vec< String > = moves_list.iter().map( | r#move | r#move.to_string() ).collect();
    Ok( Response::new( GameAvailableMoves { moves_list } ) )
  }

  ///
  /// Get info about current board state. There are only positions.
  ///
  async fn pull_board_state( &self, request : Request< GameId > ) -> Result< Response< Board >, Status >
  {
    let game_id = request.into_inner().game_id;
    let store = self.store.lock().expect( "Failed to lock the store mutex" );

    if let Some( mut board_state ) = store.get_board_state( &game_id )
    {
      let current_turn = store.current_turn( &game_id );
      let last_move = store.last_move( &game_id );
      default_board_view( &mut board_state, current_turn, last_move );

      Ok( Response::new( Board { game_id, board_state } ) )
    }
    else
    {
      Err( Status::not_found( format!( "No game found by the Game ID: {}", game_id ) ) )
    }
  }

  ///
  /// Get info about current game state - positions and history.
  ///
  async fn pull_game_state( &self, _request : Request< GameId > ) -> Result< Response< GameState >, Status > { todo!() }

  ///
  /// Get list of games.
  ///
  async fn pull_games_list( &self, _request : Request< () > ) -> Result< Response< Games >, Status >
  {
    let store = self.store.lock().expect( "Failed to lock the store mutex" );
    let games = store.get_games();

    if games.is_empty()
    {
      Err( Status::not_found( "No game found on server!" ) )
    }
    else
    {
      Ok( Response::new( Games { games : games.clone() } ) )
    }
  }

  ///
  /// Send request to forfeit.
  ///
  async fn push_game_gg( &self, _request : Request< GamePlayer > ) -> Result< Response< () >, Status >
  {
    let message = _request.into_inner();
    let game_id = message.game_id;
    let player_id = message.player_id;

    let _winner = 
    {
      let mut memory_store = self.store.lock().unwrap();
      let mut winner = None;
      let mut current_game = memory_store.get_game( &game_id ).unwrap().clone();

      memory_store.update_game( &game_id, current_game.clone() );
      for player in &current_game.players
      {
        if player.player_id != player_id
        {
          winner = Some( player.clone() );
          break;
        }
      }

      current_game.status = MultiplayerStatus::Ended as i32;
      winner
    };
    // Will be moved from function push_mgs
    //
    // let request;
    // if let Some(winner) = winner {
    //   let msg = format!("The player {} gave up. The player {} is the winner!", player_id, winner.player_id);
    //   request = Request::new(Msg {
    //     player: Some(message),
    //     text: msg.into(),
    //   });
    // }

    // return self.push_mgs(request).await;

    Ok( Response::new( () ) )
  }

  ///
  /// Send message to game chat.
  ///
  async fn push_msg( &self, request : Request< Msg > ) -> Result< Response< () >, Status >
  {
    let msg_req = request.into_inner();
    let player = msg_req.player;

    if let Some( player ) = player
    {
      let msg = MultiplayerMessage::new( player.player_id, msg_req.text );

      let mut store = self.store.lock().expect( "Failed to lock the store mutex" );
      store.add_chat( &player.game_id, msg );

      Ok( Response::new( () ) )
    }
    else
    {
      Err( Status::not_found( "No player found on input!" ) )
    }
  }

  ///
  /// Read messages from game chat.
  ///
  async fn read_msgs( &self, request : Request< GamePlayer > ) -> Result< Response< Msgs >, Status >
  {
    let player = request.into_inner();
    let mut msgs = Msgs { messages : Vec::new() };

    let store = self.store.lock().expect( "Failed to lock the store mutex" );

    // Does not guarantees ordered message
    let chats = store.get_chats( &player.game_id, &player.player_id );
    for msg in chats.iter()
    {
      msgs.messages.push( msg.pretty_print() );
    }

    if msgs.messages.is_empty()
    {
      msgs.messages.push( "No Chat Messages!".to_owned() );
    }
    Ok( Response::new( msgs ) )
  }

  async fn pull_game_updates( &self, request : Request< GameId > ) -> Result< Response< Self::pull_game_updatesStream >, Status >
  {
    let mut streams = self.streams.lock().expect( "Failed to lock the streams mutex" );
    let ( tx, rx ) = mpsc::unbounded_channel();
    streams.insert( request.remote_addr().expect( "Expected a client address" ), tx );

    Ok( Response::new
    (
      Box::pin( tokio_stream::wrappers::UnboundedReceiverStream::new( rx ) ) as Self::pull_game_updatesStream,
    ) )
  }
}

fn default_board_view( board : &mut String, turn : Player, r#move : Option< UCI > )
{
  let turn_msg = format!( "\n\nCurrent turn: {} - ", turn );
  let last_move = if let Some( last_move ) = r#move
  {
    format!( "Last move: {}", last_move.0 )
  }
  else
  {
    "Enjoy the game!".to_owned()
  };

  board.push_str( &turn_msg );
  board.push_str( &last_move );
}
