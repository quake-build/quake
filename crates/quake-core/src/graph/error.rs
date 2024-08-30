use thiserror::Error;

#[derive(Debug, Error)]
pub enum GraphError<K> {
    #[error("invalid key: {0}")]
    InvalidKey(K),
}
