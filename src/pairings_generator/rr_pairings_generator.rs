use serde_json::{Map, Value};

use crate::database_models::PlayerRowModel;
use crate::errors::ErrorType;
use crate::game_match::{GameMatchCreator, IGameMatch};
use crate::properties::PlayerColor;
use crate::tournament_manager::IResultKeeper;
use crate::utils::generate_random_number_ranged;

use super::{get_player_1_color, PairingGenerator, Pairings};

pub struct RRPairingsGenerator {
    players: Vec<PlayerRowModel>,
    past_results: Box<dyn IResultKeeper>,
}

impl RRPairingsGenerator {
    pub fn new(
        players: Vec<PlayerRowModel>,
        past_results: Box<dyn IResultKeeper>,
    ) -> RRPairingsGenerator {
        RRPairingsGenerator {
            players,
            past_results,
        }
    }

    fn generate_rr_pairings(&self, round_id: &i32, shift: &i32) -> Pairings {
        let bye_player = self.generate_bye_player();

        let mut player_1_vec = self.players.iter().collect::<Vec<&PlayerRowModel>>();
        if player_1_vec.len() % 2 == 1 {
            player_1_vec.push(&bye_player);
        }

        let mut remaining_players = player_1_vec.split_off(1);
        let splitted_players = remaining_players.split_off(*shift as usize);
        let mut shifted_players: Vec<&PlayerRowModel> =
            [player_1_vec, splitted_players, remaining_players].concat();

        let midpoint = (shifted_players.len() / 2) as usize;
        let mut second_part_sorted_players = shifted_players.split_off(midpoint);
        second_part_sorted_players.reverse(); // For the rr algorithm to work
        let mut second_part_players_iter = second_part_sorted_players.iter();

        let mut matches = Vec::new();
        for player_1 in shifted_players {
            let &player_2 = second_part_players_iter.next().unwrap();
            if player_1 == &bye_player {
                matches.push(self.generate_bye_match(round_id, player_2));
                continue;
            }

            if player_2 == &bye_player {
                matches.push(self.generate_bye_match(round_id, player_1));
                continue;
            }

            matches.push(self.generate_match(round_id, player_1, player_2));
        }
        matches
    }

    fn generate_bye_player(&self) -> PlayerRowModel {
        PlayerRowModel {
            id: 0,
            tournament_id: 0,
            joueurs_id: "".to_string(),
            first_name: "".to_string(),
            last_name: "".to_string(),
            country: "".to_string(),
            rating: 0,
            meta_data: Default::default(),
        }
    }

    fn generate_bye_match(&self, round_id: &i32, player: &PlayerRowModel) -> Box<dyn IGameMatch> {
        Box::from(GameMatchCreator::create_new_bye_match(
            round_id,
            &player.id,
            &Value::from(Map::new()),
        ))
    }

    fn generate_match(
        &self,
        round_id: &i32,
        player_1: &PlayerRowModel,
        player_2: &PlayerRowModel,
    ) -> Box<dyn IGameMatch> {
        let player_1_color = get_player_1_color(&player_1.id, &player_2.id, &self.past_results);
        let black_player_id = match player_1_color {
            PlayerColor::Black => player_1.id,
            PlayerColor::White => player_2.id,
        };
        let white_player_id = match player_1_color {
            PlayerColor::White => player_1.id,
            PlayerColor::Black => player_2.id,
        };
        Box::from(GameMatchCreator::create_new_match(
            round_id,
            &black_player_id,
            &white_player_id,
            &Value::from(Map::new()),
        ))
    }

    fn is_players_matched(
        &self,
        pairings: &Pairings,
        player_1_id: &i32,
        player_2_id: &i32,
    ) -> bool {
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
            let shift = generate_random_number_ranged(0, self.players.len() as i32 - 1);
            return Ok(self.generate_rr_pairings(round_id, &shift));
        }

        let standings = self.past_results.get_standings();
        let highest_ranked_player_id = standings.first().unwrap();
        let next_opponent_for_highest_ranked_player = standings.iter().find(|&id| {
            id != highest_ranked_player_id
                && !self
                    .past_results
                    .has_players_met(highest_ranked_player_id, id)
        });

        match next_opponent_for_highest_ranked_player {
            Some(opponent_id) => {
                for shift in 0..standings.len() - 1 {
                    let pairings = self.generate_rr_pairings(round_id, &(shift as i32));
                    if self.is_players_matched(&pairings, highest_ranked_player_id, opponent_id) {
                        return Ok(pairings);
                    }
                }
                Err(ErrorType::AutomaticPairingError)
            }
            None => Err(ErrorType::AutomaticPairingError),
        }
    }
}

#[cfg(test)]
mod tests {
    mod test_rr_pairing {
        use mocktopus::mocking::{MockResult, Mockable};
        use serde_json::{Map, Value};

        use crate::database_models::{MatchRowModel, PlayerRowModel};
        use crate::game_match::{GameMatchCreator, GameMatchTransformer, IGameMatch};
        use crate::pairings_generator::{PairingGenerator, RRPairingsGenerator};
        use crate::properties::PlayerColor;
        use crate::tournament_manager::create_result_keeper;
        use crate::utils;
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
            utils::generate_random_number_ranged.mock_safe(|_, _| MockResult::Return(0));
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

            assert_eq!(pairings[0].get_player_color(&6), Some(PlayerColor::White));
            assert_eq!(pairings[0].get_player_color(&1), Some(PlayerColor::Black));
            assert_eq!(pairings[1].get_player_color(&2), Some(PlayerColor::Black));
            assert_eq!(pairings[1].get_player_color(&5), Some(PlayerColor::White));
            assert_eq!(pairings[2].get_player_color(&3), Some(PlayerColor::Black));
            assert_eq!(pairings[2].get_player_color(&4), Some(PlayerColor::White));
        }

        #[test]
        fn test_first_round_odd() {
            utils::generate_random_number_ranged.mock_safe(|_, _| MockResult::Return(0));
            let player_lists = vec![
                create_dummy_player(1, 1500),
                create_dummy_player(2, 2000),
                create_dummy_player(3, 1000),
                create_dummy_player(4, 200),
                create_dummy_player(5, 3000),
            ];
            let game_matches: Vec<Box<dyn IGameMatch>> = Vec::new();
            let result_keeper = create_result_keeper(&game_matches);

            let pairings_generator = RRPairingsGenerator::new(player_lists, result_keeper);
            let pairings = pairings_generator.generate_pairings(&0).unwrap();

            assert_eq!(pairings[0].get_player_color(&1), None);
            assert_eq!(pairings[0].is_player_playing(&1), true);
            assert_eq!(pairings[1].get_player_color(&2), Some(PlayerColor::Black));
            assert_eq!(pairings[1].get_player_color(&5), Some(PlayerColor::White));
            assert_eq!(pairings[2].get_player_color(&3), Some(PlayerColor::Black));
            assert_eq!(pairings[2].get_player_color(&4), Some(PlayerColor::White));
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
                create_dummy_match(1, 4, 20, 44),
                create_dummy_match(2, 5, 32, 32),
                create_dummy_match(3, 6, 19, 45),
            ];
            let result_keeper = create_result_keeper(&game_matches);

            let pairings_generator = RRPairingsGenerator::new(player_lists, result_keeper);
            let pairings = pairings_generator.generate_pairings(&0).unwrap();

            assert_eq!(pairings[0].get_player_color(&5), Some(PlayerColor::Black));
            assert_eq!(pairings[0].get_player_color(&1), Some(PlayerColor::White));
            assert_eq!(pairings[1].get_player_color(&6), Some(PlayerColor::Black));
            assert_eq!(pairings[1].get_player_color(&4), Some(PlayerColor::White));
            assert_eq!(pairings[2].get_player_color(&2), Some(PlayerColor::Black));
            assert_eq!(pairings[2].get_player_color(&3), Some(PlayerColor::White));
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

            let pairings_generator = RRPairingsGenerator::new(player_lists, result_keeper);
            let pairings = pairings_generator.generate_pairings(&0).unwrap();

            assert_eq!(pairings[0].get_player_color(&1), Some(PlayerColor::Black));
            assert_eq!(pairings[0].get_player_color(&4), Some(PlayerColor::White));
            assert_eq!(pairings[1].get_player_color(&5), Some(PlayerColor::Black));
            assert_eq!(pairings[1].get_player_color(&3), Some(PlayerColor::White));
            assert_eq!(pairings[2].get_player_color(&2), None);
            assert_eq!(pairings[2].is_player_playing(&2), true);
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
                create_dummy_match(1, 3, 20, 44),
                create_dummy_match(4, 2, 32, 32),
                create_dummy_match(1, 2, 20, 44),
                create_dummy_match(3, 4, 33, 31),
            ];
            let result_keeper = create_result_keeper(&game_matches);

            let pairings_generator = RRPairingsGenerator::new(player_lists, result_keeper);
            let pairings = pairings_generator.generate_pairings(&0).unwrap();

            assert_eq!(pairings[0].get_player_color(&4), Some(PlayerColor::Black));
            assert_eq!(pairings[0].get_player_color(&1), Some(PlayerColor::White));
            assert_eq!(pairings[1].get_player_color(&2), Some(PlayerColor::Black));
            assert_eq!(pairings[1].get_player_color(&3), Some(PlayerColor::White));
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
                create_dummy_match(3, 4, 20, 44),
                create_dummy_match(1, 4, 32, 32),
                create_dummy_match(2, 3, 32, 32),
                create_dummy_match(1, 3, 20, 44),
                create_dummy_match(4, 2, 20, 44),
            ];
            let result_keeper = create_result_keeper(&game_matches);

            let pairings_generator = RRPairingsGenerator::new(player_lists, result_keeper);
            let pairings_result = pairings_generator.generate_pairings(&0);

            assert_eq!(pairings_result.is_err(), true);
        }
    }
}
