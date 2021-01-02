pub mod thread_pool;

use std::io::prelude::*;
use std::net::TcpStream;
use std::{fs, str};

pub fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let data = b"GET /?";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "./html/hello.html")
    } else if buffer.starts_with(data) {
        parse_data_request(&buffer);

        ("HTTP/1.1 200 OK\r\n\r\n", "./html/success.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "./html/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn parse_data_request(buffer: &[u8]) {
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
                if tmp.1 != "other" {
                    println!("Type_str: {}", req_type_to_str(tmp.1));
                    break;
                }
            },
            "opt_cont=" => println!("Cont: {}", tmp.1.replace("+", " ")),
            _ => println!("Nope"),
        }
    }
}

fn req_type_to_str(type_str: &str) -> &str {
    match type_str {
      "comein"	        => "Come In",
      "work"	        => "Work",
      "school"	        => "School",
      "donotdisturb"	=> "Do Not Disturb",
      _                 => "Invalid",
    }
}
