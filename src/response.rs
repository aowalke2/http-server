use std::collections::HashMap;

#[derive(Debug)]
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

pub struct Response {
    status: StatusCode,
    headers: HashMap<String, String>,
    body: String,
}

impl Response {
    pub fn new(status: StatusCode) -> Self {
        Response {
            status,
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }

    pub fn build(self) -> String {
        let mut response = String::from(self.status);
        for (key, value) in self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        response.push_str("\r\n");
        response.push_str(&self.body);
        response
    }
}
