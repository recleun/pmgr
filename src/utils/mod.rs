mod functions;
mod macros;

use std::fmt::Debug;
pub use functions::*;

pub trait ExpectWith<T, E> {
    fn expect_with(self, msg: &str) -> T;
}

impl<T, E: Debug> ExpectWith<T, E> for Result<T, E> {
    fn expect_with(self, msg: &str) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                panic!("{}: {:?}", msg, e);
            }
        }
    }
}
