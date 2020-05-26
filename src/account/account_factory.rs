use diesel::prelude::*;

use super::account::Account;
use super::account_admin::AdminAccount;
use super::account_superuser::SuperuserAccount;
use super::account_visitor::VisitorAccount;
use super::super::database_models::User;
use super::super::properties::UserRole;
use super::super::utils::JWTMediator;
use super::super::utils::verify;

pub struct AccountFactory {}

impl AccountFactory {
    pub fn login_from_jwt(jwt: &String, connection: &PgConnection) -> Box<dyn Account> {
        let username = match JWTMediator::get_username_from_jwt(jwt) {
            Ok(username) => username,
            Err(_) => return Box::new(VisitorAccount {})
        };

        let user = match User::get(&username, connection) {
            Ok(user) => user,
            Err(_) => return Box::new(VisitorAccount {})
        };
        AccountFactory::get_account_from_user(user)
    }

    pub fn login_from_password(username: &String, password: &String, connection: &PgConnection)
                               -> Box<dyn Account> {
        let user = match User::get(&username, connection) {
            Ok(user) => user,
            Err(_) => return Box::new(VisitorAccount {})
        };

        let is_password_correct = verify(password, &user.hashed_password);
        if !is_password_correct {
            return Box::new(VisitorAccount {});
        }
        AccountFactory::get_account_from_user(user)
    }

    fn get_account_from_user(user: User) -> Box<dyn Account> {
        match user.get_role() {
            UserRole::Superuser => Box::new(SuperuserAccount::from_user(user).unwrap()),
            UserRole::Admin => Box::new(AdminAccount::from_user(user).unwrap()),
            _ => Box::new(VisitorAccount {})
        }
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
            );
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
            );
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
            let account = AccountFactory::login_from_password(
                &display_name, &password, &test_connection
            );
            assert_eq!(account.has_superuser_access(), false);
            assert_eq!(account.has_admin_access(), false);
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
            let account = AccountFactory::login_from_password(
                &username, &hashed_password, &test_connection
            );
            assert_eq!(account.has_superuser_access(), false);
            assert_eq!(account.has_admin_access(), false);
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

            let account = AccountFactory::login_from_jwt(&jwt, &test_connection);
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

            let account = AccountFactory::login_from_jwt(&jwt, &test_connection);
            assert_eq!(account.has_superuser_access(), false);
            assert_eq!(account.has_admin_access(), true);
            assert_eq!(account.get_username(), username);
        }

        #[test]
        fn test_login_incorrect_username() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);

            let jwt = utils::JWTMediator::generate_jwt_from_username(&username).unwrap();
            let account = AccountFactory::login_from_jwt(&jwt, &test_connection);
            assert_eq!(account.has_superuser_access(), false);
            assert_eq!(account.has_admin_access(), false);
        }

        #[test]
        fn test_login_random_jwt() {
            let test_connection = utils::get_test_connection();
            let jwt = utils::generate_random_string(20);
            let account = AccountFactory::login_from_jwt(&jwt, &test_connection);
            assert_eq!(account.has_superuser_access(), false);
            assert_eq!(account.has_admin_access(), false);
        }
    }
}
