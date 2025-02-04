use diesel::{connection, prelude::*};
use schema::passwords::password;
//importing schema.rs and models.rs
pub mod models;
pub mod schema;
use self::models::*;

fn main() {
    use schema::passwords::dsl::{website, passwords};
    
    let connection = &mut connect_to_db();

    let results = display_database(connection);
    println!("{:?}", results);
    
    let _password_out = create_password(connection, "example.com", "example", "hi");

    let result = display_database(connection);
    println!("after \n");
    println!("{:?}", result);

    delete_from_database("example.com", connection);
    
    println!("\n \n {:?}", display_database(connection));
}

//this connects to a database
fn connect_to_db() -> SqliteConnection {
    SqliteConnection::establish("main.db").unwrap_or_else(|_| panic!("Error connecting to main.db"))
}

fn display_database(connection: &mut SqliteConnection) -> Vec<Password> {
    use schema::passwords::dsl::passwords;
    let result: Vec<Password> = passwords
        .load(connection)
        .expect("error connecting to database"); // reading the db
    result

}

fn create_password(
    conn: &mut SqliteConnection,
    website: &str,
    username: &str,
    password_value: &str,
) -> Password {
    
    use crate::schema::passwords;
    let new_password_data = NewPassword {
        website,
        username,
        password: password_value,
    };

    diesel::insert_into(passwords::table)
        .values(&new_password_data)
        .returning(Password::as_returning())
        .get_result(conn)
        .expect("Error saving password data")
}

fn delete_from_database(target: &str, connection: SqliteConnection) {
    let num_deleted = diesel::delete(passwords.filter(website.like(pattern)))
        .execute(connection)
        .expect("Error deleting entries.");
}
