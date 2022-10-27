use serde_json::Value;

use crate::message::{ContentData, ContentType, ECPMessage};

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    pub subject:            String,
    pub response_id:        i32,
    pub content_data:       Option<ContentData>,
    pub content_type:       Option<ContentType>,
    pub status_code:        i32,
    pub status_message:     String,
    pub raw_bytes:          Vec<u8>,
}

impl Response {
    /// Create a Response struct from an ECP Text message
    pub fn from_message(message: ECPMessage) -> Option<Self> {
        match message {
            ECPMessage::Text { text } => {
                let raw_bytes = Vec::from(text.as_bytes());

                // Deserialize response JSON
                match serde_json::from_str::<Value>(&text) {
                    Err(e) => {
                        println!("[!] Unable to deserialize response: {:?}", e);
                        Some(
                            Response {
                                subject: String::new(),
                                response_id: 0,
                                content_data: None,
                                content_type: None,
                                status_code: 0,
                                status_message: String::new(),
                                raw_bytes,
                            }
                        )
                    }
                    Ok(json) => {
                        let mut response = Self::parse_response(json);
                        response.raw_bytes = raw_bytes;
                        Some(response)
                    }
                }
            }
            _ => { None }
        }
    }

    /// Parse message JSON data
    fn parse_response(json: Value) -> Response {
        let subject = match &json["response"] {
            Value::String(text) => String::from(text),
            _ => String::new(),
        };

        let response_id = match &json["response-id"] {
            Value::String(text) => {
                match text.parse::<i32>() {
                    Ok(parsed) => parsed,
                    Err(_) => -1,
                }
            },
            _ => -1,
        };

        let content_type = match &json["content-type"] {
            Value::String(text) => {
                if text.contains("xml") {
                    Some(ContentType::Xml)
                }
                else if text.contains("jpeg") {
                    Some(ContentType::Jpeg)
                }
                else if text.contains("json") {
                    Some(ContentType::Json)
                }
                else if text.contains("png") {
                    Some(ContentType::Png)
                }
                else {
                    Some(ContentType::None)
                }
            },
            _ => None,
        };

        let content_data = match &json["content-data"] {
            Value::String(text) => {
                // Attempt to decode base-64 data
                match base64::decode(text) {
                    Ok(decoded) => {
                        match content_type {
                            Some(ContentType::Xml) | Some(ContentType::Json) => {
                                match String::from_utf8(decoded.clone()) {
                                    Ok(utf8) => Some(ContentData::Text { string: utf8 }),
                                    Err(e) => {
                                        println!("[!] Unable to decode JSON/XML from UTF-8: {:?}", e);
                                        Some(ContentData::Data { bytes: decoded })
                                    },
                                }
                            }
                            Some(ContentType::Png) | Some(ContentType::Jpeg) => Some(ContentData::Data { bytes: decoded }),
                            Some(ContentType::None) | None => None,
                        }
                    }
                    Err(e) => {
                        println!("[!] Unable to decode JSON/XML from base64: {:?}", e);
                        Some(ContentData::Text { string: String::from(text) })
                    }
                }
            },
            _ => None,
        };

        let status_code = match &json["status"] {
            Value::String(text) => {
                match text.parse::<i32>() {
                    Ok(parsed) => parsed,
                    Err(_) => 0,
                }
            },
            _ => 0,
        };

        let status_message = match &json["status-msg"] {
            Value::String(text) => String::from(text),
            _ => String::new(),
        };

        Response {
            subject,
            response_id,
            content_data,
            content_type,
            status_code,
            status_message,
            raw_bytes: vec![],
        }
    }

    /// Whether or not this response has a success status code
    pub fn is_success(&self) -> bool {
        self.status_code == 200
    }
}