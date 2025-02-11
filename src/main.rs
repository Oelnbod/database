//use self::models::*;
pub mod database;
pub mod api;
//using serde for parsing json

use std::net::TcpListener;
use std::thread;
use threadpool::ThreadPool;

fn main() {
    let thread_pool = ThreadPool::new(10);

    thread::spawn(move || {
        let http_listener = TcpListener::bind("0.0.0.0:7878").unwrap();
        for stream in http_listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread_pool.execute(move || api::handle_connection(stream));
                }
                Err(_) => {
                    println!("Failed to connect to request.");
                }
            }
        }
    });

    //prevents the main thread from ending (for multithreading)
    loop {
        thread::park();
    }
}
