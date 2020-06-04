use rocket::http::Cookies;
use diesel::prelude::*;

use super::account::Account;
use super::account_admin::AdminAccount;
use super::account_superuser::SuperuserAccount;
use super::super::database_models::User;
use super::super::properties::UserRole;
use super::super::utils::JWTMediator;
use super::super::utils::verify;

pub struct AccountFactory {}

impl AccountFactory {
    pub fn login_from_jwt(jwt: &String, connection: &PgConnection) -> Result<Box<dyn Account>, String> {
        let username = match JWTMediator::get_username_from_jwt(jwt) {
            Ok(username) => username,
            Err(_) => return Err(String::from("Login expired."))
        };

        let user = match User::get(&username, connection) {
            Ok(user) => user,
            Err(_) => return Err(String::from("Requester user not found."))
        };
        AccountFactory::get_account_from_user(user)
    }

    pub fn login_from_password(username: &String, password: &String, connection: &PgConnection)
                               -> Result<Box<dyn Account>, String> {
        let user = match User::get(&username, connection) {
            Ok(user) => user,
            Err(_) => return Err(String::from("Username not found."))
        };

        let is_password_correct = verify(password, &user.hashed_password);
        if !is_password_correct {
            return Err(String::from("Incorrect password."))
        }
       AccountFactory::get_account_from_user(user)
    }

    fn get_account_from_user(user: User) -> Result<Box<dyn Account>, String> {
        match user.get_role() {
            UserRole::Superuser => Ok(Box::new(SuperuserAccount::from_user(user).unwrap())),
            UserRole::Admin => Ok(Box::new(AdminAccount::from_user(user).unwrap())),
            _ => Err(String::from("Something is wrong, please contact admin."))
        }
    }

    pub fn login_from_cookies(cookies: Cookies, connection: &PgConnection) -> Result<Box<dyn Account>, String> {
        let cookies_jwt = cookies.get("jwt").map(|c| c.value()).unwrap_or("");
        let jwt = String::from(cookies_jwt);
        AccountFactory::login_from_jwt(&jwt, connection)
    }

    pub fn get(username: &String, connection: &PgConnection) -> Result<Box<dyn Account>, String> {
        let user = match User::get(&username, connection) {
            Ok(user) => user,
            Err(_) => return Err(String::from("Username not found."))
        };
        AccountFactory::get_account_from_user(user)
    }

}

#[cfg(test)]
mod tests {
    mod test_login_from_password {
        use crate::account::account_factory::AccountFactory;
        use crate::database_models::User;
        use crate::properties::UserRole;
        use crate::utils;

        #[test]
        fn test_superuser_login() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);
            let display_name = utils::generate_random_string(20);
            let password = utils::generate_random_string(30);
            let hashed_password = utils::hash(&password);

            let result = User::create(
                &username,
                &display_name,
                &hashed_password,
                UserRole::Superuser,
                &test_connection,
            );

            assert_eq!(result.is_ok(), true);
            let account = AccountFactory::login_from_password(
                &username, &password, &test_connection
            ).unwrap();
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

            let result = User::create(
                &username,
                &display_name,
                &hashed_password,
                UserRole::Admin,
                &test_connection,
            );

            assert_eq!(result.is_ok(), true);
            let account = AccountFactory::login_from_password(
                &username, &password, &test_connection
            ).unwrap();
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

            let result = User::create(
                &username,
                &display_name,
                &hashed_password,
                UserRole::Superuser,
                &test_connection,
            );

            assert_eq!(result.is_ok(), true);
            let login_result = AccountFactory::login_from_password(
                &display_name, &password, &test_connection
            );
            assert_eq!(login_result.is_err(), true);
        }

        #[test]
        fn test_login_incorrect_password() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);
            let display_name = utils::generate_random_string(20);
            let password = utils::generate_random_string(30);
            let hashed_password = utils::hash(&password);

            let result = User::create(
                &username,
                &display_name,
                &hashed_password,
                UserRole::Superuser,
                &test_connection,
            );

            assert_eq!(result.is_ok(), true);
            let login_result = AccountFactory::login_from_password(
                &username, &hashed_password, &test_connection
            );
            assert_eq!(login_result.is_err(), true);
        }
    }

    mod test_login_from_jwt {
        use crate::account::account_factory::AccountFactory;
        use crate::database_models::User;
        use crate::properties::UserRole;
        use crate::utils;

        #[test]
        fn test_superuser_login() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);
            let display_name = utils::generate_random_string(20);
            let password = utils::generate_random_string(30);
            let hashed_password = utils::hash(&password);

            let result = User::create(
                &username,
                &display_name,
                &hashed_password,
                UserRole::Superuser,
                &test_connection,
            );
            assert_eq!(result.is_ok(), true);
            let jwt = utils::JWTMediator::generate_jwt_from_username(&username).unwrap();

            let account = AccountFactory::login_from_jwt(
                &jwt, &test_connection
            ).unwrap();
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

            let result = User::create(
                &username,
                &display_name,
                &hashed_password,
                UserRole::Admin,
                &test_connection,
            );
            assert_eq!(result.is_ok(), true);
            let jwt = utils::JWTMediator::generate_jwt_from_username(&username).unwrap();

            let account = AccountFactory::login_from_jwt(
                &jwt, &test_connection
            ).unwrap();
            assert_eq!(account.has_superuser_access(), false);
            assert_eq!(account.has_admin_access(), true);
            assert_eq!(account.get_username(), username);
        }

        #[test]
        fn test_login_incorrect_username() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);

            let jwt = utils::JWTMediator::generate_jwt_from_username(&username).unwrap();
            let login_result = AccountFactory::login_from_jwt(&jwt, &test_connection);
            assert_eq!(login_result.is_err(), true);
        }

        #[test]
        fn test_login_random_jwt() {
            let test_connection = utils::get_test_connection();
            let jwt = utils::generate_random_string(20);
            let login_result = AccountFactory::login_from_jwt(
                &jwt, &test_connection
            );
            assert_eq!(login_result.is_err(), true);
        }
    }
}
