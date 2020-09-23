use diesel::PgConnection;
use rocket::http::Cookies;
use rocket_contrib::json::JsonValue;

use super::ResponseCommand;
use crate::account::Account;
use crate::database_models::UserRowModel;
use crate::meta_generator::{MetaGenerator, UserMetaGenerator};
use crate::utils::hash;
use crate::properties::UserRole;

pub struct GetUserCommand {
    pub username: String,
}

impl ResponseCommand for GetUserCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, String> {
        let meta_generator = UserMetaGenerator::from_username(
            &self.username, connection
        )?;
        Ok(json!(meta_generator.generate_meta()))
    }
}

pub struct CreateUserCommand<'a> {
    pub cookies: Cookies<'a>,
    pub username: String,
    pub display_name: String,
    pub password: String,
}

impl ResponseCommand for CreateUserCommand<'_> {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, String> {
        let account = Account::login_from_cookies(&self.cookies, &connection)?;

        if !account.has_superuser_access() {
            return Err(String::from(
                "You are not authorized to create an admin account."
            ));
        }
        let hashed_password = hash(&self.password);
        UserRowModel::create(
            &self.username,
            &self.display_name,
            &hashed_password,
            UserRole::Admin,
            connection
        )?;
        Ok(json!({"message": "User created."}))
    }
}

pub struct UpdateUserCommand<'a> {
    pub cookies: Cookies<'a>,
    pub username: String,
    pub display_name: Option<String>,
    pub password: Option<String>,
}

impl UpdateUserCommand<'_> {
    fn is_able_to_update_user(&self, username: &String, current_account: &Account) -> bool {
        current_account.has_superuser_access() || &current_account.get_username() == username
    }
}

impl ResponseCommand for UpdateUserCommand<'_> {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, String> {
        let account = Account::login_from_cookies(&self.cookies, &connection)?;

        if !(self.is_able_to_update_user(&self.username, &account)) {
            return Err(
                String::from("You are not authorized to change other people profile.")
            );
        }
        let mut user_model = account.user;

        if let Some(name) = &self.display_name {
            user_model.display_name = name.clone()
        }
        if let Some(password) = &self.password {
            user_model.hashed_password = hash(password)
        }
        user_model.update(connection)?;
        Ok(json!({"message": "User updated."}))
    }
}
