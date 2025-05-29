use flate2::{write::GzEncoder, Compression};
use std::{collections::HashMap, io::Write};

#[derive(Debug, Clone, Copy)]
pub enum StatusCode {
    NotFound,
    Ok,
    Created,
}

impl From<StatusCode> for String {
    fn from(status: StatusCode) -> Self {
        match status {
            StatusCode::Ok => "HTTP/1.1 200 OK\r\n".to_string(),
            StatusCode::Created => "HTTP/1.1 201 Created\r\n".to_string(),
            StatusCode::NotFound => "HTTP/1.1 404 Not Found\r\n".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CompressionKind {
    Gzip,
    None,
}

#[derive(Debug)]
pub struct Response {
    status: StatusCode,
    headers: HashMap<String, String>,
    body: Vec<u8>,
    compression_kind: CompressionKind,
}

impl Response {
    pub fn new(status: StatusCode) -> Self {
        Response {
            status,
            headers: HashMap::new(),
            body: Vec::new(),
            compression_kind: CompressionKind::None,
        }
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_body(mut self, body: &Vec<u8>) -> Self {
        self.body = body.clone();
        self
    }

    pub fn with_compression(mut self, compression_kind: CompressionKind) -> Self {
        self.compression_kind = compression_kind;
        self
    }

    pub fn build(&mut self) -> Vec<u8> {
        let mut response = Vec::new();
        response.extend_from_slice(String::from(self.status).as_bytes());

        match self.compression_kind {
            CompressionKind::Gzip => {
                let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                encoder.write_all(&self.body).unwrap();
                self.body = encoder.finish().unwrap();
                if let Some(length) = self.headers.get_mut("Content-Length") {
                    *length = self.body.len().to_string();
                }
            }
            CompressionKind::None => {}
        }

        for (key, value) in self.headers.iter() {
            response.extend_from_slice(&format!("{}: {}\r\n", key, value).as_bytes());
        }

        response.extend_from_slice(b"\r\n");
        response.extend_from_slice(&self.body);
        response
    }
}
