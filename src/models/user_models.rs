use std::io;

use diesel::prelude::*;
use pwhash::bcrypt;

use super::super::schema::users;
use super::super::properties::UserRole;
use super::super::utils::establish_connection;

#[derive(Queryable)]
struct UserRowWrapper {
    pub id: i32,
    pub username: String,
    pub display_name: String,
    pub hashed_password: String,
    pub role: String,
}

#[derive(Insertable)]
#[table_name = "users"]
struct NewUserRowWrapper<'a> {
    pub username: &'a str,
    pub display_name: &'a str,
    pub hashed_password: &'a str,
    pub role: &'a str,
}

pub struct User {
    pub username: String,
    pub display_name: String,
    hashed_password: String,
    role: UserRole,
}

impl User {
    pub fn create_new_admin(username: String, display_name: String, password: String) -> Result<User, io::Error> {
        if User::is_username_exists(&username) {
            panic!("Username exists")
        }
        let hashed_password = bcrypt::hash(password).unwrap();
        let user = User {
            username,
            display_name,
            hashed_password,
            role: UserRole::from_string(String::from("admin")),
        };
        user.insert_to_database();
        Ok(user)
    }

    fn is_username_exists(username: &String) -> bool {
        if let Ok(_) = User::get_user(username) {
            true
        } else {
            false
        }
    }

    fn get_user(username: &String) -> Result<User, io::Error> {
        let connection = establish_connection();
        let user_row_wrapper = users::table
            .filter(users::username.eq(&username))
            .first::<UserRowWrapper>(&connection)
            .expect("User not found");
        Ok(User {
            username: user_row_wrapper.username,
            display_name: user_row_wrapper.display_name,
            hashed_password: user_row_wrapper.hashed_password,
            role: UserRole::from_string(user_row_wrapper.role),
        })
    }

    fn insert_to_database(&self) {
        let connection = establish_connection();
        diesel::insert_into(users::table)
            .values(NewUserRowWrapper {
                username: self.username.as_ref(),
                display_name: self.display_name.as_ref(),
                hashed_password: self.hashed_password.as_ref(),
                role: self.role.to_string().as_ref(),
            })
            .get_result::<UserRowWrapper>(&connection)
            .expect("Cannot create new user.");
    }

//    pub fn login(username: String, password: String) -> Result<User, io::Error> {
//
//    }
//
//    fn is_password_correct(&self, password: String) -> bool {
//
//    }
//
//    pub fn generate_jwt_token(&self) -> String {
//
//    }

    pub fn is_allowed_to_access(&self, allowed_role: Vec<UserRole>) -> bool {
        allowed_role.contains(&self.role)
    }
}
