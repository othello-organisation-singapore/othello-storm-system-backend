use crate::utils::http_get_text;
use crate::regex::Regex;
use super::Player;

pub fn get_joueurs_data(no_of_try: i32) -> Result<String, String> {
    if no_of_try <= 0 {
        return Err(String::from("Failed getting joueurs from WOF website."));
    }

    let url = String::from("https://www.worldothello.org/files/joueurs.txt");
    match http_get_text(&url) {
        Ok(joueurs) => {
            info!("Joueurs successfully obtained");
            Ok(joueurs)
        }
        Err(_) => get_joueurs_data(no_of_try - 1)
    }
}


pub struct JoueursParser {}

impl JoueursParser {
    pub fn parse(joueurs: &String) -> Vec<Player> {
        let re = Regex::new(r"pays = ").unwrap();
        let mut country_joueurs: Vec<String> = re.split(joueurs).map(|x | String::from(x)).collect();
//        println!("{}", country_joueurs[0]);
//        println!("===========================");
//        println!("{}", country_joueurs[1]);
//        println!("===========================");
//        println!("{}", country_joueurs[2]);
//        println!("===========================");
        let re_country_joueurs = Regex::new(r"(.+)\n\n([\S\s]+)\n$").unwrap();
        let mut first = true;
        for country_joueur in country_joueurs.iter() {
            if first {
                first = false;
                continue;
            }
            let parsed_country_joueurs = re_country_joueurs.captures(country_joueur).unwrap();
            let country = String::from(parsed_country_joueurs[1].trim());
            let joueurs = String::from(&parsed_country_joueurs[2]);
//            println!("===========================");
//            println!("country = {}", &country);
//            println!("===========================");
//            println!("joueurs = {}", joueurs);
//            println!("===========================");
            let newline = Regex::new(r"\n").unwrap();
            let mut joueurs_vec: Vec<String> = newline.split(&joueurs).map(|x | String::from(x)).filter(|x| !x.is_empty()).collect();
            for player in joueurs_vec {
//                println!("{}", player);
                let re_player = Regex::new(r"(\w+) (.+) (.+)").unwrap();
                let parsed_player = re_player.captures(&player).unwrap();

                let id = String::from(&parsed_player[1]);
                let name = String::from(&parsed_player[2]);
                let re_name = match name.find(",") {
                    Some(_) => Regex::new(r"(.+), (.+)"),
                    None => Regex::new(r"(.+) \((.*)\)"),
                }.unwrap();
                let parsed_name = re_name.captures(&name).unwrap();
                let first_name = String::from(parsed_name[2].trim());
                let last_name = String::from(&parsed_name[1]);

                let rating = String::from(&parsed_player[3]);
                let re_rating = Regex::new("%_<(.+)>").unwrap();
                let rating_score = match re_rating.captures(&rating) {
                    Some(parsed_rating) => String::from(&parsed_rating[1]),
                    None => String::from("1200")
                }.parse::<i32>().unwrap();
//                println!("id:{}, first_name:{}, last_name:{}, rating:{}, country:{}", id, first_name, last_name, rating_score, country);
            }
        }
        Vec::new()
    }
}