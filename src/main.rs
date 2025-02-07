//use self::models::*;
use serde_json;

fn main() {
    let connection = &mut database::connect();

    // the order of what follows is just for testing and can be changed

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

mod database {
    //importing database/schema.rs, models.rs, diesel and various lower levels of schema.rs and models.rs
    use crate::database::schema::passwords::password;
    use diesel::prelude::*;
    
    pub mod models;
    pub mod schema;
    use crate::database::models::*;
    use crate::database::schema::passwords::dsl::{passwords, website};
    
    //this connects to a database
    pub fn connect() -> SqliteConnection {
        SqliteConnection::establish("main.db")
            .unwrap_or_else(|_| panic!("Error connecting to main.db"))
    }

    //reading the database
    pub fn display(connection: &mut SqliteConnection) -> Vec<Password> {
        let result: Vec<Password> = passwords
            .load(connection)
            .expect("error connecting to database");
        result
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
}
