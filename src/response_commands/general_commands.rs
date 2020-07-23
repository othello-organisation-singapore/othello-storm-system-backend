use diesel::PgConnection;
use rocket::http::Cookies;
use rocket_contrib::json::JsonValue;

use super::ResponseCommand;
use crate::account::Account;

pub struct LoginCommand {
    pub username: String,
    pub password: String,
}

impl ResponseCommand for LoginCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, String> {
        let account = Account::login_from_password(
            &self.username, &self.password, &connection,
        )?;

        info!("{} is logged in.", account.get_username());

        let jwt = account.generate_jwt()?;
        Ok(json!({"jwt": jwt}))
    }
}


pub struct CurrentUserCommand<'a> {
    pub cookies: Cookies<'a>
}

impl ResponseCommand for CurrentUserCommand<'_> {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, String> {
        let account = Account::login_from_cookies(&self.cookies, &connection)?;
        Ok(json!(account.generate_meta()))
    }
}
