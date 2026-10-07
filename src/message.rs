// -----------------------------------------------------------------------------
// Protocol message
//
// This is our JSON application-level message type. The rest of the SDK
// doesn't need to know about tungstenite's Message type.
// -----------------------------------------------------------------------------
use crate::{Error, Result};
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message as WsMessage;

#[derive(Debug, Clone)]
pub struct Message(pub(crate) Value);

impl Message {
    /// Convert a WebSocket frame into an application message.
    ///
    /// Control frames such as Ping/Pong are valid WebSocket frames but aren't
    /// application messages, so they return None rather than an error.
    pub(crate) fn from_ws(message: WsMessage) -> Result<Option<Self>> {
        match message {
            WsMessage::Text(text) => serde_json::from_str(text.as_ref())
                .map(|value| Some(Self(value)))
                .map_err(Error::InvalidJson),
            WsMessage::Binary(_) => Err(Error::BinaryMessage),
            _ => Ok(None),
        }
    }

    pub(crate) fn into_ws(self) -> WsMessage {
        WsMessage::Text(serde_json::to_string(&self.0).unwrap().into())
    }
}

impl From<Value> for Message {
    fn from(value: Value) -> Self {
        Self(value)
    }
}
