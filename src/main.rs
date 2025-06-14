
pub mod api;
pub mod database;
//using serde for parsing json
use std::net::TcpListener;
use tokio;

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

   
}
