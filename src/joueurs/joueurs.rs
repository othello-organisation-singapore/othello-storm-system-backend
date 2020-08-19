use crate::utils::http_get_text;


pub struct Joueurs {}

impl Joueurs {
    pub fn get(no_of_try: i32) -> Result<String, String> {
        if no_of_try <= 0 {
            return Err(String::from("Failed getting joueurs from WOF website."));
        }

        let url = String::from("https://www.worldothello.org/files/joueurs.txt");
        match http_get_text(&url) {
            Ok(joueurs) => {
                info!("Joueurs successfully obtained");
                Ok(joueurs)
            }
            Err(_) => Joueurs::get(no_of_try - 1)
        }
    }
}
