use crate::utils::http_get_text;


pub fn get_joueurs_data(no_of_try: i32) -> Result<String, String> {
    if no_of_try <= 0 {
        return Err(String::from("Failed getting joueurs from WOF website."))
    }

    let url = String::from("https://www.worldothello.org/files/joueurs.txt");
    match http_get_text(&url) {
        Ok(joueurs) => {
            info!("Joueurs successfully obtained");
            Ok(joueurs)
        },
        Err(_) => get_joueurs_data(no_of_try - 1)
    }
}


pub struct JoueursPlayer {
    pub joueurs_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub country: String,
    pub rating: Option<i32>,
}


pub struct JoueursParser {}

impl JoueursParser {
    pub fn parse(joueurs: &String) -> Vec<JoueursPlayer> {
        Vec::new()
    }
}