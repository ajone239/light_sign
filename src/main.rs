use std::{env, fs, str};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use tut_final::ThreadPool;

fn main() {
    let args : Vec<String> = env::args().collect();

    let ip_str = if args.len() > 1 {
        format!("{}:8080", args[1])
    } else {
        String::from("localhost:8080")
    };

    let listener = match TcpListener::bind(&ip_str) {
        Ok(listen) => {
            println!("Bound at -> http://{}", &ip_str);
            listen
        },
        Err(_) => panic!("No Socket Bound"),
    };

    let pool = ThreadPool::new(4).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let data = b"GET /?";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "./html/hello.html")
    } else if buffer.starts_with(data) {
        let lines = str::from_utf8(&buffer).unwrap().lines();
        for line in lines {
            println!("{}", line);
        }
        ("HTTP/1.1 200 OK\r\n\r\n", "./html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "./html/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
