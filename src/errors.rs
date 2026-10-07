// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

use std::{io, net::SocketAddr};

type SourceErr = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("failed to bind {addr}")]
    Bind {
        addr: SocketAddr,
        #[source]
        source: io::Error,
    },

    #[error("failed to accept connection")]
    Accept(#[source] io::Error),

    #[error("websocket handshake failed")]
    Handshake(#[source] SourceErr),

    #[error("websocket read error")]
    Read(#[source] SourceErr),

    #[error("websocket write error")]
    Write(#[source] SourceErr),

    #[error("invalid JSON message")]
    InvalidJson(#[source] serde_json::Error),

    #[error("binary WebSocket messages are not supported")]
    BinaryMessage,
}

pub type Result<T> = std::result::Result<T, Error>;
