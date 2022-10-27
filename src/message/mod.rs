pub mod request;
pub mod response;

use tokio_tungstenite::{
    tungstenite::protocol::Message
};

// Content data, which could be a string or some bytes
#[derive(Debug, Eq, PartialEq)]
pub enum ContentData {
    Text { string: String },
    Data { bytes: Vec<u8> },
}

// Content type indicator
#[derive(Debug, Eq, PartialEq)]
pub enum ContentType {
    Jpeg,
    Json,
    Png,
    Xml,
    None,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ECPMessage {
    Authentication { text: String, response: Option<Message> },
    Binary { bytes: Vec<u8> },
    Control { bytes: Vec<u8> },
    Text { text: String },
    Unrecognized { bytes: Vec<u8> },
}

impl ECPMessage {
    // TODO: Implement request-id parsing to check we got the correct response
    /// Handle non-auth messages
    pub fn from_message(message: Message) -> ECPMessage {
        return if message.is_close() || message.is_ping() || message.is_pong() {
            ECPMessage::Control {
                bytes: message.into_data(),
            }
        }
        else if message.is_binary() {
            ECPMessage::Binary {
                bytes: message.into_data(),
            }
        }
        else if message.is_text() {
            let text = String::from(message.into_text().unwrap());
            if Self::is_auth_message(&text) {
                println!("[!] Unexpected auth message received: {}", text);
                ECPMessage::Authentication {
                    text,
                    response: None,
                }
            }
            else {
                ECPMessage::Text {
                    text,
                }
            }
        }
        else {
            ECPMessage::Unrecognized {
                bytes: message.into_data(),
            }
        }
    }

    /// Consume an ECPMessage and return a WebSocket Message
    pub fn into_message(self) -> Message {
        return match self {
            ECPMessage::Authentication { text, .. } |
            ECPMessage::Text { text } => {
                Message::Text(text)
            }
            ECPMessage::Binary { bytes } |
            ECPMessage::Control { bytes } |
            ECPMessage::Unrecognized { bytes } => {
                Message::Binary(bytes)
            }
        }
    }

    /// Return the parsed message only if it is an authentication message
    pub(crate) fn try_from_auth_message(message: Message, counter: i32, key: &Vec<u8>) -> Option<ECPMessage> {
        return if message.is_text() {
            let text = String::from(message.into_text().unwrap());
            if Self::is_auth_challenge(&text) {
                let response = Self::generate_challenge_response(&text, counter, key);
                Some(ECPMessage::Authentication {
                    text,
                    response: Some(response),
                })
            }
            else if Self::is_auth_message(&text) {
                Some(ECPMessage::Authentication {
                    text,
                    response: None,
                })
            }
            else {
                None
            }
        }
        else {
            None
        }
    }
}