use std::env;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Validation};
use jsonwebtoken::errors::{ErrorKind};

use super::super::models::User;
use super::super::utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: u64,
    pub iss: String,
    pub username: String
}

pub struct JWTMediator {}

impl JWTMediator {

    pub fn generate_jwt_from_user(user: &User) -> Result<String, String> {
        let claims = JWTMediator::generate_claims(&user.username);
        let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        match encode(&Header::default(), &claims, secret_key.as_ref()) {
            Ok(token) => Ok(token),
            Err(_) => Err(String::from("Failed to generate token"))
        }
    }

    fn generate_claims(username: &String) -> Claims {
        Claims {
            exp: 60 * 60 * 30 + utils::get_current_timestamp(),
            iss: JWTMediator::get_issuer(),
            username: username.clone()
        }
    }

    fn get_issuer() -> String {
        String::from("Othello Storm System")
    }

    pub fn get_user_from_jwt(jwt: String, services: &utils::ExternalServices) -> Result<User, String> {
        let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let validation = Validation { iss: Some(JWTMediator::get_issuer()), ..Validation::default()};
        let token_data = match decode::<Claims>(&jwt, secret_key.as_ref(), &validation) {
            Ok(t) => t,
            Err(err) => return match *err.kind() {
                ErrorKind::ExpiredSignature => Err(String::from("Token expired")),
                _ => Err(String::from("Something is wrong"))
            }
        };
        if let Ok(user) = User::get(&token_data.claims.username, services) {
            return Ok(user)
        }
        Err(String::from("User not found"))
    }
}
