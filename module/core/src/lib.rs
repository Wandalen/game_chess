
pub use pleco;

pub use pleco::
{
  core::Piece,
  core::PieceType,
  core::Player,
  core::move_list::MoveList,
  core::piece_move::BitMove, //https://docs.rs/pleco/latest/pleco/core/piece_move/index.html,
  core::piece_move::MoveType,

  board::piece_locations::PieceLocations,
  board::castle_rights::Castling,
  board::Board, // Game struct
  board::board_state::BoardState,

  tools::eval::Eval,

  helper::Helper,
  helper::prelude,

  bot_prelude
};





pub fn sample()
{
  let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2";
  let board: pleco::Board = pleco::Board::from_fen( fen ).unwrap();
  board.pretty_print();
}

/*
cargo test -- --show-output
*/

#[test]
fn run_sample()
{
  sample();
}

#[test]
fn run_exp()
{
  let p = Piece::WhitePawn{};
  println!( "{:#?}", p.type_of() );
}