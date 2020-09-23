use crate::tournament_manager::Player;

use super::joueurs_player_parser::PlayerParser;

pub struct JoueursParser {}

impl JoueursParser {
    pub fn parse(joueurs: &String) -> Result<Vec<Player>, String> {
        let mut country_joueurs: Vec<String> = joueurs
            .split("pays = ")
            .map(|x| String::from(x))
            .collect();

        if country_joueurs.is_empty() {
            return Err(String::from("Joueurs file is empty, please contact the administrator."));
        }
        country_joueurs.remove(0);  // remove title

        let mut players: Vec<Player> = Vec::new();
        let mut player_parser = PlayerParser::create();
        country_joueurs.iter().for_each(|country_joueur| {
            let rows: Vec<&str> = country_joueur.splitn(2, '\n').collect();
            if rows.len() < 2 {
                return;
            }

            let country = String::from(rows[0].trim());
            let joueurs = String::from(rows[1]);
            joueurs
                .split('\n')
                .map(|x| String::from(x.trim_start()))
                .filter(|x| !x.is_empty())
                .for_each(|player| players.push(player_parser.parse(&player, &country)))
        });
        info!("Joueurs successfully parsed.");
        Ok(players)
    }
}


#[cfg(test)]
mod tests {
    mod joueurs_parser {
        use crate::joueurs::JoueursParser;

        #[test]
        fn test_parse_success() {
            let joueurs = String::from("% Liste des joueurs par pays\n\npays = ARG\n\n280016 ACUNA, Ricardo                       %_<1484>\n280045 ALOATTI, Matias                      %_<1072>\n280028 ANANOS, Sergio                      \n280009 BILINKIS, Mariano                   \n280046 BLATMAN, Ariel                       %_<482>");
            let result = JoueursParser::parse(&joueurs);
            assert_eq!(result.is_ok(), true);

            let joueurs_content = result.unwrap();
            assert_eq!(joueurs_content.len(), 5);
        }

        #[test]
        fn test_parse_failed_empty() {
            let joueurs = String::from("");
            let joueurs_content = JoueursParser::parse(&joueurs).unwrap();
            assert_eq!(joueurs_content.len(), 0);
        }

        #[test]
        fn test_parse_failed_wrong_format() {
            let joueurs = String::from("Err");
            let joueurs_content = JoueursParser::parse(&joueurs).unwrap();
            assert_eq!(joueurs_content.len(), 0);
        }
    }
}
