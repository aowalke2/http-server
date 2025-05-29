use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

#[derive(Debug, Clone, Copy)]
pub enum HttpMethod {
    Get,
    Post,
}

impl From<&str> for HttpMethod {
    fn from(value: &str) -> Self {
        match value {
            "GET" => HttpMethod::Get,
            "POST" => HttpMethod::Post,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct Request {
    method: HttpMethod,
    route: Vec<String>,
    headers: HashMap<String, String>,
    body: String,
}

impl Request {
    pub fn new(stream: &TcpStream) -> Self {
        let mut buf_reader = BufReader::new(stream);

        let mut request_line = String::new();
        buf_reader.read_line(&mut request_line).unwrap();
        let request_line: Vec<&str> = request_line.split(" ").collect();

        let method = request_line[0].into();
        let route = request_line[1]
            .split("/")
            .skip(1)
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let mut headers: HashMap<String, String> = HashMap::new();
        loop {
            let mut header_line = String::new();
            let length = buf_reader.read_line(&mut header_line).unwrap();

            if header_line.trim().is_empty() {
                break;
            }

            if length == 0 {
                break;
            }

            let entry: Vec<&str> = header_line.split(": ").collect();
            headers.insert(entry[0].trim().into(), entry[1].trim().into());
        }

        let mut body = String::new();
        if let Some(content_length) = headers.get("Content-Length") {
            let content_length = content_length.parse::<usize>().unwrap();
            let mut buffer = vec![0; content_length];
            buf_reader.read_exact(&mut buffer).unwrap();
            body = String::from_utf8(buffer).unwrap();
        }

        Request {
            method,
            route,
            headers,
            body,
        }
    }

    pub fn method(&self) -> HttpMethod {
        self.method
    }

    pub fn route(&self) -> &Vec<String> {
        &self.route
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn body(&self) -> String {
        self.body.clone()
    }
}
