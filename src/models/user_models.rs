use diesel::prelude::*;
use pwhash::bcrypt;

use super::super::schema::users;
use super::super::properties::UserRole;
use super::super::utils;

#[derive(PartialEq, Debug, Queryable)]
pub struct User {
    id: i32,
    pub username: String,
    pub display_name: String,
    pub hashed_password: String,
    role: String,
}

#[derive(Insertable)]
#[table_name = "users"]
struct NewUser<'a> {
    pub username: &'a str,
    pub display_name: &'a str,
    pub hashed_password: &'a str,
    pub role: &'a str,
}

impl User {
    pub fn get(username: &String, connection: &PgConnection) -> Result<User, String> {
        let mut filtered_users = users::table
            .filter(users::username.eq(&username))
            .load::<User>(connection)
            .expect("Error connecting to database");

        if let Some(user) = filtered_users.pop() {
            return Ok(user);
        }
        Err(String::from("User not found."))
    }

    pub fn get_role(&self) -> UserRole {
        return UserRole::from_string(self.role.clone());
    }

    pub fn get_dummy_visitor() -> User {
        User {
            id: utils::generate_random_number(),
            username: utils::generate_random_string(30),
            display_name: utils::generate_random_string(30),
            hashed_password: utils::generate_random_string(30),
            role: String::from("visitor")
        }
    }
}

impl User {
    pub fn create_new_superuser(username: String, display_name: String, password: String, connection: &PgConnection)
                                -> Result<User, String> {
        User::create_new_user(username, display_name, password, UserRole::Superuser, connection)
    }

    pub fn create_new_admin(username: String, display_name: String, password: String, connection: &PgConnection)
                            -> Result<User, String> {
        User::create_new_user(username, display_name, password, UserRole::Admin, connection)
    }

    fn create_new_user(username: String, display_name: String, password: String, role: UserRole, connection: &PgConnection)
                       -> Result<User, String> {
        if User::is_username_exists(&username, connection) {
            return Err(String::from("Username exists."));
        }
        let new_user = NewUser {
            username: &username,
            display_name: &display_name,
            hashed_password: &User::hash_password(password),
            role: &role.to_string(),
        };
        User::insert_to_database(new_user, connection);
        User::get(&username, connection)
    }

    fn is_username_exists(username: &String, connection: &PgConnection) -> bool {
        if let Ok(_) = User::get(username, connection) {
            return true;
        }
        false
    }

    fn hash_password(password: String) -> String {
        bcrypt::hash(password).unwrap()
    }

    fn insert_to_database(new_user: NewUser, connection: &PgConnection) {
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(connection)
            .expect("Cannot create new user.");
    }

    pub fn update_password(username: String, updated_password: String, connection: &PgConnection)
                  -> Result<User, String> {
        if !User::is_username_exists(&username, connection) {
            return Err(String::from("Username not exists"));
        }

        let hashed_password = User::hash_password(updated_password);
        diesel::update(users::table.filter(users::username.eq(&username)))
            .set((users::hashed_password.eq(&hashed_password), ))
            .get_result::<User>(connection)
            .expect("Failed to update.");
        User::get(&username, connection)
    }
}

impl User {
    pub fn login(username: String, password: String, connection: &PgConnection) -> Result<User, String> {
        let user = User::get(&username, connection)?;
        if user.is_password_correct(password) {
            return Ok(user);
        }
        Err(String::from("Password mismatch."))
    }

    fn is_password_correct(&self, password: String) -> bool {
        bcrypt::verify(password, &self.hashed_password)
    }

    pub fn get_all_admin_or_higher(connection: &PgConnection) -> Vec<User> {
        let users = users::table
            .filter(users::role.eq_any(vec![UserRole::Admin.to_string(), UserRole::Superuser.to_string()]))
            .load::<User>(connection)
            .expect("Error connecting to database");
        users
    }
}

#[cfg(test)]
mod tests {
    mod test_user_creation {
        use crate::models::User;
        use crate::properties::UserRole;
        use crate::utils;
        use crate::utils::ExternalServices;

        #[test]
        fn test_create_new_superuser() {
            let test_services = ExternalServices::create_test_services();
            let connection = test_services.get_connection();

            let random_username = utils::generate_random_string(30);
            let random_display_name = utils::generate_random_string(30);
            let user = User::create_new_superuser(
                random_username.clone(),
                random_display_name.clone(),
                utils::generate_random_string(30),
                &connection,
            ).unwrap();

            assert_eq!(user.username, random_username);
            assert_eq!(user.display_name, random_display_name);
            assert_eq!(user.get_role(), UserRole::Superuser);
        }

        #[test]
        fn test_create_new_admin() {
            let test_services = ExternalServices::create_test_services();
            let connection = test_services.get_connection();

            let random_username = utils::generate_random_string(30);
            let random_display_name = utils::generate_random_string(30);
            let user = User::create_new_admin(
                random_username.clone(),
                random_display_name.clone(),
                utils::generate_random_string(30),
                &connection,
            ).unwrap();

            assert_eq!(user.username, random_username);
            assert_eq!(user.display_name, random_display_name);
            assert_eq!(user.get_role(), UserRole::Admin);
        }

        #[test]
        #[should_panic]
        fn test_create_used_username() {
            let test_services = ExternalServices::create_test_services();
            let connection = test_services.get_connection();

            let user = User::create_new_admin(
                String::from("test_username"),
                String::from("Test Name"),
                String::from("test_password"),
                &connection,
            ).unwrap();
            let _user_with_same_username = User::create_new_superuser(
                user.username.clone(),
                String::from("Another Name"),
                String::from("another_password"),
                &connection,
            ).unwrap();
        }
    }

    mod test_user_update_password {
        use crate::models::User;
        use crate::utils;
        use crate::utils::ExternalServices;
        use pwhash::bcrypt;

        #[test]
        fn test_update_password() {
            let test_services = ExternalServices::create_test_services();
            let connection = test_services.get_connection();

            let updated_password = utils::generate_random_string(30);
            let user = User::create_new_admin(
                utils::generate_random_string(30),
                utils::generate_random_string(30),
                utils::generate_random_string(30),
                &connection,
            ).unwrap();
            let updated_user =User::update_password(
                user.username.clone(),
                updated_password.clone(),
                &connection
            ).unwrap();
            assert_eq!(bcrypt::verify(updated_password, &updated_user.hashed_password), true);
        }

        #[test]
        #[should_panic]
        fn test_update_password_not_found_username() {
            let test_services = ExternalServices::create_test_services();
            let connection = test_services.get_connection();

            User::update_password(
                utils::generate_random_string(30),
                utils::generate_random_string(30),
                &connection
            ).unwrap();
        }
    }

    mod test_user_login {
        use crate::models::User;
        use crate::utils;
        use crate::utils::ExternalServices;

        #[test]
        fn test_login_success() {
            let test_services = ExternalServices::create_test_services();
            let connection = test_services.get_connection();

            let username = utils::generate_random_string(30);
            let password = utils::generate_random_string(30);
            let created_user = User::create_new_admin(
                username.clone(),
                utils::generate_random_string(30),
                password.clone(),
                &connection,
            ).unwrap();
            let logged_in_user = User::login(username.clone(), password.clone(), &connection).unwrap();
            assert_eq!(created_user, logged_in_user);
        }

        #[test]
        #[should_panic]
        fn test_login_failed() {
            let test_services = ExternalServices::create_test_services();
            let connection = test_services.get_connection();

            let username = utils::generate_random_string(30);
            let password = utils::generate_random_string(30);
            User::create_new_admin(
                username.clone(),
                utils::generate_random_string(30),
                password.clone(),
                &connection,
            ).unwrap();
            User::login(
                username.clone(),
                utils::generate_random_string(30),
                &connection
            ).unwrap();
        }

        #[test]
        #[should_panic]
        fn test_login_no_user() {
            let test_services = ExternalServices::create_test_services();
            let connection = test_services.get_connection();

            let username = utils::generate_random_string(30);
            User::login(
                username,
                utils::generate_random_string(30),
                &connection
            ).unwrap();
        }
    }

    mod test_get_all_admin_or_higher {
        use crate::models::User;
        use crate::utils;
        use crate::utils::ExternalServices;

        #[test]
        fn test_zero_user() {
            let test_services = ExternalServices::create_test_services();
            let connection = test_services.get_connection();

            let users = User::get_all_admin_or_higher(&connection);
            assert_eq!(users.len(), 0);
        }

        #[test]
        fn test_get_all_admin_and_superuser() {
            let test_services = ExternalServices::create_test_services();
            let connection = test_services.get_connection();

            User::create_new_superuser(
                utils::generate_random_string(30),
                utils::generate_random_string(30),
                utils::generate_random_string(30),
                &connection,
            ).unwrap();
            User::create_new_admin(
                utils::generate_random_string(30),
                utils::generate_random_string(30),
                utils::generate_random_string(30),
                &connection,
            ).unwrap();
            User::create_new_admin(
                utils::generate_random_string(30),
                utils::generate_random_string(30),
                utils::generate_random_string(30),
                &connection,
            ).unwrap();

            let users = User::get_all_admin_or_higher(&connection);
            assert_eq!(users.len(), 3);
        }
    }
}
