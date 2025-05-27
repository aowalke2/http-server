pub fn not_found() -> String {
    "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
}

pub fn ok() -> String {
    "HTTP/1.1 200 OK\r\n\r\n".to_string()
}

pub fn ok_with_content(content_type: &str, content: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        content_type,
        content.chars().count(),
        content
    )
}

pub fn created() -> String {
    "HTTP/1.1 201 Created\r\n\r\n".to_string()
}
