use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

//------------------------------------------------------------------------------------------------//
// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];
//     stream.read(&mut buffer).unwrap();
//
//     let get = b"GET / HTTP/1.1\r\n";
//
//     if buffer.starts_with(get) {
//         let contents = fs::read_to_string("hello.html").unwrap();
//
//         let response = format!(
//             "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
//             contents.len(),
//             contents
//         );
//
//         stream.write(response.as_bytes()).unwrap();
//         stream.flush().unwrap();
//
//     } else {
//         let status_line = "HTTP/1.1 404 NOT FOUND";
//         let contents = fs::read_to_string("404.html").unwrap();
//
//         let response = format!(
//             "{}\r\nContent-Length: {}\r\n\r\n{}",
//             status_line,
//             contents.len(),
//             contents
//         );
//
//         stream.write(response.as_bytes()).unwrap();
//         stream.flush().unwrap();
//     }
// }

//------------------------------------------------------------------------------------------------//
// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];
//     stream.read(&mut buffer).unwrap();
//
//     let contents = fs::read_to_string("hello.html").unwrap();
//
//     let response = format!(
//         "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
//         contents.len(),
//         contents
//     );
//
//     stream.write(response.as_bytes()).unwrap();
//     stream.flush().unwrap();
// }

//------------------------------------------------------------------------------------------------//
// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];
//
//     stream.read(&mut buffer).unwrap();
//
//     let response = "HTTP/1.1 200 OK\r\n\r\n";
//
//     stream.write(response.as_bytes()).unwrap();
//     stream.flush().unwrap();
//     // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
// }

//------------------------------------------------------------------------------------------------//
// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];
//
//     stream.read(&mut buffer).unwrap();
//
//     println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
// }

//------------------------------------------------------------------------------------------------//
// fn main() {
//     let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
//
//     for stream in listener.incoming() {
//         let stream = stream.unwrap();
//
//         println!("Connection established!");
//     }
// }