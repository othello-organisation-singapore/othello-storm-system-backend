#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub display_name: String,
    pub hashed_password: String,
    pub role: String,
}
