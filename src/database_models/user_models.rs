use diesel::prelude::*;

use crate::errors::ErrorType;
use crate::properties::UserRole;
use crate::schema::users;

#[derive(AsChangeset, PartialEq, Debug, Queryable, Identifiable)]
#[table_name = "users"]
#[primary_key(username)]
pub struct UserRowModel {
    pub username: String,
    pub display_name: String,
    pub hashed_password: String,
    pub role: String,
}

#[derive(Insertable)]
#[table_name = "users"]
struct NewUserRowModel<'a> {
    pub username: &'a String,
    pub display_name: &'a String,
    pub hashed_password: &'a String,
    pub role: &'a String,
}

impl UserRowModel {
    pub fn create(
        username: &String,
        display_name: &String,
        hashed_password: &String,
        role: UserRole,
        connection: &PgConnection,
    ) -> Result<UserRowModel, ErrorType> {
        if UserRowModel::is_username_exists(&username, connection) {
            return Err(ErrorType::BadRequestError(String::from("Username exists.")));
        }
        let new_user = NewUserRowModel {
            username,
            display_name,
            hashed_password,
            role: &role.to_string(),
        };
        UserRowModel::insert_to_database(new_user, connection)
    }

    fn is_username_exists(username: &String, connection: &PgConnection) -> bool {
        if let Ok(_) = UserRowModel::get(username, connection) {
            return true;
        }
        false
    }

    fn insert_to_database(
        new_user: NewUserRowModel,
        connection: &PgConnection,
    ) -> Result<UserRowModel, ErrorType> {
        let username = new_user.username.clone();
        let display_name = new_user.display_name.clone();
        let role = new_user.role.clone();

        let result = diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<UserRowModel>(connection);
        match result {
            Ok(user) => {
                info!(
                    "User {} ({}) is created as {}.",
                    username, display_name, role
                );
                Ok(user)
            }
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }

    pub fn get(username: &String, connection: &PgConnection) -> Result<UserRowModel, ErrorType> {
        let result = users::table.find(username).first(connection);

        match result {
            Ok(user) => Ok(user),
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::BadRequestError(String::from("User not found")))
            }
        }
    }

    pub fn get_role(&self) -> UserRole {
        UserRole::from_string(self.role.clone())
    }

    pub fn update(&self, connection: &PgConnection) -> Result<UserRowModel, ErrorType> {
        let result = diesel::update(self)
            .set(self)
            .get_result::<UserRowModel>(connection);
        match result {
            Ok(user) => {
                info!(
                    "User {} ({}) is updated.",
                    &self.username, &self.display_name
                );
                Ok(user)
            }
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod test_user_creation {
        use crate::database_models::UserRowModel;
        use crate::properties::UserRole;
        use crate::utils;

        #[test]
        fn test_create_new_user() {
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

            let _ = UserRowModel::create(
                &username,
                &display_name,
                &hashed_password,
                UserRole::Superuser,
                &test_connection,
            );
            let result = UserRowModel::create(
                &username,
                &second_display_name,
                &second_hashed_password,
                UserRole::Admin,
                &test_connection,
            );
            assert_eq!(result.is_err(), true);
        }
    }

    mod test_user_retrieval {
        use crate::database_models::UserRowModel;
        use crate::properties::UserRole;
        use crate::utils;

        #[test]
        fn test_get_user() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);
            let display_name = utils::generate_random_string(20);
            let password = utils::generate_random_string(30);
            let hashed_password = utils::hash(&password);

            let _ = UserRowModel::create(
                &username,
                &display_name,
                &hashed_password,
                UserRole::Superuser,
                &test_connection,
            );

            let user = UserRowModel::get(&username, &test_connection).unwrap();
            assert_eq!(user.username, username);
            assert_eq!(user.display_name, display_name);
            assert_eq!(user.hashed_password, hashed_password);
        }

        #[test]
        fn test_get_user_not_found() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);

            let result = UserRowModel::get(&username, &test_connection);
            assert_eq!(result.is_err(), true);
        }
    }

    mod test_user_update {
        use crate::database_models::UserRowModel;
        use crate::properties::UserRole;
        use crate::utils;

        #[test]
        fn test_update_user() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);
            let display_name = utils::generate_random_string(20);
            let password = utils::generate_random_string(30);
            let hashed_password = utils::hash(&password);

            let mut user = UserRowModel::create(
                &username,
                &display_name,
                &hashed_password,
                UserRole::Superuser,
                &test_connection,
            )
            .unwrap();

            let updated_display_name = utils::generate_random_string(20);
            user.display_name = updated_display_name.clone();
            let updated_user = user.update(&test_connection).unwrap();
            assert_ne!(updated_user.display_name, display_name);
            assert_eq!(updated_user.display_name, updated_display_name);
            assert_eq!(updated_user.username, username);
            assert_eq!(updated_user.hashed_password, hashed_password);
        }
    }
}
