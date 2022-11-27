use std::fs::File;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

extern crate lite_web_server;
use lite_web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    println!("***** Listening...");
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        println!("***** Connected!");
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 512];
    stream.read(&mut buf).unwrap();
    // println!("***** Received!\n{}", String::from_utf8_lossy(&buf));
    println!("***** Received!");

    let (status, target_file) = {
        if buf.starts_with(b"GET /sleep HTTP/1.1\r\n") {
            thread::sleep(Duration::from_secs(10));
            ("200 OK", "index.html")
        } else if buf.starts_with(b"GET / HTTP/1.1\r\n") {
            ("200 OK", "index.html")
        } else {
            ("404 NOT FOUND", "404.html")
        }
    };

    let mut file = File::open(target_file).unwrap();
    let mut content= String::new();
    file.read_to_string(&mut content).unwrap();

    let http_resp = format!("HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}", status, content.len(), content);
    // println!("***** Sending response...\r\n{}", http_resp);
    println!("***** Sending response...");
    stream.write(http_resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}