use futures_util::{SinkExt, StreamExt};
use crate::message::ECPMessage;
use crate::message::request::Request;
use crate::message::response::Response;
use crate::protocol::session::ECPSocket;

#[derive(Debug)]
pub struct Connection {
    pub ipv4:           [u8; 4],
    pub port:           usize,
    pub key:            Vec<u8>,
    pub sync_counter:   i32,
    pub socket:         Option<ECPSocket>,
}

impl Connection {
    /// Default ECP port
    const DEFAULT_PORT: usize = 8060;

    /// Create a new connection object with no socket connection
    pub fn new(ipv4: [u8; 4], key: Vec<u8>) -> Self {
        Self {
            ipv4,
            port: Self::DEFAULT_PORT,
            key,
            sync_counter: -1,
            socket: None,
        }
    }

    /// Whether or not the connection has been opened
    pub fn is_open(&self) -> bool {
        match &self.socket {
            None => false,
            Some(_) => true,
        }
    }

    /// Whether or not the connection has completed authentication
    pub fn is_authenticated(&self) -> bool {
        match &self.socket {
            None => false,
            Some(socket) => socket.authenticated
        }
    }

    /// Get and increment the request-id sync counter
    pub fn next_sync_number(&mut self) -> i32 {
        self.sync_counter+=1;
        self.sync_counter
    }

    /// Open connection to device and initialize authenticated ECP session
    pub async fn open(&mut self) -> bool {
        let mut socket = ECPSocket::open(
            &format!("{}.{}.{}.{}", self.ipv4[0], self.ipv4[1], self.ipv4[2], self.ipv4[3]),
            &format!("{}", self.port)
        ).await;

        let counter = self.next_sync_number();
        socket.authenticate(&self.key, counter).await;

        let result = socket.authenticated;
        self.socket = Some(socket);
        result
    }

    /// Send an ECPMessage request and wait for the next response
    pub async fn send_request(&mut self, request: Request) -> Option<Response> {
        match &mut self.socket {
            None => None,
            Some(socket) => {
                let _ = socket.writer.send(request.build().into_message()).await;
                self.sync_counter+=1;
                Response::from_message(self.next().await.unwrap())
            }
        }
    }

    /// Get next message of any type
    pub async fn next(&mut self) -> Option<ECPMessage> {
        match &mut self.socket {
            None => None,
            Some(socket) => {
                loop {
                    if let Some(socket_message) = socket.reader.next().await {
                        if let Ok(message) = socket_message {
                            return Some(ECPMessage::from_message(message))
                        }
                    }
                }
            }
        }
    }
}

impl Clone for Connection {
    /// Create a new unopened connection identical to the given one
    fn clone(&self) -> Self {
        Connection::new(self.ipv4, self.key.clone())
    }
}