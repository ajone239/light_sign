//! # Extra functions for the light sign
//!
//! These functions serve to tidy up the main function.
//! Their function manly relates to string processing and connection handling.

// Expose thread_pool to the project
pub mod thread_pool;

use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::{fs, str};

use rppal::uart::Uart;

/// # Handles a new HTTP connection
///
/// This function takes a stream object that it will consume at the end of it's running.
/// The stream object is used to talk back and forth with the client.
/// The function all takes a mutxed reference to a uart object used to send the pertenant data
/// to an arduino.
/// The content sent to the arduino is parsed out of the HTTP request.
pub fn handle_connection(mut stream: TcpStream, str_arc: Arc<Mutex<Uart>>) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // Request types
    let get = b"GET / HTTP/1.1\r\n";
    let query = b"GET /?";

    let (status_line, filename) = if buffer.starts_with(get) {
        (
            "HTTP/1.1 200 OK\r\n\r\n",
            "/home/pi/Rust/cross_bins/html/hello.html",
        )
    } else if buffer.starts_with(query) {
        let mut response_path = "/home/pi/Rust/cross_bins/html/failure.html";

        // Attempt to parse the string
        match parse_data_request(&buffer) {
            Ok(good_line) => {
                let mut uart = str_arc.lock().unwrap();

                // Try to send the data
                if uart.write(good_line.as_bytes()).unwrap() > 0 {
                    println!("Success -> {}", &good_line);
                    response_path = "/home/pi/Rust/cross_bins/html/success.html";
                } else {
                    println!("Failed -> {}", &good_line);
                }
            }
            Err(bad_line) => eprintln!("{}", bad_line),
        }

        ("HTTP/1.1 200 OK\r\n\r\n", response_path)
    } else {
        (
            "HTTP/1.1 404 NOT FOUND\r\n\r\n",
            "/home/pi/Rust/cross_bins/html/404.html",
        )
    };

    // get the html file contents
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    // write to the stream
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

/// # Parse HTTP Request
///
/// Does processing of the HTTP request to extract the querie in the request.
/// The caller asures that this will be a querie.
fn parse_data_request(buffer: &[u8]) -> Result<String, String> {
    // take first line
    let line = str::from_utf8(&buffer).unwrap().lines().next().unwrap();
    // extract request
    let line = line.split_at(6).1;
    let line = line.split_at(line.rfind(" ").unwrap()).0;

    let mut ret_string = String::from("ERROR");

    // parse the content
    for obj in line.split('&') {
        let tmp = obj.split_at(obj.find("=").unwrap() + 1);
        match tmp.0 {
            "type=" => {
                if tmp.1 != "other" {
                    ret_string = req_type_to_str(tmp.1).to_string();
                    break;
                }
            }
            "opt_cont=" => {
                ret_string = req_sanitize(tmp.1.replace("+", " ").as_str());
                println!("Cont: {}", ret_string);
                break;
            }
            _ => {
                let ret_string = format!("ERROR: {:?}", tmp);
                return Err(ret_string);
            }
        }
    }
    Ok(ret_string)
}

/// Easily convert preset types to their strings
fn req_type_to_str(type_str: &str) -> &str {
    match type_str {
        "comein" => "Come In",
        "work" => "Work",
        "school" => "School",
        "donotdisturb" => "Do Not Disturb",
        _ => "Invalid",
    }
}

/// Converts HTML codes into their charaters
fn req_sanitize(req: &str) -> String {
    req.split("%")
        .map(|s| match s {
            "21" => "!",
            "40" => "@",
            "23" => "#",
            "24" => "$",
            "25" => "%",
            "5E" => "^",
            "26" => "&",
            "28" => ")",
            "29" => "(",
            "2B" => "+",
            "5B" => "[",
            "5D" => "]",
            "7B" => "{",
            "7D" => "}",
            "3A" => ":",
            "3B" => ";",
            _ => s,
        })
        .collect::<Vec<&str>>()
        .join("")
}
