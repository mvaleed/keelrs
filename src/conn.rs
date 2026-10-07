// -----------------------------------------------------------------------------
// Connection
//
// Owns everything related to WebSockets:
//
//     TCP -> WebSocket handshake
//     WebSocket reads
//     WebSocket writes
//     idle timeout
//     control frames
//     connection close
//
// The protocol/session layer doesn't need to know about tungstenite.
// -----------------------------------------------------------------------------

use crate::{Error, Message, Result, Session};

use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::time::timeout;

use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, accept_async};

struct Connection {
    ws: WebSocketStream<TcpStream>,
    idle_timeout: Duration,
}

impl Connection {
    async fn accept(stream: TcpStream, idle_timeout: Duration) -> Result<Self> {
        let ws = accept_async(stream)
            .await
            .map_err(|err| Error::Handshake(Box::new(err)))?;

        Ok(Self { ws, idle_timeout })
    }

    /// Receive the next application-level message.
    ///
    /// Ok(Some(message)) -> application message received
    /// Ok(None)          -> connection ended / idle timeout
    /// Err(...)          -> actual transport failure
    async fn recv(&mut self) -> Result<Option<Message>> {
        loop {
            let frame = match timeout(self.idle_timeout, self.ws.next()).await {
                // Nothing received before the idle timeout.
                Err(_) => return Ok(None),

                // WebSocket stream ended normally.
                Ok(None) => return Ok(None),

                // Actual WebSocket read failure.
                Ok(Some(Err(err))) => {
                    return Err(Error::Read(Box::new(err)));
                }

                Ok(Some(Ok(frame))) => frame,
            };

            // The peer explicitly closed the connection.
            if frame.is_close() {
                return Ok(None);
            }

            // Text/Binary become protocol messages.
            //
            // Ping/Pong and other control frames aren't application messages,
            // so just keep waiting for the next frame.
            if let Some(message) = Message::from_ws(frame)? {
                return Ok(Some(message));
            }
        }
    }

    async fn send(&mut self, message: Message) -> Result<()> {
        self.ws
            .send(message.into_ws())
            .await
            .map_err(|err| Error::Write(Box::new(err)))
    }

    async fn close(&mut self) {
        let _ = self.ws.close(None).await;
    }
}

// -----------------------------------------------------------------------------
// Connection lifecycle
//
// Deliberately boring.
//
// Connection handles transport.
// Session handles protocol.
// -----------------------------------------------------------------------------

pub(crate) async fn handle_conn(stream: TcpStream, idle_timeout: Duration) -> Result<()> {
    let mut conn = Connection::accept(stream, idle_timeout).await?;
    let mut session = Session::new();

    loop {
        let message = match conn.recv().await {
            Ok(Some(message)) => message,
            Ok(None) => break,
            Err(Error::InvalidJson(_)) => {
                conn.send(Message::from(serde_json::json!({
                    "error": "invalid JSON message"
                })))
                .await?;
                continue;
            }
            Err(Error::BinaryMessage) => {
                conn.send(Message::from(serde_json::json!({
                    "error": "binary messages are not supported"
                })))
                .await?;
                continue;
            }
            Err(err) => return Err(err),
        };

        if let Some(reply) = session.on_message(message).await {
            conn.send(reply).await?;
        }
    }

    conn.close().await;

    Ok(())
}
