use diesel::PgConnection;
use rocket::http::Cookies;
use rocket_contrib::json::JsonValue;

use crate::account::Account;
use crate::errors::ErrorType;
use crate::meta_generator::{MetaGenerator, UserMetaGenerator};

use super::ResponseCommand;

pub struct LoginCommand {
    pub username: String,
    pub password: String,
}

impl ResponseCommand for LoginCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let account = Account::login_from_password(
            &self.username, &self.password, &connection,
        )?;

        info!("{} is logged in.", account.get_username());

        let jwt = account.generate_jwt()?;
        Ok(json!({"jwt": jwt}))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!("Login for {}", &self.username))
    }
}


pub struct CurrentUserCommand<'a> {
    pub cookies: Cookies<'a>
}

impl ResponseCommand for CurrentUserCommand<'_> {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let account = Account::login_from_cookies(&self.cookies, connection)?;
        let meta_generator = UserMetaGenerator::from_username(
            &account.get_username(), connection,
        )?;
        Ok(json!(meta_generator.generate_meta()))
    }

    fn get_request_summary(&self) -> String {
        String::from("GetCurrentUser")
    }
}
