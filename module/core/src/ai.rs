//!
//! Implement ai for the chess game.
//! Wraps pleco bots in dyn traits. Serializes algorithms.
//!

use super::{Board, Move};

use pleco::tools::Searcher;
use serde::{Serialize, Deserialize, Serializer, Deserializer};

///
/// AI algorithm. Implements gameplay strategy.
///

pub trait Algorithm: Send + Sync
{
  ///
  /// Pleco bot name
  ///
  fn name(&self) -> &'static str;

  ///
  /// Short name for reference
  ///
  fn short_name(&self) -> &'static str;

  ///
  /// Calculates the best move with depth
  ///
  fn best_move(&self, board : Board, depth : u16) -> Move;
}

macro_rules! implement_algorithm_trait {
  ($name:ident, $searcher:ty, $short_name:expr) => {
    struct $name;
    impl Algorithm for $name
    {
      fn name(&self) -> &'static str { <$searcher>::name() }

      fn short_name(&self) -> &'static str { $short_name }

      fn best_move(&self, board : Board, depth : u16) -> Move { <$searcher>::best_move(board.pleco_board, depth) }
    }
  };
}

implement_algorithm_trait!(MinMaxAlgorithm, pleco::bots::ParallelMiniMaxSearcher, "min_max");
implement_algorithm_trait!(IterativeAlgorithm, pleco::bots::IterativeSearcher, "iterative");
implement_algorithm_trait!(RandomAlgorithm, pleco::bots::RandomBot, "random");

///
/// Encapsulates all data required for AI algorithms to work.
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Engine
{
  #[serde(serialize_with = "ai_ser", deserialize_with = "ai_der")]
  algorithm : Box<dyn Algorithm>,
  ///
  /// Depth of calculation of moves by the engine
  ///
  pub depth : u16,
}

///
/// Error during creating of AI Algorithm
///
#[derive(Debug)]
pub enum CreationError
{
  ///
  /// Unrecognised algorithm name
  ///
  UnknownAlgorithm,
}

impl Engine
{
  ///
  /// Create new engine by name
  ///
  pub fn new(name : String) -> Result<Self, CreationError> { Self::new_with_depth(name, 5) }

  ///
  /// Create new engine by name and depth
  ///
  pub fn new_with_depth(name : String, depth : u16) -> Result<Self, CreationError>
  {
    let algorithm = Self::new_algorithm(name)?;
    Ok(Engine { algorithm, depth })
  }

  fn new_algorithm(name : String) -> Result<Box<dyn Algorithm>, CreationError>
  {
    match name.as_str()
    {
      "min_max" => Ok(Box::new(MinMaxAlgorithm {})),
      "iterative" => Ok(Box::new(IterativeAlgorithm {})),
      "random" => Ok(Box::new(RandomAlgorithm {})),
      _ => Err(CreationError::UnknownAlgorithm),
    }
  }

  ///
  /// Find best move on board
  ///
  pub fn best_move(&self, board : Board) -> Move { self.algorithm.best_move(board, self.depth) }
}

impl std::default::Default for Box<dyn Algorithm>
{
  fn default() -> Box<dyn Algorithm> { Box::new(IterativeAlgorithm {}) }
}

impl core::fmt::Debug for dyn Algorithm
{
  fn fmt(&self, f : &mut core::fmt::Formatter<'_>) -> core::fmt::Result { write!(f, "ai::Algorithm{{{}}}", self.short_name()) }
}

///
/// Serialize ai::Algorithm to string.
///

pub fn ai_ser<S : Serializer>(algorithm : &Box<dyn Algorithm>, s : S) -> Result<S::Ok, S::Error>
{
  s.serialize_str(&algorithm.short_name())
}

///
/// Deserialize ai::Algorithm from string.
///

pub fn ai_der<'de, D : Deserializer<'de>>(d : D) -> Result<Box<dyn Algorithm>, D::Error>
{
  let short_name : String = Deserialize::deserialize(d)?;
  Ok(Engine::new_algorithm(short_name).unwrap_or_default())
}
