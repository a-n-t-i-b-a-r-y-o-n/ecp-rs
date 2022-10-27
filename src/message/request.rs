use std::collections::HashMap;
use crate::message::ECPMessage;
use crate::protocol::command::Set;
use crate::protocol::query::Get;

/// Buildable Request objects
pub struct Request {
    subject:        String,
    request_id:     i32,
    params:         HashMap<String, String>,
}

impl Request {
    /// Create new RequestBuilder to build request
    pub fn new() -> Self {
        Request {
            subject: String::new(),
            request_id: 0,
            params: HashMap::new(),
        }
    }

    /// Set the request-id
    pub fn set_request_id(mut self, id: i32) -> Self {
        self.request_id = id;
        self
    }

    /// Set the request subject
    pub fn set_subject(mut self, subject: &str) -> Self {
        self.subject = String::from(subject);
        self
    }

    /// Add a key/value param to the request
    pub fn add_param(mut self, key: &str, value: &str) -> Self {
        self.params.insert(String::from(key), String::from(value));
        self
    }

    /// Set a list of params
    pub fn set_params(mut self, params: Option<HashMap<String, String>>) -> Request {
        if let Some(map) = params {
            self.params = map;
        }
        self
    }

    /// Turn the RequestBuilder into a built Request
    pub fn build(&self) -> ECPMessage {
        let mut params = String::new();
        for (key, value) in &self.params {
            params.push_str(&format!("\"{}\":\"{}\",", key, value))
        }
        let content = format!(
            "{{\"request\":\"{}\",{}\"request-id\":\"{}\"}}",
            self.subject,
            params,
            self.request_id
        );

        ECPMessage::Text { text: content }
    }
}

impl From<Get> for Request {
    fn from(get: Get) -> Self {
        Request::new()
            .set_subject(get.subject())
            .set_params(get.params())
    }
}

impl From<Set> for Request {
    fn from(set: Set) -> Self {
        Request::new()
            .set_subject(set.subject())
            .set_params(set.params())
    }
}
