mod functions;

use std::{process, fmt::Debug};
pub use functions::*;

pub trait ExpectWith<T, E> {
    fn expect_with(self, msg: &str) -> T;
}

impl<T, E: Debug> ExpectWith<T, E> for Result<T, E> {
    fn expect_with(self, msg: &str) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}: {:?}", msg, e);
                process::exit(-1)
            }
        }
    }
}
