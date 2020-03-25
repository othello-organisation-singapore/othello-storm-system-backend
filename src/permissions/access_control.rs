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
