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

pub fn handle_connection(mut stream: TcpStream) {
    let http_request = Request::new(&mut stream);
    let route = http_request.route();
    let method = http_request.method();
    //println!("{}", &http_request.content_type());

    let response = match (method, route[0].as_str()) {
        (HttpMethod::Get, "echo") => handle_echo(route[1].clone()),
        (HttpMethod::Get, "user-agent") => handle_user_agent(&http_request.headers()),
        (HttpMethod::Get, "files") => handle_read_file(route[1].clone()),
        (HttpMethod::Post, "files") => handle_create_file(route[1].clone(), http_request.body()),
        (HttpMethod::Get, "") => ok(),
        _ => not_found(),
    };

    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_echo(content: String) -> String {
    ok_with_content("text/plain", &content)
}

fn handle_user_agent(headers: &HashMap<String, String>) -> String {
    let content = headers.get("User-Agent").unwrap();
    ok_with_content("text/plain", content)
}

fn handle_read_file(file: String) -> String {
    if let Some(dir) = env::args().nth(2) {
        match File::open(Path::new(&dir).join(file)) {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                ok_with_content("application/octet-stream", &content)
            }
            Err(_) => not_found(),
        }
    } else {
        not_found()
    }
}

fn handle_create_file(file: String, content: String) -> String {
    if let Some(dir) = env::args().nth(2) {
        match File::create(Path::new(&dir).join(file)) {
            Ok(mut file) => {
                file.write_all(content.as_bytes()).unwrap();
                created()
            }
            Err(_) => not_found(),
        }
    } else {
        not_found()
    }
}
