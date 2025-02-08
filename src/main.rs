//use self::models::*;
pub mod database;

//using serde for parsing json
use serde_json;
use std::net::{TcpListener, TcpStream};
use std::io::*;
use std::thread;
use threadpool::ThreadPool;

fn main() {
    let thread_pool = ThreadPool::new(10);

    thread::spawn(move || {
    let http_listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    for stream in http_listener.incoming() {
        match stream {
            Ok(stream) => {
		thread_pool.execute(move || {
		    handle_connection(stream)
		});
	    },		    
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
    /* database stuff (commented out whilst api/web server stuff implemented)
    let connection = &mut database::connect(); //connecting to the database

    // --- the order of what follows is just for testing and can be changed --

    //displaying the results
    let results = database::display(connection);
    println!("{:?}", results);

    //writing the password, it returns the row added (I think)
    let password_out = database::create(connection, "example.com", "example", "hi");
    let password_out = database::create(connection, "example.com", "example", "hi");

    //displaying the results
    let result = database::display(connection);
    let json = serde_json::to_string(&result);

    println!("\n {:?}", json);

    //deleting the database
    database::delete("example.com", connection);

    //displaying the database again.
    println!("\n \n {:?}", database::display(connection));
     */
}

fn handle_connection(mut stream: TcpStream) {
    //reading the request
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // this next bit returns the bit after the /  in the http request

  //  let file = file_target(&request_line); //this is the file but after the slash that was entered

    //let target = return_file(file.to_string());

    //let mut full_file: String = "./www/".to_string();
    //full_file.push_str(&target); //full_file is the file including the ./www/ bit
    //println!("{full_file}");

    if request_line == "GET / HTTP/1.1" {
        //this is different so that the index.html is the landing page
        let status_line = "HTTP/1.1 200 OK";
        let contents = "hi";
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 200 OK";
        let contents = "hi";
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();

        //note: there is no 404.html exception as this is handled by return_file()
    }
}
