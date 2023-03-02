#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Chess game client implemented for educational purpose.
//!
mod client;

pub use client::*;
pub use multiplayer::generated::chess::{ GameId, CreateGame, AcceptGame, GamePlayer, Msg, GameMove, Board };