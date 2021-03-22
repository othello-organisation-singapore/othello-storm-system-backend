use itertools::Itertools;
use serde_json::{Map, Value};

use crate::database_models::PlayerRowModel;
use crate::errors::ErrorType;
use crate::game_match::GameMatchCreator;
use crate::properties::PlayerColor;
use crate::tournament_manager::IResultKeeper;

use super::{PairingGenerator, Pairings, get_player_1_color};

pub struct RRPairingsGenerator {
    players: Vec<PlayerRowModel>,
    past_results: Box<dyn IResultKeeper>,
}

impl RRPairingsGenerator {
    pub fn new(
        players: Vec<PlayerRowModel>,
        past_results: Box<dyn IResultKeeper>,
    ) -> RRPairingsGenerator {
        RRPairingsGenerator { players, past_results }
    }

    fn generate_rr_pairings(&self, round_id: &i32, shift: &i32) -> Pairings {
        let mut player_1_vec = self.players.iter().collect::<Vec<&PlayerRowModel>>();
        let mut remaining_players = player_1_vec.split_off(1);
        let splitted_players = remaining_players.split_off(*shift as usize);

        let mut shifted_players: Vec<&PlayerRowModel> = [
            player_1_vec,
            splitted_players,
            remaining_players,
        ].concat();
        let mut matches = Vec::new();

        let midpoint = (shifted_players.len() as f32 / 2 as f32).ceil() as usize;
        let second_part_sorted_players = shifted_players.split_off(midpoint);
        let mut second_part_players_iter = second_part_sorted_players.iter();

        for player_1 in shifted_players {
            let player_2_option = second_part_players_iter.next();
            match player_2_option {
                Some(player_2) => {
                    let player_1_color = get_player_1_color(
                        &player_1.id,
                        &player_2.id,
                        &self.past_results,
                    );

                    let black_player_id = match player_1_color {
                        PlayerColor::Black => player_1.id,
                        PlayerColor::White => player_2.id,
                    };
                    let white_player_id = match player_1_color {
                        PlayerColor::White => player_1.id,
                        PlayerColor::Black => player_2.id,
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

    fn is_players_matched(&self, pairings: &Pairings, player_1_id: &i32, player_2_id: &i32) -> bool {
        pairings
            .iter()
            .find(|pairing| {
                let result = pairing.get_opponent_id(player_1_id);
                result.is_some() && &result.unwrap() == player_2_id
            })
            .is_some()
    }
}

impl PairingGenerator for RRPairingsGenerator {
    fn generate_pairings(&self, round_id: &i32) -> Result<Pairings, ErrorType> {
        if self.past_results.is_empty() {
            return Ok(self.generate_rr_pairings(round_id, &0))
        }

        let standings = self.past_results.get_standings();
        let highest_ranked_player_id = standings.first().unwrap();
        let next_opponent_for_highest_ranked_player = standings
            .iter()
            .find(|&id| {
                id != highest_ranked_player_id
                    && !self.past_results.has_players_met(highest_ranked_player_id, id)
            });

        match next_opponent_for_highest_ranked_player {
            Some(opponent_id) => {
                for shift in 0..standings.len() - 2 {
                    let pairings = self.generate_rr_pairings(round_id, &(shift as i32));


                    if self.is_players_matched(&pairings, highest_ranked_player_id, opponent_id) {
                        return Ok(pairings);
                    }
                }
                Err(ErrorType::AutomaticPairingError)
            }
            None => Err(ErrorType::AutomaticPairingError)
        }
    }
}


#[cfg(test)]
mod tests {
    mod test_rr_pairing {
        use serde_json::{Map, Value};
        use std::collections::HashSet;

        use crate::database_models::{PlayerRowModel, MatchRowModel};
        use crate::game_match::{GameMatchCreator, GameMatchTransformer, IGameMatch};
        use crate::pairings_generator::{PairingGenerator, RRPairingsGenerator};
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

            let pairings_generator = RRPairingsGenerator::new(player_lists, result_keeper);
            let pairings = pairings_generator.generate_pairings(&0).unwrap();

            assert_eq!(pairings[0].get_player_color(&1), Some(PlayerColor::Black));
            assert_eq!(pairings[0].get_player_color(&4), Some(PlayerColor::White));
            assert_eq!(pairings[1].get_player_color(&2), Some(PlayerColor::Black));
            assert_eq!(pairings[1].get_player_color(&5), Some(PlayerColor::White));
            assert_eq!(pairings[2].get_player_color(&3), Some(PlayerColor::Black));
            assert_eq!(pairings[2].get_player_color(&6), Some(PlayerColor::White));
        }
    }
}
