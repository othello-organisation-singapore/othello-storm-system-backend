use std::io;

use diesel::prelude::*;
use pwhash::bcrypt;

use super::super::schema::users;
use super::super::properties::UserRole;
use super::super::utils::establish_connection;

#[derive(Queryable)]
pub struct User {
    id: i32,
    pub username: String,
    pub display_name: String,
    hashed_password: String,
    role: String,
}

#[derive(Insertable)]
#[table_name = "users"]
struct NewUser<'a> {
    pub username: &'a str,
    pub display_name: &'a str,
    pub hashed_password: &'a str,
    pub role: &'a str,
}

impl User {
    pub fn get(username: &String) -> Result<User, io::Error> {
        let connection = establish_connection();
        let user = users::table
            .filter(users::username.eq(&username))
            .first::<User>(&connection)
            .expect("User not found.");
        Ok(user)
    }
}

impl User {
    pub fn create_new_superuser(username: String, display_name: String, password: String) -> Result<User, io::Error> {
        User::create_new_user(username, display_name, password, UserRole::Superuser)
    }

    pub fn create_new_admin(username: String, display_name: String, password: String) -> Result<User, io::Error> {
        User::create_new_user(username, display_name, password, UserRole::Admin)
    }

    fn create_new_user(username: String, display_name: String, password: String, role: UserRole) -> Result<User, io::Error> {
        if User::is_username_exists(&username) {
            panic!("Username exists.")
        }
        let new_user = NewUser {
            username: &username,
            display_name: &display_name,
            hashed_password: &User::hash_password(password),
            role: &role.to_string(),
        };
        User::insert_to_database(new_user);
        User::get(&username)
    }

    fn is_username_exists(username: &String) -> bool {
        if let Ok(_) = User::get(username) {
            return true;
        }
        false
    }

    fn hash_password(password: String) -> String {
        bcrypt::hash(password).unwrap()
    }

    fn insert_to_database(new_user: NewUser) {
        let connection = establish_connection();
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(&connection)
            .expect("Cannot create new user.");
    }

    pub fn update(username: String, display_name: String, password: String) {
        let hashed_password = User::hash_password(password);
        let connection = establish_connection();
        diesel::update(users::table.filter(users::username.eq(&username)))
            .set((
                users::display_name.eq(&display_name),
                users::hashed_password.eq(&hashed_password),
            ))
            .get_result::<User>(&connection)
            .expect("Failed to update.");
    }

    pub fn login(username: String, password: String) -> Result<User, io::Error> {
        let user = User::get(&username)?;
        if user.is_password_correct(password) {
            return Ok(user);
        }
        panic!("Password mismatch.")
    }

    fn is_password_correct(&self, password: String) -> bool {
        bcrypt::verify(password, &self.hashed_password)
    }
}
