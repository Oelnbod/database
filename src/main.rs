//use self::models::*;
pub mod database;

//using serde for parsing json
use serde_json;

fn main() {
    //connecting to the database
    let connection = &mut database::connect();

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
}
