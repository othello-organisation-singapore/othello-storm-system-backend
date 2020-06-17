use std::env;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Validation};
use jsonwebtoken::errors::{ErrorKind};

use super::get_current_timestamp;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: u64,
    pub iss: String,
    pub username: String
}

pub struct JWTMediator {}

impl JWTMediator {

    pub fn generate_jwt_from_username(username: &String) -> Result<String, String> {
        let claims = JWTMediator::generate_claims(username);
        let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        match encode(&Header::default(), &claims, secret_key.as_ref()) {
            Ok(token) => Ok(token),
            Err(_) => Err(String::from("Failed to generate token"))
        }
    }

    fn generate_claims(username: &String) -> Claims {
        Claims {
            exp: 60 * 60 * 30 + get_current_timestamp(),
            iss: JWTMediator::get_issuer(),
            username: username.clone()
        }
    }

    fn get_issuer() -> String {
        String::from("Othello Storm System")
    }

    pub fn get_username_from_jwt(jwt: &String) -> Result<String, String> {
        let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let validation = Validation { iss: Some(JWTMediator::get_issuer()), ..Validation::default()};
        let token_data = match decode::<Claims>(jwt, secret_key.as_ref(), &validation) {
            Ok(t) => t,
            Err(err) => return match *err.kind() {
                ErrorKind::ExpiredSignature => Err(String::from("Token expired")),
                _ => Err(String::from("Something is wrong"))
            }
        };
        Ok(token_data.claims.username)
    }
}

#[cfg(test)]
mod tests {
    mod test_get_generate_jwt {
        use crate::utils::JWTMediator;
        use crate::utils;
        use mocktopus::mocking::{Mockable, MockResult};

        #[test]
        fn test_generate_jwt() {
            let username = utils::generate_random_string(10);
            let result = JWTMediator::generate_jwt_from_username(&username);
            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn test_generate_get_jwt() {
            let username = utils::generate_random_string(10);
            let jwt = JWTMediator::generate_jwt_from_username(&username).unwrap();
            let username_claimed = JWTMediator::get_username_from_jwt(&jwt).unwrap();
            assert_eq!(username, username_claimed);
        }

        #[test]
        fn test_generate_get_jwt_error() {
            let jwt = utils::generate_random_string(60);
            let result = JWTMediator::get_username_from_jwt(&jwt);
            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn test_expired_jwt() {
            utils::get_current_timestamp.mock_safe(|| MockResult::Return(0));
            let username = utils::generate_random_string(10);
            let jwt = JWTMediator::generate_jwt_from_username(&username).unwrap();
            let result = JWTMediator::get_username_from_jwt(&jwt);
            assert_eq!(result.is_err(), true);
        }
    }
}