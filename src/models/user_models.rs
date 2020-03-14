use super::super::schema::users;
use super::super::properties::UserRole;

#[derive(Queryable)]
struct UserRowWrapper {
    pub id: i32,
    pub username: String,
    pub display_name: String,
    pub hashed_password: String,
    pub role: String,
}

#[derive(Insertable)]
#[table_name="users"]
struct NewUserRowWrapper<'a> {
    pub username: &'a str,
    pub display_name: &'a str,
    pub hashed_password: &'a str,
    pub role: &'a str,
}

pub struct User {
    pub username: String,
    pub display_name: String,
    pub hashed_password: String,
    pub role: UserRole,
}

impl User {

}
