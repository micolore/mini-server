use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::thread;
use std::time::Duration;
use mini_server::ThreadPool;

// https://www.bookstack.cn/read/trpl-zh-cn-1.41/ch20-00-final-project-a-web-server.md
fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:18080").unwrap();
    for stream in listener.incoming().take(2)
    {
        let stream = stream.unwrap();
        println!("conn is incoming!");
        let pool = ThreadPool::new(5);
        pool.execute(|| {
            handle_stream(stream)
        })
    }
    println!("shutting down.");
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


