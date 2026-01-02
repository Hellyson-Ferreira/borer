use crate::error::ProtocolError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TunnelHttpRequest {
    pub method: String,
    pub path: String,
    pub query: Option<String>,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TunnelHttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TunnelMessage {
    HttpRequest(TunnelHttpRequest),
    HttpResponse(TunnelHttpResponse),
    Error { message: String },
}

impl TunnelMessage {
    pub fn to_bytes(&self) -> Result<Vec<u8>, ProtocolError> {
        let bytes = serde_json::to_vec(self)?;
        Ok(bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ProtocolError> {
        let msg = serde_json::from_slice(bytes)?;
        Ok(msg)
    }
}
