use itertools::Itertools;
use serde_json::{Map, Value};
use std::collections::HashMap;

use crate::database_models::PlayerRowModel;
use crate::game_match::{GameMatchCreator, IGameMatch};
use crate::tournament_manager::IResultKeeper;

use super::PairingGenerator;
use crate::errors::ErrorType;
use crate::properties::PlayerColor;

struct SwissPairingsGenerator {}

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
        players: &Vec<PlayerRowModel>,
        past_results: &Box<dyn IResultKeeper>,
    ) -> Result<Vec<Box<dyn IGameMatch>>, ErrorType> {
        let mut memo: HashMap<i128, Option<Vec<Box<dyn IGameMatch>>>> = HashMap::new();
        match self.generate_remaining_pairings(
            round_id,
            &(0 as i128),
            players,
            past_results,
            &mut memo,
        ) {
            Some(matches) => Ok(matches),
            None => Err(ErrorType::AutomaticPairingError),
        }
    }

    // TODO: use player_ids from standings instead of players
    fn generate_remaining_pairings(
        &self,
        round_id: &i32,
        bitmask: &i128,
        players: &Vec<PlayerRowModel>,
        past_results: &Box<dyn IResultKeeper>,
        memo: &mut HashMap<i128, Option<Vec<Box<dyn IGameMatch>>>>,
    ) -> Option<Vec<Box<dyn IGameMatch>>> {
        if self.has_all_players_paired(bitmask, players) {
            return Some(Vec::new());
        }

        if !memo.contains_key(bitmask) {
            let (player_1_idx, player_1) = players
                .iter()
                .enumerate()
                .map(|(idx, player)| (idx as i32, player))
                .find(|(idx, _)| !self.has_player_paired(bitmask, idx))
                .unwrap();


            let result = players
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
                    self.generate_remaining_pairings(round_id, &updated_bitmask, players, past_results, memo)
                        .is_some()
                });

            let pairings = match result {
                Some((player_2_idx, player_2)) => {
                    let pairing = self.generate_pairing(round_id, player_1, player_2, past_results);

                    let updated_bitmask = self.add_new_pair_to_bitmask(bitmask, &player_1_idx, &player_2_idx);
                    let remaining_pairings = self.generate_remaining_pairings(
                        round_id,
                        &updated_bitmask,
                        players,
                        past_results,
                        memo,
                    ).unwrap();
                    Some([vec![pairing], remaining_pairings].concat())
                },
                None => None
            };
            memo.insert(bitmask.clone(), pairings);
        }

        memo.get(bitmask).unwrap().clone()
    }

    fn has_all_players_paired(&self, bitmask: &i128, players: &Vec<PlayerRowModel>) -> bool {
        let player_count = players.len();
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
        player_1: &PlayerRowModel,
        player_2: &PlayerRowModel,
        past_results: &Box<dyn IResultKeeper>,
    ) -> Box<dyn IGameMatch> {
        let player_1_color = self.get_player_1_color(player_1, player_2, past_results);
        let black_player_id = match player_1_color {
            PlayerColor::Black => player_1.id,
            PlayerColor::White => player_2.id,
        };
        let white_player_id = match player_1_color {
            PlayerColor::Black => player_2.id,
            PlayerColor::White => player_1.id,
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
        player_1: &PlayerRowModel,
        player_2: &PlayerRowModel,
        past_results: &Box<dyn IResultKeeper>,
    ) -> PlayerColor {
        let player_1_black_count = past_results.get_color_count(&player_1.id, PlayerColor::Black);
        let player_1_white_count = past_results.get_color_count(&player_1.id, PlayerColor::White);
        let player_2_black_count = past_results.get_color_count(&player_2.id, PlayerColor::Black);
        let player_2_white_count = past_results.get_color_count(&player_2.id, PlayerColor::White);

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
        self.generate_normal_pairings(round_id, players, past_results)
    }
}
