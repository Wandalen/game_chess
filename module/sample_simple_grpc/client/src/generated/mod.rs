pub mod chess_example;

use std::fmt::{Display, Formatter};

use crate::generated::chess_example::MoveResultResponse;

impl Display for MoveResultResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "MoveResultResponse {{ status: {}, message: {:?} }}",
            self.status, self.message
        ))
    }
}
