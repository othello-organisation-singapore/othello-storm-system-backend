use crate::regex::Regex;
use crate::tournament_manager::Player;

use super::joueurs_name_parser::NameParser;

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
            let newline = Regex::new(r"\n").unwrap();
            let joueurs_vec: Vec<String> = newline.split(&joueurs).map(|x| String::from(x)).filter(|x| !x.is_empty()).collect();
            for player in joueurs_vec {
                println!("{}", player);

                let mut id = String::new();
                let mut name = String::new();
                let mut rating = String::new();
                let player = String::from(player.trim_start());
                let mut curr = 0;
                for char in player.chars() {
                    match curr {
                        0 => {
                            if char == ' ' {
                                curr += 1;
                                continue;
                            }
                            id.push(char);
                        }
                        1 => {
                            if char == '%' || char == '_' {
                                continue;
                            }
                            if char == '<' {
                                curr += 1;
                                continue;
                            }
                            name.push(char);
                        }
                        2 => {
                            if char == '>' {
                                break;
                            }
                            rating.push(char);
                        }
                        _ => break,
                    }
                }
                if rating.is_empty() {
                    rating = String::from("1200");
                }
                rating.parse::<i32>().unwrap();
                let name = String::from(name.trim());
                let mut name_parser = NameParser::create();
                let parsed_name = name_parser.parse(&name);
                let first_name = parsed_name.first_name;
                let last_name = parsed_name.last_name;
                println!("id:{}, first_name:{}, last_name:{}, rating:{}, country:{}", id, first_name, last_name, rating, country);
            }
        }
        Vec::new()
    }
}
