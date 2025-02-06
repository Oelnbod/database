use diesel::prelude::*;
use schema::passwords::password;
//importing schema.rs and models.rs
pub mod models;
pub mod schema;
use self::models::*;
use serde_json;
use crate::schema::passwords::dsl::{passwords, website};

fn main() {
    
    let connection = &mut connect_to_db();

    // the order of what follows is just for testing and can be changed

    //displaying the results
    let results = display_database(connection);
    println!("{:?}", results);

    //writing the password, it returns the row added (I think)
    let password_out = create_password(connection, "example.com", "example", "hi");
    let password_out = create_password(connection, "example.com", "example", "hi");

    //displaying the results
    let result = display_database(connection);
    let json = serde_json::to_string(&result);

    println!("\n {:?}", json);
    
    //deleting the database
    delete_from_database("example.com", connection);

    //displaying the database again.
    println!("\n \n {:?}", display_database(connection));
}

//this connects to a database
fn connect_to_db() -> SqliteConnection {
    SqliteConnection::establish("main.db").unwrap_or_else(|_| panic!("Error connecting to main.db"))
}

//reading the database
fn display_database(connection: &mut SqliteConnection) -> Vec<Password> {
    
    let result: Vec<Password> = passwords
        .load(connection)
        .expect("error connecting to database");
    result
}
//creating a new entry
fn create_password(
    conn: &mut SqliteConnection,
    website_input: &str,
    username_input: &str,
    password_input: &str,
) -> Password {
    use crate::schema::passwords;
    let new_password_data = NewPassword {
        website: website_input,
        username: username_input,
        password: password_input,
    };

    diesel::insert_into(passwords::table)
        .values(&new_password_data)
        .returning(Password::as_returning())
        .get_result(conn)
        .expect("Error saving password data")
}

//deleting the database
fn delete_from_database(target: &str, connection: &mut SqliteConnection) {
        let num_deleted = diesel::delete(passwords.filter(website.like(format!("%{}%", target))))
        .execute(connection)
        .expect("Error deleting entries.");
}
