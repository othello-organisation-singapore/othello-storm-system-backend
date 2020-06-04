use std::collections::HashMap;

use diesel::prelude::*;

use super::super::database_models::User;
use super::super::properties::UserRole;
use super::super::utils::JWTMediator;

pub trait Account {
    fn has_superuser_access(&self) -> bool;
    fn has_admin_access(&self) -> bool;
    fn get_username(&self) -> String;
    fn generate_meta(&self) -> HashMap<String, String>;

    fn create_new_admin(&self, username: &String, display_name: &String, hashed_password: &String,
                        connection: &PgConnection) -> Result<(), String> {
        if !self.has_superuser_access() {
            return Err(String::from("Only superuser can create new admin account."));
        }
        User::create(username, display_name, hashed_password, UserRole::Admin, connection)
    }

    fn generate_jwt(&self) -> Result<String, String> {
        let username = self.get_username();
        JWTMediator::generate_jwt_from_username(&username)
    }
}
