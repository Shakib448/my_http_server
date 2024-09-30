use serde_json::json;
use std::{
    io::{BufRead, BufReader, Result, Write},
    net::{TcpListener, TcpStream},
};

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
    Ok(())
}
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let data = json!({
        "name": "John Doe",
        "age": 30,
        "city": "New York"
    });

    let json_string = serde_json::to_string(&data).unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\n\
        Content-Length: {}\r\n\
        Content-Type: application/json\r\n\
        Access-Control-Allow-Origin: *\r\n\
        Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n\
        Access-Control-Allow-Headers: Content-Type\r\n\
        \r\n\
        {}",
        json_string.len(),
        json_string
    );

    println!("Request: {http_request:#?}");
    stream.write(&response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
