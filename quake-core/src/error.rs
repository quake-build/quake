pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// #[derive(Debug, Error)]
// pub enum Error {
//     #[error("I/O error: {0}")]
//     Io(#[source] std::io::Error),
// }
