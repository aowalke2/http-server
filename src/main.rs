use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => println!("error: {}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<String>>();

    let request_line = http_request[0].as_str().split(" ").collect::<Vec<&str>>();
    let route = request_line[1];
    let response = match route {
        "/" => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
        route if route.starts_with("/echo/") => handle_echo(route.to_string()),
        _ => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
    };

    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_echo(route: String) -> String {
    let route = route.split("/").collect::<Vec<&str>>();
    let content = route.last().unwrap();
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        content.chars().count(),
        content
    )
}
