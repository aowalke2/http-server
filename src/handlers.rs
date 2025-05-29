use std::{
    collections::HashMap,
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

pub fn handle_connection(mut stream: TcpStream) {
    let http_request = Request::new(&mut stream);
    let route = http_request.route();
    let method = http_request.method();

    let response = match (method, route[0].as_str()) {
        (HttpMethod::Get, "echo") => handle_echo(route[1].clone(), &http_request.headers()),
        (HttpMethod::Get, "user-agent") => handle_user_agent(&http_request.headers()),
        (HttpMethod::Get, "files") => handle_read_file(route[1].clone()),
        (HttpMethod::Post, "files") => handle_create_file(route[1].clone(), http_request.body()),
        (HttpMethod::Get, "") => Response::new(StatusCode::Ok).build(),
        _ => Response::new(StatusCode::NotFound).build(),
    };

    stream.write_all(&response).unwrap();
}

fn handle_echo(content: String, headers: &HashMap<String, String>) -> Vec<u8> {
    let response = Response::new(StatusCode::Ok)
        .with_header("Content-Type", "text/plain")
        .with_header("Content-Length", &content.chars().count().to_string());

    if let Some(encodings) = headers.get("Accept-Encoding") {
        let mut valid_encodings = Vec::new();
        for encoding in encodings.split(", ") {
            if COMPRESSION_SCHEMES.contains(&encoding) {
                valid_encodings.push(encoding);
            }
        }

        match valid_encodings.contains(&"gzip") {
            true => response
                .with_header("Content-Encoding", valid_encodings.join(", ").as_str())
                .with_body(&content.as_bytes().to_vec())
                .with_gzip()
                .build(),
            false => response
                .with_header("Content-Encoding", valid_encodings.join(", ").as_str())
                .with_body(&content.as_bytes().to_vec())
                .build(),
        }
    } else {
        response.with_body(&content.as_bytes().to_vec()).build()
    }
}

fn handle_user_agent(headers: &HashMap<String, String>) -> Vec<u8> {
    let content = headers.get("User-Agent").unwrap();
    Response::new(StatusCode::Ok)
        .with_header("Content-Type", "text/plain")
        .with_header("Content-Length", &content.chars().count().to_string())
        .with_body(&content.as_bytes().to_vec())
        .build()
}

fn handle_read_file(file: String) -> Vec<u8> {
    if let Some(dir) = env::args().nth(2) {
        match File::open(Path::new(&dir).join(file)) {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                Response::new(StatusCode::Ok)
                    .with_header("Content-Type", "application/octet-stream")
                    .with_header("Content-Length", &content.chars().count().to_string())
                    .with_body(&content.as_bytes().to_vec())
                    .build()
            }
            Err(_) => Response::new(StatusCode::NotFound).build(),
        }
    } else {
        Response::new(StatusCode::NotFound).build()
    }
}

fn handle_create_file(file: String, content: String) -> Vec<u8> {
    if let Some(dir) = env::args().nth(2) {
        match File::create(Path::new(&dir).join(file)) {
            Ok(mut file) => {
                file.write_all(content.as_bytes()).unwrap();
                Response::new(StatusCode::Created).build()
            }
            Err(_) => Response::new(StatusCode::NotFound).build(),
        }
    } else {
        Response::new(StatusCode::NotFound).build()
    }
}
