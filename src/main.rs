use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};
use crate::lib::ThreadPool;

mod lib;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // Create a thread pool with 4 threads
    let pool = ThreadPool::new(3);

    // Listen for incoming connections and spawn a task for each one
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let hang = async{ handle_connection(stream) };
        // Spawn a task to handle each connection asynchronously
    }
}
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader
        .lines()
        .next()
        .expect("Next Something wrong")
        .expect("Read error");

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK\r\n\r\n", "src/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK\r\n\r\n", "src/sleep.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "src/404.html"),
    };

    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(err) => panic!("Something happens when read {} , {}", err, filename),
    };
    let length = contents.len();
    println!("{} - {}", status_line, filename);
    let response = format!(
        "{status_line}\r\nContent-Type: text/html\r\nContent-Length: {length}\r\n\r\n{contents}"
    );
    stream.write_all(response.as_bytes()).unwrap();
}
