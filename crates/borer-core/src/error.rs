use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("serialization error")]
    Serialization(#[from] serde_json::Error),
}
