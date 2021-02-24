use itertools::Itertools;
use serde_json::{Map, Value};

use crate::database_models::PlayerRowModel;
use crate::game_match::{GameMatchCreator, IGameMatch};
use crate::tournament_manager::IResultKeeper;

use super::PairingGenerator;

struct SwissPairPairingsGenerator {}

impl SwissPairPairingsGenerator {
    fn generate_first_round_pairings(
        &self,
        round_id: &i32,
        players: &Vec<PlayerRowModel>,
    ) -> Vec<Box<dyn IGameMatch>> {
        let mut matches = Vec::new();
        let sorted_players =  players
            .into_iter()
            .sorted_by_key(|player| -player.rating)
            .collect::<Vec<&PlayerRowModel>>();
        for pair in sorted_players.chunks(2) {
            if pair.len() == 1 {
                let player = pair.first().unwrap();
                matches.push(Box::from(GameMatchCreator::create_new_bye_match(
                    round_id,
                    &player.id,
                    &Value::from(Map::new()),
                )));
                continue;
            }
            let black_player = pair.first().unwrap();
            let white_player = pair.last().unwrap();
            matches.push(Box::from(GameMatchCreator::create_new_match(
                round_id,
                &black_player.id,
                &white_player.id,
                &Value::from(Map::new()),
            )));
        }
        matches
    }
}

impl PairingGenerator for SwissPairPairingsGenerator {
    fn generate_pairings(
        &self,
        round_id: &i32,
        players: &Vec<PlayerRowModel>,
        past_results: &Box<dyn IResultKeeper>,
    ) -> Vec<Box<dyn IGameMatch>> {
        if past_results.is_empty() {
            return self.generate_first_round_pairings(round_id, players);
        }
        unimplemented!()
    }
}
