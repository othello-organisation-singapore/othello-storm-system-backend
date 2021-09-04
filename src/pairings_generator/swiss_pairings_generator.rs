use std::collections::HashMap;

use itertools::Itertools;
use serde_json::{Map, Value};

use crate::database_models::PlayerRowModel;
use crate::errors::ErrorType;
use crate::game_match::{GameMatchCreator, IGameMatch};
use crate::properties::PlayerColor;
use crate::tournament_manager::IResultKeeper;

use super::{get_player_1_color, PairingGenerator, Pairings};

pub struct SwissPairingsGenerator {
    players: Vec<PlayerRowModel>,
    past_results: Box<dyn IResultKeeper>,
}

impl SwissPairingsGenerator {
    pub fn new(
        players: Vec<PlayerRowModel>,
        past_results: Box<dyn IResultKeeper>,
    ) -> SwissPairingsGenerator {
        SwissPairingsGenerator {
            players,
            past_results,
        }
    }

    fn generate_first_round_pairings(&self, round_id: &i32) -> Pairings {
        let mut matches = Vec::new();
        let mut sorted_players = self.players[..]
            .into_iter()
            .sorted_by_key(|player| -player.rating)
            .collect::<Vec<&PlayerRowModel>>();

        let midpoint = (sorted_players.len() as f32 / 2 as f32).ceil() as usize;
        let second_part_sorted_players = sorted_players.split_off(midpoint);
        let mut second_part_players_iter = second_part_sorted_players.iter();

        for (idx, player_1) in sorted_players.iter().enumerate() {
            let player_2_option = second_part_players_iter.next();
            match player_2_option {
                Some(player_2) => {
                    let black_player_id = match idx % 2 == 0 {
                        true => player_1.id,
                        false => player_2.id,
                    };

                    let white_player_id = match idx % 2 == 0 {
                        true => player_2.id,
                        false => player_1.id,
                    };
                    matches.push(Box::from(GameMatchCreator::create_new_match(
                        round_id,
                        &black_player_id,
                        &white_player_id,
                        &Value::from(Map::new()),
                    )));
                }
                None => {
                    matches.push(Box::from(GameMatchCreator::create_new_bye_match(
                        round_id,
                        &player_1.id,
                        &Value::from(Map::new()),
                    )));
                }
            }
        }
        matches
    }

    fn generate_normal_pairings(
        &self,
        round_id: &i32,
    ) -> Result<Vec<Box<dyn IGameMatch>>, ErrorType> {
        let mut memo: HashMap<i128, Option<Pairings>> = HashMap::new();
        match self.generate_remaining_pairings(round_id, &(0 as i128), &mut memo) {
            Some(matches) => Ok(matches),
            None => Err(ErrorType::AutomaticPairingError),
        }
    }

    fn generate_remaining_pairings(
        &self,
        round_id: &i32,
        bitmask: &i128,
        memo: &mut HashMap<i128, Option<Pairings>>,
    ) -> Option<Pairings> {
        if self.has_all_players_paired(bitmask) {
            return Some(Vec::new());
        }

        let standings = self.past_results.get_detailed_standings();
        if !memo.contains_key(bitmask) {
            let (player_1_idx, player_1_standing) = standings
                .iter()
                .enumerate()
                .map(|(idx, player)| (idx as i32, player))
                .find(|(idx, _)| !self.has_player_paired(bitmask, idx))
                .unwrap();

            let pairings = match self.get_no_of_unpaired_players(bitmask) {
                1 => {
                    if self
                        .past_results
                        .has_player_bye(&player_1_standing.player_id)
                    {
                        return None;
                    }

                    let pairing = GameMatchCreator::create_new_bye_match(
                        round_id,
                        &player_1_standing.player_id,
                        &Value::from(Map::new()),
                    );
                    let updated_bitmask =
                        self.add_new_pair_to_bitmask(bitmask, &player_1_idx, &player_1_idx);
                    let remaining_pairings = self
                        .generate_remaining_pairings(round_id, &updated_bitmask, memo)
                        .unwrap();
                    Some([vec![pairing], remaining_pairings].concat())
                }
                _ => {
                    let result = standings
                        .iter()
                        .enumerate()
                        .map(|(idx, player)| (idx as i32, player))
                        .find(|(idx, player_2_standing)| {
                            if idx == &player_1_idx {
                                return false;
                            }

                            if self.has_player_paired(bitmask, idx) {
                                return false;
                            }

                            if self.past_results.has_players_met(
                                &player_1_standing.player_id,
                                &player_2_standing.player_id,
                            ) {
                                return false;
                            }

                            let updated_bitmask =
                                self.add_new_pair_to_bitmask(bitmask, &player_1_idx, &idx);
                            self.generate_remaining_pairings(round_id, &updated_bitmask, memo)
                                .is_some()
                        });

                    match result {
                        Some((player_2_idx, player_2_standing)) => {
                            let pairing = self.generate_pairing(
                                round_id,
                                &player_1_standing.player_id,
                                &player_2_standing.player_id,
                            );

                            let updated_bitmask =
                                self.add_new_pair_to_bitmask(bitmask, &player_1_idx, &player_2_idx);
                            let remaining_pairings = self
                                .generate_remaining_pairings(round_id, &updated_bitmask, memo)
                                .unwrap();
                            Some([vec![pairing], remaining_pairings].concat())
                        }
                        None => None,
                    }
                }
            };

            memo.insert(bitmask.clone(), pairings);
        }

        memo.get(bitmask).unwrap().clone()
    }

    fn has_all_players_paired(&self, bitmask: &i128) -> bool {
        let player_count = self.past_results.get_standings().len();
        let all_players_paired_bitmask = (2 as i128).pow(player_count as u32) - 1;
        bitmask == &all_players_paired_bitmask
    }

    fn has_player_paired(&self, bitmask: &i128, player_index: &i32) -> bool {
        ((1 << player_index) & bitmask) != 0
    }

    fn get_no_of_unpaired_players(&self, bitmask: &i128) -> i32 {
        let mut cnt = 0;
        for i in 0..self.past_results.get_standings().len() {
            if !self.has_player_paired(bitmask, &(i as i32)) {
                cnt += 1;
            }
        }
        cnt
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
    ) -> Box<dyn IGameMatch> {
        let player_1_color = get_player_1_color(player_1_id, player_2_id, &self.past_results);
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
}

impl PairingGenerator for SwissPairingsGenerator {
    fn generate_pairings(&self, round_id: &i32) -> Result<Pairings, ErrorType> {
        if self.past_results.is_empty() {
            return Ok(self.generate_first_round_pairings(round_id));
        }
        self.generate_normal_pairings(round_id)
    }
}

#[cfg(test)]
mod tests {
    mod test_swiss_pairing {
        use serde_json::{Map, Value};

        use crate::database_models::{MatchRowModel, PlayerRowModel};
        use crate::game_match::{GameMatchCreator, GameMatchTransformer, IGameMatch};
        use crate::pairings_generator::{PairingGenerator, SwissPairingsGenerator};
        use crate::properties::PlayerColor;
        use crate::tournament_manager::create_result_keeper;
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

        fn create_dummy_match(
            black_player_id: i32,
            white_player_id: i32,
            black_score: i32,
            white_score: i32,
        ) -> Box<dyn IGameMatch> {
            let match_model = MatchRowModel {
                id: 0,
                round_id: 0,
                black_player_id,
                white_player_id,
                black_score,
                white_score,
                meta_data: Value::from(Map::new()),
            };
            GameMatchTransformer::transform_to_game_match(&match_model)
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

            let pairings_generator = SwissPairingsGenerator::new(player_lists, result_keeper);
            let pairings = pairings_generator.generate_pairings(&0).unwrap();

            assert_eq!(pairings[0].get_player_color(&5), Some(PlayerColor::Black));
            assert_eq!(pairings[0].get_player_color(&1), Some(PlayerColor::White));
            assert_eq!(pairings[1].get_player_color(&3), Some(PlayerColor::Black));
            assert_eq!(pairings[1].get_player_color(&2), Some(PlayerColor::White));
            assert_eq!(pairings[2].get_player_color(&6), Some(PlayerColor::Black));
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

            let pairings_generator = SwissPairingsGenerator::new(player_lists, result_keeper);
            let pairings = pairings_generator.generate_pairings(&0).unwrap();

            assert_eq!(pairings[0].get_player_color(&5), Some(PlayerColor::Black));
            assert_eq!(pairings[0].get_player_color(&3), Some(PlayerColor::White));
            assert_eq!(pairings[1].get_player_color(&4), Some(PlayerColor::Black));
            assert_eq!(pairings[1].get_player_color(&2), Some(PlayerColor::White));
            assert_eq!(pairings[2].get_player_color(&1), None);
            assert_eq!(pairings[2].is_player_playing(&1), true);
        }

        #[test]
        fn test_normal_round_even() {
            let player_lists = vec![
                create_dummy_player(1, 1500),
                create_dummy_player(2, 2000),
                create_dummy_player(3, 1000),
                create_dummy_player(4, 200),
                create_dummy_player(5, 3000),
                create_dummy_player(6, 1700),
            ];
            let game_matches = vec![
                create_dummy_match(5, 1, 20, 44),
                create_dummy_match(3, 2, 32, 32),
                create_dummy_match(6, 4, 19, 45),
            ];
            let result_keeper = create_result_keeper(&game_matches);

            let pairings_generator = SwissPairingsGenerator::new(player_lists, result_keeper);
            let pairings = pairings_generator.generate_pairings(&0).unwrap();

            assert_eq!(pairings[0].get_player_color(&4), Some(PlayerColor::Black));
            assert_eq!(pairings[0].get_player_color(&1), Some(PlayerColor::White));
            assert_eq!(pairings[1].get_player_color(&2), Some(PlayerColor::Black));
            assert_eq!(pairings[1].get_player_color(&5), Some(PlayerColor::White));
            assert_eq!(pairings[2].get_player_color(&3), Some(PlayerColor::Black));
            assert_eq!(pairings[2].get_player_color(&6), Some(PlayerColor::White));
        }

        #[test]
        fn test_normal_round_odd() {
            let player_lists = vec![
                create_dummy_player(1, 1500),
                create_dummy_player(2, 2000),
                create_dummy_player(3, 1000),
                create_dummy_player(4, 200),
                create_dummy_player(5, 3000),
            ];
            let game_matches = vec![
                create_dummy_match(5, 1, 20, 44),
                create_dummy_match(3, 2, 32, 32),
                GameMatchCreator::create_new_bye_match(&0, &4, &Value::from(Map::new())),
            ];
            let result_keeper = create_result_keeper(&game_matches);

            let pairings_generator = SwissPairingsGenerator::new(player_lists, result_keeper);
            let pairings = pairings_generator.generate_pairings(&0).unwrap();

            assert_eq!(pairings[0].get_player_color(&1), Some(PlayerColor::Black));
            assert_eq!(pairings[0].get_player_color(&4), Some(PlayerColor::White));
            assert_eq!(pairings[1].get_player_color(&2), Some(PlayerColor::Black));
            assert_eq!(pairings[1].get_player_color(&5), Some(PlayerColor::White));
            assert_eq!(pairings[2].get_player_color(&3), None);
            assert_eq!(pairings[2].is_player_playing(&3), true);
        }

        #[test]
        fn test_normal_round_only_one_possibility() {
            let player_lists = vec![
                create_dummy_player(1, 1500),
                create_dummy_player(2, 2000),
                create_dummy_player(3, 1000),
                create_dummy_player(4, 200),
            ];
            let game_matches = vec![
                create_dummy_match(1, 2, 20, 44),
                create_dummy_match(3, 4, 32, 32),
                create_dummy_match(1, 3, 20, 44),
                create_dummy_match(2, 4, 33, 31),
            ];
            let result_keeper = create_result_keeper(&game_matches);

            let pairings_generator = SwissPairingsGenerator::new(player_lists, result_keeper);
            let pairings = pairings_generator.generate_pairings(&0).unwrap();

            assert_eq!(pairings[0].get_player_color(&2), Some(PlayerColor::Black));
            assert_eq!(pairings[0].get_player_color(&3), Some(PlayerColor::White));
            assert_eq!(pairings[1].get_player_color(&4), Some(PlayerColor::Black));
            assert_eq!(pairings[1].get_player_color(&1), Some(PlayerColor::White));
        }

        #[test]
        fn test_normal_round_no_possibility() {
            let player_lists = vec![
                create_dummy_player(1, 1500),
                create_dummy_player(2, 2000),
                create_dummy_player(3, 1000),
                create_dummy_player(4, 200),
            ];
            let game_matches = vec![
                create_dummy_match(1, 2, 20, 44),
                create_dummy_match(1, 4, 32, 32),
                create_dummy_match(1, 3, 20, 44),
            ];
            let result_keeper = create_result_keeper(&game_matches);

            let pairings_generator = SwissPairingsGenerator::new(player_lists, result_keeper);
            let pairings_result = pairings_generator.generate_pairings(&0);

            assert_eq!(pairings_result.is_err(), true);
        }

        #[test]
        fn test_normal_round_double_bye() {
            let player_lists = vec![
                create_dummy_player(4449, 1500),
                create_dummy_player(4486, 2000),
                create_dummy_player(4487, 1000),
                create_dummy_player(4488, 200),
                create_dummy_player(4489, 200),
                create_dummy_player(4490, 200),
                create_dummy_player(4491, 200),
                create_dummy_player(4492, 200),
                create_dummy_player(4493, 200),
            ];
            let game_matches = vec![
                create_dummy_match(4449, 4486, 31, 33),
                create_dummy_match(4489, 4490, 32, 32),
                create_dummy_match(4491, 4492, 32, 32),
                create_dummy_match(4487, 4488, 0, 64),
                create_dummy_match(4493, -1, -2, -2),
                create_dummy_match(4488, 4493, 40, 24),
                create_dummy_match(4486, 4489, 31, 33),
                create_dummy_match(4490, 4491, 32, 32),
                create_dummy_match(4492, 4449, 36, 28),
                create_dummy_match(4487, -1, -2, -2),
                create_dummy_match(4488, 4489, 35, 29),
                create_dummy_match(4492, 4490, 39, 25),
                create_dummy_match(4493, 4491, 32, 32),
                create_dummy_match(4486, 4487, 30, 34),
                create_dummy_match(4449, -1, -2, -2),
            ];
            let result_keeper = create_result_keeper(&game_matches);

            let pairings_generator = SwissPairingsGenerator::new(player_lists, result_keeper);
            let pairings = pairings_generator.generate_pairings(&0).unwrap();

            assert_eq!(pairings[0].get_player_color(&4488), Some(PlayerColor::Black));
            assert_eq!(pairings[0].get_player_color(&4492), Some(PlayerColor::White));
            assert_eq!(pairings[1].get_player_color(&4491), Some(PlayerColor::Black));
            assert_eq!(pairings[1].get_player_color(&4487), Some(PlayerColor::White));
            assert_eq!(pairings[2].get_player_color(&4489), Some(PlayerColor::Black));
            assert_eq!(pairings[2].get_player_color(&4493), Some(PlayerColor::White));
            assert_eq!(pairings[3].get_player_color(&4490), Some(PlayerColor::Black));
            assert_eq!(pairings[3].get_player_color(&4449), Some(PlayerColor::White));
            assert_eq!(pairings[4].get_player_color(&4486), None);
            assert_eq!(pairings[4].is_player_playing(&4486), true);
        }
    }
}
