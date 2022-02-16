use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:18080").unwrap();
    for stream in
    listener.incoming()
    {
        let stream = stream.unwrap();
        println!("conn is incoming!");
        thread::spawn(|| {
            handle_stream(stream);
        })
    }
}

fn handler_conn(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    println!("request! {}", String::from_utf8_lossy(&buffer[..]));

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "html/hello_rust.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(3));
        ("HTTP/1.1 200 OK\r\n\r\n", "html/hello_rust.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "html/404.html")
    };
    write_result(filename, status_line, stream);
}

fn write_result(filename: &str, status_line: &str, mut stream: TcpStream) {
    let contents = fs::read_to_string(filename).unwrap();
    let resp = format!("{}{}", status_line, contents);
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_stream_v1(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("request! {}", String::from_utf8_lossy(&buffer[..]));
    let (status_line, filename) =
        ("HTTP/1.1 200 OK\r\n\r\n", "html/hello_rust.html");
    let contents = fs::read_to_string(filename).unwrap();
    let resp = format!("{}{}", status_line, contents);
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("request! {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "html/hello_rust.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "html/404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let resp = format!("{}{}", status_line, contents);
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}
