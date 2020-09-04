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
        country_joueurs.iter().for_each(|country_joueur| {
            let rows: Vec<&str> = country_joueur.splitn(2, '\n').collect();
            if rows.len() < 2 {
                return;
            }

            let country = String::from(rows[0].trim());
            let joueurs = String::from(rows[1]);
            let mut player_parser = PlayerParser::create();

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
