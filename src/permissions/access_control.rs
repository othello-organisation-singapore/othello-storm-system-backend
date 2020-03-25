use super::JWTMediator;
use super::super::models::User;
use super::super::properties::UserRole;
use diesel::PgConnection;

pub struct AccessControl {
    user: User
}

impl AccessControl {
    pub fn from_user(user: User) -> AccessControl {
        AccessControl {
            user
        }
    }

    pub fn from_jwt(jwt: String, connection: &PgConnection) -> AccessControl {
        if let Ok(user) = JWTMediator::get_user_from_jwt(jwt, connection) {
            return AccessControl::from_user(user)
        }
        AccessControl::get_visitor_access()
    }

    fn get_visitor_access() -> AccessControl {
        let user = User::get_dummy_visitor();
        AccessControl::from_user(user)
    }

    pub fn generate_jwt(&self) -> Result<String, String> {
        JWTMediator::generate_jwt_from_user(&self.user)
    }
}

impl AccessControl {
    pub fn has_access_to_user_with_username(&self, username: String) -> bool {
        if self.user.get_role() == UserRole::Superuser {
            return true
        }
        return self.user.username == username
    }

    pub fn has_superuser_access(&self) -> bool {
        return self.user.get_role() == UserRole::Superuser
    }

    pub fn has_admin_access(&self) -> bool {
        return [UserRole::Superuser, UserRole::Admin].contains(&self.user.get_role())
    }
}

#[cfg(test)]
mod tests {
    mod test_access_control {
        use crate::permissions::AccessControl;
        use crate::utils;
        use crate::utils::ExternalServices;
        use crate::models::User;

        #[test]
        fn test_access_control_creation_visitor() {
            let access_control = AccessControl::get_visitor_access();

            assert_eq!(access_control.has_superuser_access(), false);
            assert_eq!(access_control.has_admin_access(), false);
        }

        #[test]
        fn test_access_control_creation_from_admin() {
            let test_services = ExternalServices::create_test_services();
            let connection = test_services.get_connection();
            let superuser = User::create_new_superuser(
                utils::generate_random_string(30),
                utils::generate_random_string(30),
                utils::generate_random_string(30),
                &connection
            ).unwrap();
            let username = superuser.username.clone();

            let access_control = AccessControl::from_user(superuser);

            assert_eq!(access_control.has_superuser_access(), true);
            assert_eq!(access_control.has_admin_access(), true);
            assert_eq!(access_control.has_access_to_user_with_username(username), true);
            assert_eq!(access_control.has_access_to_user_with_username(String::from("other")), true);
        }

        #[test]
        fn test_access_control_creation_from_superuser() {
            let test_services = ExternalServices::create_test_services();
            let connection = test_services.get_connection();
            let admin = User::create_new_admin(
                utils::generate_random_string(30),
                utils::generate_random_string(30),
                utils::generate_random_string(30),
                &connection
            ).unwrap();
            let username = admin.username.clone();

            let access_control = AccessControl::from_user(admin);

            assert_eq!(access_control.has_superuser_access(), false);
            assert_eq!(access_control.has_admin_access(), true);
            assert_eq!(access_control.has_access_to_user_with_username(username), true);
            assert_eq!(access_control.has_access_to_user_with_username(String::from("other")), false);
        }

        #[test]
        fn test_access_control_creation_from_jwt() {
            let test_services = ExternalServices::create_test_services();
            let connection = test_services.get_connection();
            let admin = User::create_new_admin(
                utils::generate_random_string(30),
                utils::generate_random_string(30),
                utils::generate_random_string(30),
                &connection
            ).unwrap();
            let username = admin.username.clone();

            let access_control = AccessControl::from_user(admin);
            let jwt = access_control.generate_jwt().unwrap();
            let jwt_access_control = AccessControl::from_jwt(jwt, &connection);

            assert_eq!(jwt_access_control.has_superuser_access(), false);
            assert_eq!(jwt_access_control.has_admin_access(), true);
            assert_eq!(jwt_access_control.has_access_to_user_with_username(username), true);
            assert_eq!(jwt_access_control.has_access_to_user_with_username(String::from("other")), false);
        }

        #[test]
        fn test_access_control_creation_from_random_string() {
            let test_services = ExternalServices::create_test_services();
            let connection = test_services.get_connection();

            let random_string = utils::generate_random_string(100);
            let access_control = AccessControl::from_jwt(random_string, &connection);

            assert_eq!(access_control.has_superuser_access(), false);
            assert_eq!(access_control.has_admin_access(), false);
        }
    }
}
