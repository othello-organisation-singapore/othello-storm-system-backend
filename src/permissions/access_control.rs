use std::env;
use jsonwebtoken::{encode, decode, Header, Validation};

use super::Claims;
use super::super::models::User;
use super::super::utils;

pub struct AccessControl {
    user: User
}

impl AccessControl {
    pub fn from_user(user: User) -> AccessControl {
        AccessControl {
            user
        }
    }

    pub fn get_visitor_access() -> AccessControl {
        let user = User::get_dummy_visitor();
        AccessControl::from_user(user)
    }

    pub fn from_jwt(jwt: String, services: &utils::ExternalServices) -> AccessControl {
        let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let validation = Validation { iss: Some(AccessControl::get_issuer()), ..Validation::default()};
        let token_data = match decode::<Claims>(&jwt, secret_key.as_ref(), &validation) {
            Ok(t) => t,
            Err(_) => return AccessControl::get_visitor_access()
        };

        if let Ok(user) = User::get(&token_data.claims.username, services) {
            return AccessControl::from_user(user)
        }
        AccessControl::get_visitor_access()
    }

    fn get_issuer() -> String {
        String::from("Othello Storm System")
    }
}

impl AccessControl {
    pub fn generate_jwt(&self) -> Result<String, String> {
        let claims = self.generate_claims();
        let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        match encode(&Header::default(), &claims, secret_key.as_ref()) {
            Ok(token) => Ok(token),
            Err(_) => Err(String::from("Failed to generate token"))
        }
    }

    fn generate_claims(&self) -> Claims {
        Claims {
            exp: 60 * 60 * 30 + utils::get_current_timestamp(),
            iss: AccessControl::get_issuer(),
            username: self.user.username.clone()
        }
    }
}
