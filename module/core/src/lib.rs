
pub use pleco;

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