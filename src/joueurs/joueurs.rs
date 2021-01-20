use std::env;

use crate::errors::ErrorType;
use crate::utils::http_get_text;


pub struct Joueurs {}

impl Joueurs {
    pub fn get(no_of_try: i32) -> Result<String, ErrorType> {
        if no_of_try <= 0 {
            return Err(ErrorType::ExternalConnectionError(
                String::from("Failed getting joueurs from WOF website.")
            ));
        }

        let url = env::var("JOUEURS_URL").unwrap();
        match http_get_text(&url) {
            Ok(joueurs) => {
                info!("Joueurs successfully obtained");
                Ok(joueurs)
            }
            Err(_) => Joueurs::get(no_of_try - 1)
        }
    }
}
