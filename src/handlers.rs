use std::{
    env,
    fs::File,
    io::{Read, Write},
    net::TcpStream,
    path::Path,
};

use crate::{
    request::{HttpMethod, Request},
    response::*,
};

const COMPRESSION_SCHEMES: [&str; 1] = ["gzip"];

pub fn handle_connection(stream: &mut TcpStream) {
    loop {
        let http_request = Request::new(&stream);
        let route = http_request.route();
        let method = http_request.method();

        let response = match (method, route[0].as_str()) {
            (HttpMethod::Get, "echo") => handle_echo(route[1].clone(), &http_request),
            (HttpMethod::Get, "user-agent") => handle_user_agent(&http_request),
            (HttpMethod::Get, "files") => handle_read_file(route[1].clone(), &http_request),
            (HttpMethod::Post, "files") => handle_create_file(route[1].clone(), &http_request),
            (HttpMethod::Get, "") => Response::new(StatusCode::Ok).build(),
            _ => Response::new(StatusCode::NotFound).build(),
        };

        stream.write_all(&response).unwrap();
        stream.flush().unwrap();

        if http_request.headers().contains_key("Connection") {
            break;
        }
    }
}

fn handle_echo(content: String, request: &Request) -> Vec<u8> {
    let headers = request.headers();
    let mut response = Response::new(StatusCode::Ok)
        .with_header("Content-Type", "text/plain")
        .with_header("Content-Length", &content.chars().count().to_string());

    response = match headers.get("Accept-Encoding") {
        Some(encodings) => {
            let mut valid_encodings = Vec::new();
            for encoding in encodings.split(", ") {
                if COMPRESSION_SCHEMES.contains(&encoding) {
                    valid_encodings.push(encoding);
                }
            }

            let compression_kind = match valid_encodings.contains(&"gzip") {
                true => CompressionKind::Gzip,
                false => CompressionKind::None,
            };

            response
                .with_header("Content-Encoding", valid_encodings.join(", ").as_str())
                .with_compression(compression_kind)
        }
        None => response,
    };

    response = match request.headers().get("Connection") {
        Some(close) => response.with_header("Connection", close),
        None => response,
    };
    response.with_body(&content.as_bytes().to_vec()).build()
}

fn handle_user_agent(request: &Request) -> Vec<u8> {
    let content = request.headers().get("User-Agent").unwrap();
    let mut response = Response::new(StatusCode::Ok)
        .with_header("Content-Type", "text/plain")
        .with_header("Content-Length", &content.chars().count().to_string())
        .with_body(&content.as_bytes().to_vec());

    response = match request.headers().get("Connection") {
        Some(close) => response.with_header("Connection", close),
        None => response,
    };
    response.build()
}

fn handle_read_file(file: String, request: &Request) -> Vec<u8> {
    let mut response = if let Some(dir) = env::args().nth(2) {
        match File::open(Path::new(&dir).join(file)) {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                Response::new(StatusCode::Ok)
                    .with_header("Content-Type", "application/octet-stream")
                    .with_header("Content-Length", &content.chars().count().to_string())
                    .with_body(&content.as_bytes().to_vec())
            }
            Err(_) => Response::new(StatusCode::NotFound),
        }
    } else {
        Response::new(StatusCode::NotFound)
    };

    response = match request.headers().get("Connection") {
        Some(close) => response.with_header("Connection", close),
        None => response,
    };
    response.build()
}

fn handle_create_file(file: String, request: &Request) -> Vec<u8> {
    let content = request.body();
    let mut response = if let Some(dir) = env::args().nth(2) {
        match File::create(Path::new(&dir).join(file)) {
            Ok(mut file) => {
                file.write_all(content.as_bytes()).unwrap();
                Response::new(StatusCode::Created)
            }
            Err(_) => Response::new(StatusCode::NotFound),
        }
    } else {
        Response::new(StatusCode::NotFound)
    };

    response = match request.headers().get("Connection") {
        Some(close) => response.with_header("Connection", close),
        None => response,
    };
    response.build()
}
