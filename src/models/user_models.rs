use diesel::prelude::*;
use pwhash::bcrypt;

use super::super::schema::users;
use super::super::properties::UserRole;
use super::super::external_services::ExternalServices;

#[derive(Queryable)]
pub struct User {
    id: i32,
    pub username: String,
    pub display_name: String,
    hashed_password: String,
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
    pub fn get(username: &String, service: &ExternalServices) -> Result<User, String> {
        let connection = service.get_connection();
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
}

impl User {
    pub fn create_new_superuser(username: String, display_name: String, password: String, service: &ExternalServices)
                                -> Result<User, String> {
        User::create_new_user(username, display_name, password, UserRole::Superuser, service)
    }

    pub fn create_new_admin(username: String, display_name: String, password: String, service: &ExternalServices)
                            -> Result<User, String> {
        User::create_new_user(username, display_name, password, UserRole::Admin, service)
    }

    fn create_new_user(username: String, display_name: String, password: String, role: UserRole, service: &ExternalServices)
                       -> Result<User, String> {
        if User::is_username_exists(&username, service) {
            return Err(String::from("Username exists."));
        }
        let new_user = NewUser {
            username: &username,
            display_name: &display_name,
            hashed_password: &User::hash_password(password),
            role: &role.to_string(),
        };
        User::insert_to_database(new_user, service);
        User::get(&username, service)
    }

    fn is_username_exists(username: &String, service: &ExternalServices) -> bool {
        if let Ok(_) = User::get(username, service) {
            return true;
        }
        false
    }

    fn hash_password(password: String) -> String {
        bcrypt::hash(password).unwrap()
    }

    fn insert_to_database(new_user: NewUser, service: &ExternalServices) {
        let connection = service.get_connection();
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(connection)
            .expect("Cannot create new user.");
    }

    pub fn update(username: String, updated_display_name: String, updated_password: String, service: &ExternalServices)
                  -> Result<User, String> {
        if !User::is_username_exists(&username, service) {
            return Err(String::from("Username not exists"));
        }

        let hashed_password = User::hash_password(updated_password);
        let connection = service.get_connection();
        diesel::update(users::table.filter(users::username.eq(&username)))
            .set((
                users::display_name.eq(&updated_display_name),
                users::hashed_password.eq(&hashed_password),
            ))
            .get_result::<User>(connection)
            .expect("Failed to update.");
        User::get(&username, service)
    }

    pub fn login(username: String, password: String, service: &ExternalServices) -> Result<User, String> {
        let user = User::get(&username, service)?;
        if user.is_password_correct(password) {
            return Ok(user);
        }
        Err(String::from("Password mismatch."))
    }

    fn is_password_correct(&self, password: String) -> bool {
        bcrypt::verify(password, &self.hashed_password)
    }

    pub fn get_all_admin_or_higher(service: &ExternalServices) -> Result<Vec<User>, String> {
        let connection = service.get_connection();
        let users = users::table
            .filter(users::role.eq_any(vec![UserRole::Admin.to_string(), UserRole::Superuser.to_string()]))
            .load::<User>(connection)
            .expect("Error connecting to database");
        Ok(users)
    }
}

#[cfg(test)]
mod tests {
    mod test_user_creation {
        use crate::models::User;
        use crate::external_services::ExternalServices;
        use crate::properties::UserRole;

        #[test]
        fn test_create_new_superuser() {
            let test_service = ExternalServices::create_test_services();
            let user = User::create_new_superuser(
                String::from("test_username"),
                String::from("Test Name"),
                String::from("test_password"),
                &test_service,
            ).unwrap();

            assert_eq!(user.username, "test_username");
            assert_eq!(user.display_name, "Test Name");
            assert_eq!(user.get_role(), UserRole::Superuser);
        }

        #[test]
        fn test_create_new_admin() {
            let test_service = ExternalServices::create_test_services();
            let user = User::create_new_admin(
                String::from("test_username"),
                String::from("Test Name"),
                String::from("test_password"),
                &test_service,
            ).unwrap();

            assert_eq!(user.username, "test_username");
            assert_eq!(user.display_name, "Test Name");
            assert_eq!(user.get_role(), UserRole::Admin);
        }

        #[test]
        #[should_panic]
        fn test_create_used_username() {
            let test_service = ExternalServices::create_test_services();
            let user = User::create_new_admin(
                String::from("test_username"),
                String::from("Test Name"),
                String::from("test_password"),
                &test_service,
            ).unwrap();
            let _user_with_same_username = User::create_new_superuser(
                user.username.clone(),
                String::from("Another Name"),
                String::from("another_password"),
                &test_service,
            ).unwrap();
        }
    }

    mod test_user_update {
        use crate::models::User;
        use crate::external_services::ExternalServices;

        #[test]
        fn test_user_update_without_changing() {
            let test_service = ExternalServices::create_test_services();
            let _user = User::create_new_admin(
                String::from("test_username"),
                String::from("Test Name"),
                String::from("test_password"),
                &test_service,
            ).unwrap();
            let updated_user = User::update(
                _user.username.clone(),
                _user.display_name.clone(),
                String::from("test_password"),
                &test_service
            ).unwrap();
            assert_eq!(_user.username.clone(), updated_user.username.clone());
            assert_eq!(_user.display_name.clone(), updated_user.display_name.clone());
        }

        #[test]
        fn test_user_update_with_changing() {
            let test_service = ExternalServices::create_test_services();
            let user = User::create_new_admin(
                String::from("test_username"),
                String::from("Test Name"),
                String::from("test_password"),
                &test_service,
            ).unwrap();
            let updated_user = User::update(
                user.username.clone(),
                String::from("New Display Name"),
                String::from("new password"),
                &test_service
            ).unwrap();
            assert_eq!(user.username.clone(), updated_user.username.clone());
            assert_ne!(user.display_name.clone(), updated_user.display_name.clone());
        }

        #[test]
        #[should_panic]
        fn test_user_update_not_found_username() {
            let test_service = ExternalServices::create_test_services();
            let _ = User::update(
                String::from("test_username"),
                String::from("New Display Name"),
                String::from("new password"),
                &test_service
            ).unwrap();
        }
    }

    mod test_user_login {}

    mod test_get_all_user {}
}
