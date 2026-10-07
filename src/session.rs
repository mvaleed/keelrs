// -----------------------------------------------------------------------------
// Session
//
// One Session exists per connection. All protocol/business logic belongs here.
// -----------------------------------------------------------------------------

use crate::Message;
use serde_json::Value;

pub(crate) struct Session {
    // Per-connection state goes here:
    //
    // authenticated: bool,
    // protocol_version: u32,
    // current_request_id: Option<String>,
}

impl Session {
    pub(crate) fn new() -> Self {
        Self {}
    }

    /// Handle one application-level message.
    ///
    /// Some(reply) -> send a response
    /// None        -> no response
    pub(crate) async fn on_message(&mut self, message: Message) -> Option<Message> {
        match message {
            Message(value) if value == Value::String("ping".into()) => {
                Some(Message::from(Value::String("pong".into())))
            }

            Message(_) => None,
        }
    }
}
