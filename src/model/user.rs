use crate::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub hashed_password: String,
    pub salt: String,
    pub full_name: Option<String>,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub hashed_password: &'a str,
    pub salt: &'a str,
    pub full_name: &'a str,
}