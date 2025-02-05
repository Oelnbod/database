use diesel::{connection, prelude::*};
use schema::passwords::password;
//importing schema.rs and models.rs
pub mod models;
pub mod schema;
use self::models::*;

fn main() {
    use schema::passwords::dsl::{passwords, website};

    let connection = &mut connect_to_db();

    // the order of what follows is just for testing and can be changed

    //displaying the results
    let results = display_database(connection);
    println!("{:?}", results);

    //writing the password, it returns the row added (I think)
    let password_out = create_password(connection, "example.com", "example", "hi");

    //displaying the results
    let result = display_database(connection);
    println!("\n {:?}", result);

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
    use schema::passwords::dsl::passwords;
    let result: Vec<Password> = passwords
        .load(connection)
        .expect("error connecting to database");
    result
}
//creating a new entry
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

//deleting the database
fn delete_from_database(target: &str, connection: SqliteConnection) {
    let num_deleted = diesel::delete(passwords.filter(website.like(pattern)))
        .execute(connection)
        .expect("Error deleting entries.");
}
