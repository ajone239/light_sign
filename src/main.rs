use std::env;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

use light_sign::thread_pool::ThreadPool;

const PORT: i32 = 9999;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    // will be used mutably?
    let int_ref = Arc::new(Mutex::new(0));

    let ip_str = if args.len() > 1 {
        format!("{}:{}", args[1], PORT)
    } else {
        format!("{}:{}", "localhost", PORT)
    };

    let listener = TcpListener::bind(&ip_str)?;
    println!("Bound at -> http://{}", &ip_str);

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let ref_ref = Arc::clone(&int_ref);
        pool.execute(|| {
            light_sign::handle_connection(stream, ref_ref);
        });
    }

    Ok(())
}
