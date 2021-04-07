use diesel::prelude::*;
use rocket::http::Cookies;

use crate::database_models::UserRowModel;
use crate::errors::ErrorType;
use crate::properties::UserRole;
use crate::utils::{verify, JWTMediator};

pub struct Account {
    pub user: UserRowModel,
}

impl Account {
    pub fn has_superuser_access(&self) -> bool {
        match self.user.get_role() {
            UserRole::Superuser => true,
            UserRole::Admin => false,
            _ => false,
        }
    }

    pub fn has_admin_access(&self) -> bool {
        match self.user.get_role() {
            UserRole::Superuser => true,
            UserRole::Admin => true,
            _ => false,
        }
    }

    pub fn get_username(&self) -> String {
        self.user.username.clone()
    }

    pub fn generate_jwt(&self) -> Result<String, ErrorType> {
        JWTMediator::generate_jwt_from_username(&self.user.username)
    }
}

impl Account {
    pub fn login_from_cookies(
        cookies: &Cookies,
        connection: &PgConnection,
    ) -> Result<Account, ErrorType> {
        let cookies_jwt = cookies.get("jwt").map(|c| c.value()).unwrap_or("");
        let jwt = String::from(cookies_jwt);
        Account::login_from_jwt(&jwt, connection)
    }

    pub fn login_from_jwt(jwt: &String, connection: &PgConnection) -> Result<Account, ErrorType> {
        let username = JWTMediator::get_username_from_jwt(jwt)?;

        let user = UserRowModel::get(&username, connection)?;
        Account::get_account_from_user(user)
    }

    pub fn login_from_password(
        username: &String,
        password: &String,
        connection: &PgConnection,
    ) -> Result<Account, ErrorType> {
        let user = UserRowModel::get(&username, connection)?;

        let is_password_correct = verify(password, &user.hashed_password);
        if !is_password_correct {
            return Err(ErrorType::AuthenticationFailed);
        }
        Account::get_account_from_user(user)
    }

    pub fn get(username: &String, connection: &PgConnection) -> Result<Account, ErrorType> {
        let user = UserRowModel::get(&username, connection)?;
        Account::get_account_from_user(user)
    }

    fn get_account_from_user(user: UserRowModel) -> Result<Account, ErrorType> {
        match user.get_role() {
            UserRole::Superuser => Ok(Account { user }),
            UserRole::Admin => Ok(Account { user }),
            _ => Err(ErrorType::AuthenticationFailed),
        }
    }
}

#[cfg(test)]
mod tests {
    mod test_login_from_password {
        use crate::account::Account;
        use crate::database_models::UserRowModel;
        use crate::properties::UserRole;
        use crate::utils;

        #[test]
        fn test_superuser_login() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);
            let display_name = utils::generate_random_string(20);
            let password = utils::generate_random_string(30);
            let hashed_password = utils::hash(&password);

            let result = UserRowModel::create(
                &username,
                &display_name,
                &hashed_password,
                UserRole::Superuser,
                &test_connection,
            );

            assert_eq!(result.is_ok(), true);
            let account =
                Account::login_from_password(&username, &password, &test_connection).unwrap();
            assert_eq!(account.has_superuser_access(), true);
            assert_eq!(account.has_admin_access(), true);
            assert_eq!(account.get_username(), username);
        }

        #[test]
        fn test_admin_login() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);
            let display_name = utils::generate_random_string(20);
            let password = utils::generate_random_string(30);
            let hashed_password = utils::hash(&password);

            let result = UserRowModel::create(
                &username,
                &display_name,
                &hashed_password,
                UserRole::Admin,
                &test_connection,
            );

            assert_eq!(result.is_ok(), true);
            let account =
                Account::login_from_password(&username, &password, &test_connection).unwrap();
            assert_eq!(account.has_superuser_access(), false);
            assert_eq!(account.has_admin_access(), true);
            assert_eq!(account.get_username(), username);
        }

        #[test]
        fn test_login_incorrect_username() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);
            let display_name = utils::generate_random_string(20);
            let password = utils::generate_random_string(30);
            let hashed_password = utils::hash(&password);

            let result = UserRowModel::create(
                &username,
                &display_name,
                &hashed_password,
                UserRole::Superuser,
                &test_connection,
            );

            assert_eq!(result.is_ok(), true);
            let login_result =
                Account::login_from_password(&display_name, &password, &test_connection);
            assert_eq!(login_result.is_err(), true);
        }

        #[test]
        fn test_login_incorrect_password() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);
            let display_name = utils::generate_random_string(20);
            let password = utils::generate_random_string(30);
            let hashed_password = utils::hash(&password);

            let result = UserRowModel::create(
                &username,
                &display_name,
                &hashed_password,
                UserRole::Superuser,
                &test_connection,
            );

            assert_eq!(result.is_ok(), true);
            let login_result =
                Account::login_from_password(&username, &hashed_password, &test_connection);
            assert_eq!(login_result.is_err(), true);
        }
    }

    mod test_login_from_jwt {
        use crate::account::Account;
        use crate::database_models::UserRowModel;
        use crate::properties::UserRole;
        use crate::utils;

        #[test]
        fn test_superuser_login() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);
            let display_name = utils::generate_random_string(20);
            let password = utils::generate_random_string(30);
            let hashed_password = utils::hash(&password);

            let result = UserRowModel::create(
                &username,
                &display_name,
                &hashed_password,
                UserRole::Superuser,
                &test_connection,
            );
            assert_eq!(result.is_ok(), true);
            let jwt = utils::JWTMediator::generate_jwt_from_username(&username).unwrap();

            let account = Account::login_from_jwt(&jwt, &test_connection).unwrap();
            assert_eq!(account.has_superuser_access(), true);
            assert_eq!(account.has_admin_access(), true);
            assert_eq!(account.get_username(), username);
        }

        #[test]
        fn test_admin_login() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);
            let display_name = utils::generate_random_string(20);
            let password = utils::generate_random_string(30);
            let hashed_password = utils::hash(&password);

            let result = UserRowModel::create(
                &username,
                &display_name,
                &hashed_password,
                UserRole::Admin,
                &test_connection,
            );
            assert_eq!(result.is_ok(), true);
            let jwt = utils::JWTMediator::generate_jwt_from_username(&username).unwrap();

            let account = Account::login_from_jwt(&jwt, &test_connection).unwrap();
            assert_eq!(account.has_superuser_access(), false);
            assert_eq!(account.has_admin_access(), true);
            assert_eq!(account.get_username(), username);
        }

        #[test]
        fn test_login_incorrect_username() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);

            let jwt = utils::JWTMediator::generate_jwt_from_username(&username).unwrap();
            let login_result = Account::login_from_jwt(&jwt, &test_connection);
            assert_eq!(login_result.is_err(), true);
        }

        #[test]
        fn test_login_random_jwt() {
            let test_connection = utils::get_test_connection();
            let jwt = utils::generate_random_string(20);
            let login_result = Account::login_from_jwt(&jwt, &test_connection);
            assert_eq!(login_result.is_err(), true);
        }
    }
}
