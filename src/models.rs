use diesel::prelude::*;
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::passwords)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Password {
    pub id: i32,
    pub website: String,
    pub username: String,
    pub password: String
}

