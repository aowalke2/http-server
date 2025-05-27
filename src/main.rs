use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::{env, thread};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        thread::spawn(|| match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => println!("error: {}", e),
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_line: Vec<&str> = http_request[0].as_str().split(" ").collect();
    let headers = parse_headers(&http_request);
    let route = request_line[1].split("/").skip(1).collect::<Vec<&str>>();
    let response = match route.as_slice() {
        ["echo", content] => handle_echo(content.to_string()),
        ["user-agent"] => handle_user_agent(&headers),
        ["files", file] => handle_file(file.to_string()),
        [""] => ok(),
        _ => not_found(),
    };

    stream.write_all(response.as_bytes()).unwrap();
}

fn parse_headers(http_request: &Vec<String>) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    for i in 1..http_request.len() {
        if http_request[i] != "" {
            let entry: Vec<&str> = http_request[i].split(": ").collect();
            headers.insert(entry[0].into(), entry[1].into());
        }
    }
    headers
}

fn handle_echo(content: String) -> String {
    ok_with_content("text/plain", &content)
}

fn handle_user_agent(headers: &HashMap<String, String>) -> String {
    let content = headers.get("User-Agent").unwrap();
    ok_with_content("text/plain", content)
}

fn handle_file(file: String) -> String {
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

fn not_found() -> String {
    "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
}

fn ok() -> String {
    "HTTP/1.1 200 OK\r\n\r\n".to_string()
}

fn ok_with_content(content_type: &str, content: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        content_type,
        content.chars().count(),
        content
    )
}
