use diesel::{connection, prelude::*};
//importing schema.rs and models.rs
pub mod schema;
pub mod models;
use self::models::*;

fn main() {
    use schema::passwords::dsl::passwords;
    
    let connection = &mut connect_to_db();
    
    let result: Vec<Password> = passwords.load(connection).expect("error connecting to database"); // reading the db
    println!("{:?}", result);
}

//this connects to a database
fn connect_to_db()-> SqliteConnection {
    SqliteConnection::establish("main.db").unwrap_or_else(|_| panic!("Error connecting to main.db"))
}
