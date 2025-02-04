use diesel::prelude::*;
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::passwords)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Password {
    pub id: i32,
    pub website: String,
    pub username: String,
    pub password: String,
}

use crate::schema::passwords;

#[derive(Insertable)]
#[diesel(table_name = passwords)]
pub struct NewPassword<'a> {
    pub website: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}
