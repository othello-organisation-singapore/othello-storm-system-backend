use diesel::prelude::*;

use super::super::schema::users;
use super::super::properties::UserRole;

#[derive(AsChangeset, PartialEq, Debug, Queryable)]
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
    pub fn create(username: &String, display_name: &String, hashed_password: &String, role: UserRole,
                  connection: &PgConnection) -> Result<(), String> {
        if User::is_username_exists(&username, connection) {
            return Err(String::from("Username exists."));
        }
        let new_user = NewUser {
            username,
            display_name,
            hashed_password,
            role: &role.to_string(),
        };
        User::insert_to_database(new_user, connection);
        Ok(())
    }

    fn is_username_exists(username: &String, connection: &PgConnection) -> bool {
        if let Ok(_) = User::get(username, connection) {
            return true;
        }
        false
    }

    fn insert_to_database(new_user: NewUser, connection: &PgConnection) {
        diesel::insert_into(users::table)
            .values(new_user)
            .execute(connection)
            .expect("Cannot create new user.");
    }

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

    pub fn update(&self, connection: &PgConnection) {
        diesel::update(users::table.find(self.id))
            .set(self)
            .execute(connection)
            .expect("Failed to update.");
    }
}


#[cfg(test)]
mod tests {

    mod test_user_creation {
        use crate::database_models::User;
        use crate::properties::UserRole;
        use crate::utils;

        #[test]
        fn test_create_new_user() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);
            let display_name = utils::generate_random_string(20);
            let password = utils::generate_random_string(30);
            let hashed_password = utils::hash(&password);

            let result = User::create(&username, &display_name, &hashed_password, UserRole::Superuser, &test_connection);
            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn test_create_user_with_duplicate_username() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);
            let display_name = utils::generate_random_string(20);
            let password = utils::generate_random_string(30);
            let hashed_password = utils::hash(&password);

            let second_display_name = utils::generate_random_string(20);
            let second_password = utils::generate_random_string(30);
            let second_hashed_password = utils::hash(&second_password);

            let _ = User::create(&username, &display_name, &hashed_password, UserRole::Superuser, &test_connection);
            let result = User::create(&username, &second_display_name, &second_hashed_password, UserRole::Admin, &test_connection);
            assert_eq!(result.is_err(), true);
        }
    }

    mod test_user_retrieval {
        use crate::database_models::User;
        use crate::properties::UserRole;
        use crate::utils;

        #[test]
        fn test_get_user() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);
            let display_name = utils::generate_random_string(20);
            let password = utils::generate_random_string(30);
            let hashed_password = utils::hash(&password);

            let _ = User::create(&username, &display_name, &hashed_password, UserRole::Superuser, &test_connection);

            let user = User::get(&username, &test_connection).unwrap();
            assert_eq!(user.username, username);
            assert_eq!(user.display_name, display_name);
            assert_eq!(user.hashed_password, hashed_password);
        }

        #[test]
        fn test_get_user_not_found() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);

            let result = User::get(&username, &test_connection);
            assert_eq!(result.is_err(), true);
        }
    }

    mod test_user_update {
        use crate::database_models::User;
        use crate::properties::UserRole;
        use crate::utils;

        #[test]
        fn test_update_user() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);
            let display_name = utils::generate_random_string(20);
            let password = utils::generate_random_string(30);
            let hashed_password = utils::hash(&password);

            let _ = User::create(&username, &display_name, &hashed_password, UserRole::Superuser, &test_connection);

            let mut user = User::get(&username, &test_connection).unwrap();

            let updated_display_name = utils::generate_random_string(20);
            user.display_name = updated_display_name.clone();
            user.update(&test_connection);

            let updated_user = User::get(&username, &test_connection).unwrap();
            assert_ne!(updated_user.display_name, display_name);
            assert_eq!(updated_user.display_name, updated_display_name);
            assert_eq!(updated_user.username, username);
            assert_eq!(updated_user.hashed_password, hashed_password);
        }
    }
}
