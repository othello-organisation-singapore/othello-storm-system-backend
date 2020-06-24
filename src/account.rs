use std::collections::HashMap;

use rocket::http::Cookies;
use diesel::prelude::*;

use super::database_models::User;
use super::properties::UserRole;
use super::utils::{JWTMediator, hash, verify};

pub struct Account {
    user: User
}

impl Account {
    pub fn has_superuser_access(&self) -> bool {
        match self.user.get_role() {
            UserRole::Superuser => true,
            UserRole::Admin => false,
            _ => false
        }
    }

    pub fn has_admin_access(&self) -> bool {
        match self.user.get_role() {
            UserRole::Superuser => true,
            UserRole::Admin => true,
            _ => false
        }
    }

    pub fn create_new_admin(&self, username: &String, display_name: &String,
                            hashed_password: &String, connection: &PgConnection)
                            -> Result<(), String> {
        if !self.has_superuser_access() {
            return Err(String::from("Only superuser can create new admin account_test."));
        }
        User::create(username, display_name, hashed_password, UserRole::Admin, connection)
    }

    pub fn get_username(&self) -> String {
        self.user.username.clone()
    }

    pub fn update(&mut self, display_name: Option<&String>, password: Option<&String>,
                  connection: &PgConnection) -> Result<(), String> {
        if let Some(updated_display_name) = display_name {
            self.user.display_name = updated_display_name.clone();
        }
        if let Some(updated_password) = password {
            self.user.hashed_password = hash(updated_password);
        }
        self.user.update(connection)
    }

    pub fn generate_meta(&self) -> HashMap<String, String> {
        let mut meta: HashMap<String, String> = HashMap::new();
        meta.insert(String::from("username"), self.user.username.clone());
        meta.insert(String::from("display_name"), self.user.display_name.clone());
        meta.insert(String::from("role"), self.user.get_role().to_string());
        meta
    }

    pub fn generate_jwt(&self) -> Result<String, String> {
        let username = self.get_username();
        JWTMediator::generate_jwt_from_username(&username)
    }
}

impl Account {
    pub fn login_from_cookies(cookies: Cookies, connection: &PgConnection) -> Result<Account, String> {
        let cookies_jwt = cookies.get("jwt").map(|c| c.value()).unwrap_or("");
        let jwt = String::from(cookies_jwt);
        Account::login_from_jwt(&jwt, connection)
    }

    pub fn login_from_jwt(jwt: &String, connection: &PgConnection) -> Result<Account, String> {
        let username = match JWTMediator::get_username_from_jwt(jwt) {
            Ok(username) => username,
            Err(_) => return Err(String::from("Login expired."))
        };

        let user = match User::get(&username, connection) {
            Ok(user) => user,
            Err(_) => return Err(String::from("Requester user not found."))
        };
        Account::get_account_from_user(user)
    }

    pub fn login_from_password(username: &String, password: &String, connection: &PgConnection)
                               -> Result<Account, String> {
        let user = match User::get(&username, connection) {
            Ok(user) => user,
            Err(_) => return Err(String::from("Username not found."))
        };

        let is_password_correct = verify(password, &user.hashed_password);
        if !is_password_correct {
            return Err(String::from("Incorrect password."));
        }
        Account::get_account_from_user(user)
    }

    pub fn get(username: &String, connection: &PgConnection) -> Result<Account, String> {
        let user = match User::get(&username, connection) {
            Ok(user) => user,
            Err(_) => return Err(String::from("Username not found."))
        };
        Account::get_account_from_user(user)
    }

    fn get_account_from_user(user: User) -> Result<Account, String> {
        match user.get_role() {
            UserRole::Superuser => Ok(Account { user }),
            UserRole::Admin => Ok(Account { user }),
            _ => Err(String::from("Something is wrong, please contact admin."))
        }
    }
}

#[cfg(test)]
mod tests {
    mod test_login_from_password {
        use crate::account::Account;
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
            let account = Account::login_from_password(
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
            let account = Account::login_from_password(
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
            let login_result = Account::login_from_password(
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
            let login_result = Account::login_from_password(
                &username, &hashed_password, &test_connection
            );
            assert_eq!(login_result.is_err(), true);
        }
    }

    mod test_login_from_jwt {
        use crate::account::Account;
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

            let account = Account::login_from_jwt(
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

            let account = Account::login_from_jwt(
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
            let login_result = Account::login_from_jwt(&jwt, &test_connection);
            assert_eq!(login_result.is_err(), true);
        }

        #[test]
        fn test_login_random_jwt() {
            let test_connection = utils::get_test_connection();
            let jwt = utils::generate_random_string(20);
            let login_result = Account::login_from_jwt(
                &jwt, &test_connection
            );
            assert_eq!(login_result.is_err(), true);
        }
    }

    mod test_admin_creation {
        use crate::account::Account;
        use crate::database_models::User;
        use crate::properties::UserRole;
        use crate::utils;

        #[test]
        fn test_admin_create_admin() {
            let test_connection = utils::get_test_connection();

            let admin_username = utils::generate_random_string(20);
            let admin_display_name = utils::generate_random_string(20);
            let admin_password = utils::generate_random_string(30);
            let admin_hashed_password = utils::hash(&admin_password);
            let _ = User::create(
                &admin_username,
                &admin_display_name,
                &admin_hashed_password,
                UserRole::Admin,
                &test_connection
            );
            let user_admin = User::get(&admin_username, &test_connection).unwrap();
            let account = Account{user: user_admin};

            let username = utils::generate_random_string(20);
            let display_name = utils::generate_random_string(20);
            let password = utils::generate_random_string(30);
            let hashed_password = utils::hash(&password);
            let result = account.create_new_admin(
                &username, &display_name, &hashed_password, &test_connection
            );
            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn test_superuser_create_admin() {
            let test_connection = utils::get_test_connection();

            let admin_username = utils::generate_random_string(20);
            let admin_display_name = utils::generate_random_string(20);
            let admin_password = utils::generate_random_string(30);
            let admin_hashed_password = utils::hash(&admin_password);
            let _ = User::create(
                &admin_username,
                &admin_display_name,
                &admin_hashed_password,
                UserRole::Superuser,
                &test_connection
            );
            let user_admin = User::get(&admin_username, &test_connection).unwrap();
            let account = Account{user: user_admin};

            let username = utils::generate_random_string(20);
            let display_name = utils::generate_random_string(20);
            let password = utils::generate_random_string(30);
            let hashed_password = utils::hash(&password);
            let result = account.create_new_admin(
                &username, &display_name, &hashed_password, &test_connection
            );
            assert_eq!(result.is_ok(), true);
        }
    }

    mod test_update {
        use crate::account::Account;
        use crate::database_models::User;
        use crate::properties::UserRole;
        use crate::utils;

        #[test]
        fn test_update_display_name() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);
            let display_name = utils::generate_random_string(20);
            let password = utils::generate_random_string(30);
            let hashed_password = utils::hash(&password);
            let _ = User::create(
                &username,
                &display_name,
                &hashed_password,
                UserRole::Admin,
                &test_connection
            );
            let user = User::get(&username, &test_connection).unwrap();
            let mut account = Account{user};
            let updated_display_name = utils::generate_random_string(20);

            let result = account.update(
                Option::from(&updated_display_name),
                Option::None,
                &test_connection
            );
            assert_eq!(result.is_ok(), true);

            let updated_user = User::get(&username, &test_connection).unwrap();
            assert_eq!(updated_user.display_name, updated_display_name);
        }

        #[test]
        fn test_update_password() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);
            let display_name = utils::generate_random_string(20);
            let password = utils::generate_random_string(30);
            let hashed_password = utils::hash(&password);
            let _ = User::create(
                &username,
                &display_name,
                &hashed_password,
                UserRole::Admin,
                &test_connection
            );

            let user = User::get(&username, &test_connection).unwrap();
            let mut account = Account{user};
            let updated_password = utils::generate_random_string(20);
            let result = account.update(
                Option::None,
                Option::from(&updated_password),
                &test_connection
            );
            assert_eq!(result.is_ok(), true);

            let updated_user = User::get(&username, &test_connection).unwrap();
            assert_eq!(utils::verify(&updated_password, &updated_user.hashed_password), true);
        }

        #[test]
        fn test_update_both() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);
            let display_name = utils::generate_random_string(20);
            let password = utils::generate_random_string(30);
            let hashed_password = utils::hash(&password);
            let _ = User::create(
                &username,
                &display_name,
                &hashed_password,
                UserRole::Admin,
                &test_connection
            );

            let user = User::get(&username, &test_connection).unwrap();
            let mut account = Account{user};
            let updated_display_name = utils::generate_random_string(20);
            let updated_password = utils::generate_random_string(20);
            let result = account.update(
                Option::from(&updated_display_name),
                Option::from(&updated_password),
                &test_connection
            );
            assert_eq!(result.is_ok(), true);

            let updated_user = User::get(&username, &test_connection).unwrap();
            assert_eq!(updated_user.display_name, updated_display_name);
            assert_eq!(utils::verify(&updated_password, &updated_user.hashed_password), true);
        }

        #[test]
        fn test_update_none() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);
            let display_name = utils::generate_random_string(20);
            let password = utils::generate_random_string(30);
            let hashed_password = utils::hash(&password);
            let _ = User::create(
                &username,
                &display_name,
                &hashed_password,
                UserRole::Admin,
                &test_connection
            );

            let user = User::get(&username, &test_connection).unwrap();
            let mut account = Account{user};
            let result = account.update(
                Option::None,
                Option::None,
                &test_connection
            );
            assert_eq!(result.is_ok(), true);

            let updated_user = User::get(&username, &test_connection).unwrap();
            assert_eq!(updated_user.display_name, display_name);
            assert_eq!(utils::verify(&password, &updated_user.hashed_password), true);
        }
    }
}
