use crate::regex::Regex;
use crate::tournament_manager::Player;

use super::joueurs_player_parser::PlayerParser;

pub struct JoueursParser {}

impl JoueursParser {
    pub fn parse(joueurs: &String) -> Vec<Player> {
        let re = Regex::new(r"pays = ").unwrap();
        let country_joueurs: Vec<String> = re.split(joueurs).map(|x| String::from(x)).collect();
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
            let joueurs_vec: Vec<String> = joueurs.split('\n').map(|x| String::from(x)).collect();
            for player in joueurs_vec {
                println!("{}", player);

                let player = String::from(player.trim_start());
                let mut player_parser = PlayerParser::create();
                let parsed_player = player_parser.parse(&player, &country);
                println!("id:{}, first_name:{}, last_name:{}, rating:{}, country:{}", parsed_player.joueurs_id, parsed_player.first_name, parsed_player.last_name, parsed_player.rating, parsed_player.country);
            }
        }
        Vec::new()
    }
}
