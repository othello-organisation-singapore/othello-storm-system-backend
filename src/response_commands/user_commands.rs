use diesel::PgConnection;
use rocket::http::Cookies;
use rocket_contrib::json::JsonValue;

use super::ResponseCommand;
use super::super::account::Account;
use super::super::utils::hash;

pub struct GetUserCommand {
    pub username: String
}

impl ResponseCommand for GetUserCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, String> {
        let account = Account::get(&self.username, &connection)?;
        Ok(json!(account.generate_meta()))
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
        Account::create_new_admin(&self.username, &self.display_name, &hashed_password, connection)?;
        Ok(json!({"message": "User created"}))
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
        let mut account = Account::login_from_cookies(&self.cookies, &connection)?;

        if !(self.is_able_to_update_user(&self.username, &account)) {
            return Err(
                String::from("You are not authorized to change other people profile.")
            );
        }

        let display_name = match &self.display_name {
            Some(name) => Option::from(name),
            None => Option::None
        };
        let password = match &self.password {
            Some(password) => Option::from(password),
            None => Option::None
        };

        account.update(display_name, password, connection)?;
        Ok(json!({"message": "User updated"}))
    }
}
