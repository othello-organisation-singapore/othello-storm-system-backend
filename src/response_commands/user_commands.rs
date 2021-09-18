use diesel::PgConnection;
use rocket_contrib::json::JsonValue;

use crate::account::Account;
use crate::database_models::UserRowModel;
use crate::errors::ErrorType;
use crate::meta_generator::{MetaGenerator, UserMetaGenerator};
use crate::properties::UserRole;
use crate::utils::hash;

use super::ResponseCommand;

pub struct GetUserCommand {
    pub username: String,
}

impl ResponseCommand for GetUserCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let meta_generator = UserMetaGenerator::from_username(&self.username, connection)?;
        Ok(json!(meta_generator.generate_meta()))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!("GetUser for {}", &self.username))
    }
}

pub struct CreateUserCommand {
    pub jwt: String,
    pub username: String,
    pub display_name: String,
    pub password: String,
}

impl ResponseCommand for CreateUserCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let account = Account::login_from_jwt(&self.jwt, &connection)?;

        if !account.has_superuser_access() {
            return Err(ErrorType::PermissionDenied);
        }
        let hashed_password = hash(&self.password);
        UserRowModel::create(
            &self.username,
            &self.display_name,
            &hashed_password,
            UserRole::Admin,
            connection,
        )?;
        Ok(json!({"message": "User created."}))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!("CreateUser for {}", &self.username))
    }
}

pub struct UpdateUserCommand {
    pub jwt: String,
    pub username: String,
    pub display_name: Option<String>,
    pub password: Option<String>,
}

impl UpdateUserCommand {
    fn is_able_to_update_user(&self, username: &String, current_account: &Account) -> bool {
        current_account.has_superuser_access() || &current_account.get_username() == username
    }
}

impl ResponseCommand for UpdateUserCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let account = Account::login_from_jwt(&self.jwt, &connection)?;

        if !(self.is_able_to_update_user(&self.username, &account)) {
            return Err(ErrorType::PermissionDenied);
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

    fn get_request_summary(&self) -> String {
        String::from(format!("UpdateUser for {}", &self.username))
    }
}
