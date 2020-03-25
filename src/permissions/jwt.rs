use std::env;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Validation};
use jsonwebtoken::errors::{ErrorKind};

use super::super::models::User;
use super::super::utils;
use diesel::PgConnection;

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

    pub fn get_user_from_jwt(jwt: String, connection: &PgConnection) -> Result<User, String> {
        let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let validation = Validation { iss: Some(JWTMediator::get_issuer()), ..Validation::default()};
        let token_data = match decode::<Claims>(&jwt, secret_key.as_ref(), &validation) {
            Ok(t) => t,
            Err(err) => return match *err.kind() {
                ErrorKind::ExpiredSignature => Err(String::from("Token expired")),
                _ => Err(String::from("Something is wrong"))
            }
        };
        if let Ok(user) = User::get(&token_data.claims.username, connection) {
            return Ok(user)
        }
        Err(String::from("User not found"))
    }
}

#[cfg(test)]
mod tests {
    mod test_get_generate_jwt {
        use crate::models::User;
        use crate::permissions::JWTMediator;
        use crate::utils;

        #[test]
        fn test_generate_jwt() {
            let user = User::get_dummy_visitor();
            let _ = JWTMediator::generate_jwt_from_user(&user).unwrap();
        }

        #[test]
        fn test_generate_get_jwt() {
            let test_services = utils::ExternalServices::create_test_services();
            let connection = test_services.get_connection();

            let user = User::create_new_admin(
                utils::generate_random_string(30),
                utils::generate_random_string(30),
                utils::generate_random_string(30),
                &connection
            ).unwrap();
            let jwt = JWTMediator::generate_jwt_from_user(&user).unwrap();
            let username_claimed = JWTMediator::get_user_from_jwt(jwt, &connection).unwrap().username;
            assert_eq!(user.username, username_claimed);
        }

        #[test]
        #[should_panic]
        fn test_generate_get_jwt_error() {
            let test_services = utils::ExternalServices::create_test_services();
            let connection = test_services.get_connection();

            let jwt = utils::generate_random_string(60);
            let _ = JWTMediator::get_user_from_jwt(jwt, &connection).unwrap().username;
        }

        #[test]
        #[should_panic]
        fn test_generate_get_jwt_no_user_found() {
            let test_services = utils::ExternalServices::create_test_services();
            let connection = test_services.get_connection();

            let user = User::get_dummy_visitor();
            let jwt = JWTMediator::generate_jwt_from_user(&user).unwrap();
            let _ = JWTMediator::get_user_from_jwt(jwt, &connection).unwrap().username;
        }
    }
}
