#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// #[derive(Debug, Error)]
// pub enum Error {
//     #[error("I/O error: {0}")]
//     Io(#[source] std::io::Error),
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Severity {
    Notice,
    Error,
    Fatal,
}
