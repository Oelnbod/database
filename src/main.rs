//use self::models::*;
pub mod database;
pub mod api;
//using serde for parsing json
use tokio;
use std::net::TcpListener;


#[tokio::main]
async fn main() {


    let http_listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    for stream in http_listener.incoming() {
        match stream {
            Ok(stream) => {
		api::handle_connection(stream).await;
            }
            Err(_) => {
                println!("Failed to connect to request.");
            }
        }
    }
    

    //prevents the main thread from ending (for multithreading)
}
