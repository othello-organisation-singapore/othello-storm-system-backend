use itertools::Itertools;
use serde_json::{Map, Value};
use std::collections::HashMap;

use crate::database_models::PlayerRowModel;
use crate::game_match::{GameMatchCreator, IGameMatch};
use crate::tournament_manager::{IResultKeeper, PlayerStanding};

use super::PairingGenerator;
use crate::errors::ErrorType;
use crate::properties::PlayerColor;

pub struct SwissPairingsGenerator {}

impl SwissPairingsGenerator {
    fn generate_first_round_pairings(
        &self,
        round_id: &i32,
        players: &Vec<PlayerRowModel>,
    ) -> Vec<Box<dyn IGameMatch>> {
        let mut matches = Vec::new();
        let sorted_players = players
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

    fn generate_normal_pairings(
        &self,
        round_id: &i32,
        past_results: &Box<dyn IResultKeeper>,
    ) -> Result<Vec<Box<dyn IGameMatch>>, ErrorType> {
        let mut memo: HashMap<i128, Option<Vec<Box<dyn IGameMatch>>>> = HashMap::new();
        match self.generate_remaining_pairings(
            round_id,
            &(0 as i128),
            &past_results.get_detailed_standings(),
            past_results,
            &mut memo,
        ) {
            Some(matches) => Ok(matches),
            None => Err(ErrorType::AutomaticPairingError),
        }
    }

    fn generate_remaining_pairings(
        &self,
        round_id: &i32,
        bitmask: &i128,
        standings: &Vec<PlayerStanding>,
        past_results: &Box<dyn IResultKeeper>,
        memo: &mut HashMap<i128, Option<Vec<Box<dyn IGameMatch>>>>,
    ) -> Option<Vec<Box<dyn IGameMatch>>> {
        if self.has_all_players_paired(bitmask, standings) {
            return Some(Vec::new());
        }

        if !memo.contains_key(bitmask) {
            let (player_1_idx, player_1_standing) = standings
                .iter()
                .enumerate()
                .map(|(idx, player)| (idx as i32, player))
                .find(|(idx, _)| !self.has_player_paired(bitmask, idx))
                .unwrap();


            let result = standings
                .iter()
                .enumerate()
                .map(|(idx, player)| (idx as i32, player))
                .find(|(idx, _)| {
                    if self.has_player_paired(bitmask, idx) {
                        return false;
                    }
                    if past_results.has_players_met(&player_1_idx, idx) {
                        return false;
                    }

                    let updated_bitmask = self.add_new_pair_to_bitmask(bitmask, &player_1_idx, &idx);
                    self
                        .generate_remaining_pairings(
                            round_id,
                            &updated_bitmask,
                            standings,
                            past_results,
                            memo,
                        )
                        .is_some()
                });

            let pairings = match result {
                Some((player_2_idx, player_2_standing)) => {
                    let pairing = self.generate_pairing(
                        round_id,
                        &player_1_standing.player_id,
                        &player_2_standing.player_id,
                        past_results,
                    );

                    let updated_bitmask = self.add_new_pair_to_bitmask(
                        bitmask,
                        &player_1_idx,
                        &player_2_idx,
                    );
                    let remaining_pairings = self.generate_remaining_pairings(
                        round_id,
                        &updated_bitmask,
                        standings,
                        past_results,
                        memo,
                    ).unwrap();
                    Some([vec![pairing], remaining_pairings].concat())
                }
                None => None
            };
            memo.insert(bitmask.clone(), pairings);
        }

        memo.get(bitmask).unwrap().clone()
    }

    fn has_all_players_paired(&self, bitmask: &i128, standings: &Vec<PlayerStanding>) -> bool {
        let player_count = standings.len();
        let all_players_paired_bitmask = (2 as i128).pow(player_count as u32) - 1;
        bitmask == &all_players_paired_bitmask
    }

    fn has_player_paired(&self, bitmask: &i128, player_index: &i32) -> bool {
        (1 << player_index ^ bitmask) == 0
    }

    fn add_new_pair_to_bitmask(
        &self,
        bitmask: &i128,
        player_1_index: &i32,
        player_2_index: &i32,
    ) -> i128 {
        (1 << player_1_index) | (1 << player_2_index) | bitmask
    }

    fn generate_pairing(
        &self,
        round_id: &i32,
        player_1_id: &i32,
        player_2_id: &i32,
        past_results: &Box<dyn IResultKeeper>,
    ) -> Box<dyn IGameMatch> {
        let player_1_color = self.get_player_1_color(player_1_id, player_2_id, past_results);
        let black_player_id = match player_1_color {
            PlayerColor::Black => player_1_id,
            PlayerColor::White => player_2_id,
        };
        let white_player_id = match player_1_color {
            PlayerColor::Black => player_2_id,
            PlayerColor::White => player_1_id,
        };
        GameMatchCreator::create_new_match(
            round_id,
            &black_player_id,
            &white_player_id,
            &Value::from(Map::new()),
        )
    }

    fn get_player_1_color(
        &self,
        player_1_id: &i32,
        player_2_id: &i32,
        past_results: &Box<dyn IResultKeeper>,
    ) -> PlayerColor {
        let player_1_black_count = past_results.get_color_count(player_1_id, PlayerColor::Black);
        let player_1_white_count = past_results.get_color_count(player_1_id, PlayerColor::White);
        let player_2_black_count = past_results.get_color_count(player_2_id, PlayerColor::Black);
        let player_2_white_count = past_results.get_color_count(player_2_id, PlayerColor::White);

        if player_1_black_count + player_2_white_count > player_2_black_count + player_1_white_count {
            return PlayerColor::White;
        }
        PlayerColor::Black
    }
}

impl PairingGenerator for SwissPairingsGenerator {
    fn generate_pairings(
        &self,
        round_id: &i32,
        players: &Vec<PlayerRowModel>,
        past_results: &Box<dyn IResultKeeper>,
    ) -> Result<Vec<Box<dyn IGameMatch>>, ErrorType> {
        if past_results.is_empty() {
            return Ok(self.generate_first_round_pairings(round_id, players));
        }
        self.generate_normal_pairings(round_id, past_results)
    }
}


#[cfg(test)]
mod tests {
    mod test_swiss_pairing {
        use serde_json::{Map, Value};

        use crate::database_models::PlayerRowModel;
        use crate::game_match::{GameMatchCreator, IGameMatch};
        use crate::pairings_generator::{PairingGenerator, SwissPairingsGenerator};
        use crate::properties::PlayerColor;
        use crate::tournament_manager::{ResultKeeper, IResultKeeper, create_result_keeper};
        use crate::utils::generate_random_string;

        fn create_dummy_player(id: i32, rating: i32) -> PlayerRowModel {
            PlayerRowModel {
                id,
                tournament_id: 0,
                joueurs_id: generate_random_string(5),
                first_name: generate_random_string(10),
                last_name: generate_random_string(10),
                country: generate_random_string(10),
                rating,
                meta_data: Value::from(Map::new()),
            }
        }

        #[test]
        fn test_first_round_even() {
            let player_lists = vec![
                create_dummy_player(1, 1500),
                create_dummy_player(2, 2000),
                create_dummy_player(3, 1000),
                create_dummy_player(4, 200),
                create_dummy_player(5, 3000),
                create_dummy_player(6, 1700),
            ];
            let game_matches: Vec<Box<dyn IGameMatch>> = Vec::new();
            let result_keeper = create_result_keeper(&game_matches);

            let pairings_generator = SwissPairingsGenerator {};
            let pairings = pairings_generator.generate_pairings(&0, &player_lists, &result_keeper).unwrap();

            assert_eq!(pairings[0].get_player_color(&5), Some(PlayerColor::Black));
            assert_eq!(pairings[0].get_player_color(&2), Some(PlayerColor::White));
            assert_eq!(pairings[1].get_player_color(&6), Some(PlayerColor::Black));
            assert_eq!(pairings[1].get_player_color(&1), Some(PlayerColor::White));
            assert_eq!(pairings[2].get_player_color(&3), Some(PlayerColor::Black));
            assert_eq!(pairings[2].get_player_color(&4), Some(PlayerColor::White));
        }

        #[test]
        fn test_first_round_odd() {
            let player_lists = vec![
                create_dummy_player(1, 1500),
                create_dummy_player(2, 2000),
                create_dummy_player(3, 1000),
                create_dummy_player(4, 200),
                create_dummy_player(5, 3000),
            ];
            let game_matches: Vec<Box<dyn IGameMatch>> = Vec::new();
            let result_keeper = create_result_keeper(&game_matches);

            let pairings_generator = SwissPairingsGenerator {};
            let pairings = pairings_generator.generate_pairings(&0, &player_lists, &result_keeper).unwrap();

            assert_eq!(pairings[0].get_player_color(&5), Some(PlayerColor::Black));
            assert_eq!(pairings[0].get_player_color(&2), Some(PlayerColor::White));
            assert_eq!(pairings[1].get_player_color(&1), Some(PlayerColor::Black));
            assert_eq!(pairings[1].get_player_color(&3), Some(PlayerColor::White));
            assert_eq!(pairings[2].get_player_color(&4), None);
            assert_eq!(pairings[2].is_player_playing(&4), true);
        }
    }
}
