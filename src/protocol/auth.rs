use base64;
use sha1::{Digest, Sha1};

use tokio_tungstenite::{tungstenite::protocol::Message};
use crate::message::ECPMessage;

/// For a given auth challenge string, return the response
fn gen_challenge_response(received_challenge: &str, key: &Vec<u8>) -> String {
    let mut b64_auth_challenge_bytes = received_challenge.as_bytes().to_vec();

    let mut challenge_response_bytes = vec![];
    challenge_response_bytes.append(&mut b64_auth_challenge_bytes);
    challenge_response_bytes.append(&mut key.clone());

    let hash = Sha1::digest(challenge_response_bytes);
    base64::encode(&hash)
}

impl ECPMessage {
    /// Report any auth message
    pub fn is_auth_message(content: &str) -> bool { Self::is_auth_challenge(content) || Self::is_auth_response(content) }

    /// Is a challenge request
    pub fn is_auth_challenge(content: &str) -> bool { content.contains(r#"{"notify":"authenticate""#) }

    /// Is a challenge response result
    pub fn is_auth_response(content: &str) -> bool { content.contains(r#"{"response":"authenticate""#) }

    /// Handle the authentication challenge request
    pub fn generate_challenge_response(message: &str, counter: i32, key: &Vec<u8>) -> Message {
        // Handle authentication request
        let received_challenge = String::from_utf8(message.as_bytes()[44..68].to_vec()).unwrap();
        let challenge_response = gen_challenge_response(&received_challenge, key);

        Message::text(format!("{{\"request\":\"authenticate\",\"request-id\":\"{}\",\"param-response\":\"{}\"}}", counter, challenge_response))
    }
}