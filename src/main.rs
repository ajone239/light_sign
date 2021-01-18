/*
 * Name: Austin Jones
 * Project: Light Sign
 * File: src/main.rs
 *
 * Purpose:
 *  Define the main execution of this project. Within the main function
 * the args are parsed. A TcpListener is created and bound to listen for
 * incoming streams. The incoming streams are pushed to a thread pool to be
 * handled.
 */

use std::env;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

use light_sign::thread_pool::ThreadPool;
use rppal::uart::{Parity, Uart};

/// Port the server will talk over
const PORT: i32 = 9999;
/// Number of threads in the pool
const THREAD_COUNT: usize = 4;

fn main() -> std::io::Result<()> {
    // Gather Args from command line
    let args: Vec<String> = env::args().collect();

    // Use the ip str given if provided
    let ip_str = if args.len() > 1 {
        format!("{}:{}", args[1], PORT)
    } else {
        format!("{}:{}", "localhost", PORT)
    };

    let listener = TcpListener::bind(&ip_str)?;
    println!("Bound at -> http://{}", &ip_str);

    let mut uart = Uart::new(9600, Parity::None, 8, 1).unwrap();
    // Make the read blocking
    uart.set_write_mode(true).unwrap();

    // Put the IP_STR on the sign
    if uart.write(ip_str.as_bytes()).unwrap() > 0 {
        println!("Success -> {}", &ip_str);
    }

    // Wrap the UART so it can be used across threads
    let uart_ref = Arc::new(Mutex::new(uart));

    // Get a thread pool to handle incoming connections
    let pool = ThreadPool::new(THREAD_COUNT);

    // Watch for incoming connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // Clone the arc and pass it to the connection handler
        let ref_clone = Arc::clone(&uart_ref);
        // Push the work onto the threadpool work queue
        pool.execute(|| {
            light_sign::handle_connection(stream, ref_clone);
        });
    }

    Ok(())
}
