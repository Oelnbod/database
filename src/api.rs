//pub mod database;
//use crate::database;
use crate::database;
use bcrypt::verify;
use std::io::*;
use std::net::TcpStream;

pub async fn handle_connection(mut stream: TcpStream) {
    //reading the request
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let query = simplify_request(&request_line); //this is the data but after the slash that was entered

    if query == "" {
        //this may just display an index.html guide to the api
        let status_line = "HTTP/1.1 200 OK";
        let contents = "hi";
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 200 OK";

        //this is identifying what needs to be done.

        let segmented_query = split_list(query.clone(), '/');
        let contents = take_action(segmented_query);

        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();

        //note: there is no 404.html exception as this is handled by return_file()
    }
}

fn simplify_request(request: &String) -> String {
    //note, this is done weirdly as we don't know the length of the file requested
    let length = request.len();
    let removed_http_method = &request.to_string()[5..length]; //removing start

    let reversed = &removed_http_method.chars().rev().collect::<String>(); //reversing

    let length = reversed.len(); //overwriting length as new

    let removing_status_code = &reversed[9..length]; //removing start of reversed (so end of normal)

    let query = &removing_status_code.chars().rev().collect::<String>(); //this reverses again for normal

    let casted_query: String = query.to_owned(); //casting to String and owning
    casted_query
}

fn split_list(mut query: String, symbol: char) -> Vec<String> {
    //the query structure will be as following: domain/key/action/actionparams
    //domain will be ignored as that is removed by simplify_request

    let slash_position = query.chars().position(|c| c == symbol).unwrap(); // this finds the first slash

    let key = String::from(&query[0..slash_position]);

    query.replace_range(0..slash_position + 1, ""); //the +1 is to remove the /

    let slash_position = query.chars().position(|c| c == symbol).unwrap();
    let action = String::from(&query[0..slash_position]);

    query.replace_range(0..slash_position + 1, "");

    let parameters = query;

    let result_vec: Vec<String> = vec![key, action, parameters];
    result_vec
}

//the password is seckey (just for testing)
fn take_action(segmented_query: Vec<String>) -> String {
    let key = &segmented_query[0];
    let action = &segmented_query[1];
    let params = &segmented_query[2];

    let db_connection = &mut database::connect(); //connecting to the database

    let hash = "$2b$12$99j47M.VczzH9iX0pg6E6O5nJ2tsB.KtGP0jgI/MOsegAqJQNk5Am";
    let authorised  = verify(key, &hash).unwrap();

    if authorised  == true {
        //authentication necessary as it is a password manager, probably use hashed keys with a hashed client end authentication

        if action == "list_all" {
            let result = database::display(db_connection);
            result
        } else if action == "list_row" {
            let result = database::display_some(db_connection, params.to_string());
            result
        } else if action == "add" {
            let fields = split_list(params.to_string(), ',');
            database::create(db_connection, &fields[0], &fields[1], &fields[2]);
            "Created new entry".to_string()
        } else if action == "delete" {
            database::delete(params, db_connection);
            "Deleted password".to_string()
        } else {
            "invalid action".to_string()
        }
    } else {
        "invalid_authentication".to_string()
    }
}
