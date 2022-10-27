use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use rand::prelude::*;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    MaybeTlsStream,
    tungstenite::protocol::Message,
    WebSocketStream
};
use tokio_tungstenite::tungstenite::handshake::client::Request;
use crate::message::ECPMessage;

/// ECP WebSocket connection
#[derive(Debug)]
pub struct ECPSocket {
    pub authenticated:  bool,
    pub writer:         SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    pub reader:         SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl ECPSocket {
    /// Open unauthenticated connection to device
    pub async fn open(ipv4: &str, port: &str) -> Self {
        // Open WebSocket connection
        let websocket_stream = Self::connect_websocket(ipv4, port).await;

        // Separate sink & stream
        let (writer, reader) = websocket_stream.split();

        Self {
            authenticated: false,
            writer,
            reader,
        }
    }

    /// Perform authentication via challenge-response flow and return outcome, dropping all other messages
    pub async fn authenticate(&mut self, key: &Vec<u8>, counter: i32) -> bool {
        loop {
            if let Some(Ok(message)) = self.reader.next().await {
                if let Some(ecp_message) = ECPMessage::try_from_auth_message(message, counter, key) {
                    match ecp_message {
                        ECPMessage::Authentication { text, response } => {
                            // Send reply and move on
                            if let Some(reply) = response {
                                let _ = self.writer.send(reply).await;
                                continue;
                            }

                            if text.contains("Error") || text.contains("error") {
                                println!("[!] Authentication error: {}", text);
                                break;
                            }

                            // Check for success status code
                            if text.contains("200") {
                                self.authenticated = true;
                                break;
                            }

                            println!("[-] Unexpected auth message received: {}", text);
                        }
                        _ => {}
                    }
                }
            }
        }
        self.authenticated
    }

    /// Open WebSocket connection to device as an Android device
    async fn connect_websocket(ipv4: &str, port: &str) -> WebSocketStream<MaybeTlsStream<TcpStream>> {
        // Generate random base-64 Sec-WebSocket-Key value
        let rand_bytes = thread_rng().gen::<[u8; 16]>();
        let rand_websocket_key = base64::encode(rand_bytes.to_vec());

        // WebSocket upgrade request for /ecp-session with key, protocol, origin
        let request = Request::builder()
            .method("GET")
            .header("Host", format!("{}:{}", ipv4, port))
            .header("Upgrade", "websocket")
            .header("Connection", "Upgrade")
            .header("Sec-WebSocket-Key", &rand_websocket_key)
            .header("Sec-WebSocket-Version", "13")
            .header("Sec-WebSocket-Protocol", "ecp-2")
            .header("Sec-WebSocket-Origin", "Android")
            .uri(format!("ws://{}:{}/ecp-session", ipv4, port))
            .body(())
            .unwrap();

        // Connect and return stream
        let (websocket_stream, _) = connect_async(request).await.unwrap();
        websocket_stream
    }
}