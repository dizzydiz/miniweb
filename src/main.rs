use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, file_path) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    write_respons(&mut stream, status_line, file_path)
}

fn write_respons(stream: &mut TcpStream, status_line: &str, file_path: &str) {
    //TODO  add a test
    let contents = fs::read_to_string(file_path).unwrap();

    let content_length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
