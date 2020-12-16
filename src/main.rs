use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::{env, fs, str};

use light_sign::ThreadPool;

const PORT: i32 = 9999;

fn main() {
    let args: Vec<String> = env::args().collect();

    let ip_str = if args.len() > 1 {
        format!("{}:{}", args[1], PORT)
    } else {
        format!("{}:{}", "localhost", PORT)
    };

    let listener = match TcpListener::bind(&ip_str) {
        Ok(listen) => {
            println!("Bound at -> http://{}", &ip_str);
            listen
        }
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

        // take first line
        let line = str::from_utf8(&buffer).unwrap().lines().next().unwrap();
        // extract request
        let line = line.split_at(6).1;
        let line = line.split_at(line.rfind(" ").unwrap()).0;

        // parse the content
        for obj in line.split('&') {
            let tmp = obj.split_at(obj.find("=").unwrap() + 1);
            match tmp.0 {
                "type=" => {
                    println!("Type: {}", tmp.1);
                    if tmp.1 != "other" {
                        break;
                    }
                },
                "opt_cont=" => println!("Cont: {}", tmp.1.replace("+", " ")),
                _ => println!("Nope"),
            }
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
