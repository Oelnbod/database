//importing diesel
use diesel::prelude::*;

//using the models.rs and schema.rs scripts
pub mod models;
pub mod schema;

//using features from the models.rs and schema.rs scripts
use crate::database::models::*;
use crate::database::schema::passwords::dsl::{passwords, website};
use crate::database::schema::passwords::password;
use serde_json;

//this identifies the project directory for identifying the location of main.db
const PROJECT_DIR: &str = env!("CARGO_MANIFEST_DIR");

//this connects to a database
pub fn connect() -> SqliteConnection {
    let database = format!("{}/main.db", PROJECT_DIR);
    SqliteConnection::establish(database.clone().as_str())
        .unwrap_or_else(|_| panic!("Error connecting to {}", database))
}

//reading the database
pub fn display(connection: &mut SqliteConnection) -> String {
    let result: Vec<Password> = passwords
        .load(connection)
        .expect("error displaying database");
    let json_result =
        serde_json::to_string(&result).expect("error converting Vec<Password> to json");
    json_result
}
pub fn display_some(connection: &mut SqliteConnection, target: String) -> String {
    let result: Vec<Password> = passwords
        .filter(website.eq(target))
        .load(connection)
        .expect("error displaying database");
    let json_result =
        serde_json::to_string(&result).expect("error converting Vec<Password to json");
    json_result
}

//creating a new entry
pub fn create(
    conn: &mut SqliteConnection,
    website_input: &str,
    username_input: &str,
    password_input: &str,
) -> Password {
    use crate::database::schema::passwords;
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
pub fn delete(target: &str, connection: &mut SqliteConnection) {
    let num_deleted = diesel::delete(passwords.filter(website.like(format!("%{}%", target))))
        .execute(connection)
        .expect("Error deleting entries.");
}
