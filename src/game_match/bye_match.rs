use std::collections::HashMap;

use serde_json::Value;

use crate::database_models::MatchRowModel;
use crate::properties::{PlayerColor, SpecialConditionScore};

use super::IGameMatch;

#[derive(Clone, Debug)]
pub struct ByeGameMatch {
    pub round_id: i32,
    pub player_id: i32,
    pub meta_data: Value,
}

impl IGameMatch for ByeGameMatch {
    fn is_player_playing(&self, player_id: &i32) -> bool {
        player_id == &self.player_id
    }

    fn get_player_color(&self, player_id: &i32) -> Option<PlayerColor> {
        if !self.is_player_playing(player_id) {
            return None;
        }
        None
    }

    fn get_players_id(&self) -> (Option<i32>, Option<i32>) {
        (Some(self.player_id), None)
    }

    fn get_opponent_id(&self, player_id: &i32) -> Option<i32> {
        if !self.is_player_playing(player_id) {
            return None;
        }
        None
    }

    fn calculate_major_score(&self, player_id: &i32) -> f64 {
        if !self.is_player_playing(player_id) {
            return 0.0;
        }
        1.0
    }

    fn calculate_minor_score(
        &self,
        player_id: &i32,
        major_scores_by_player_ids: &HashMap<i32, f64>,
        brightwell_constant: &f64,
    ) -> f64 {
        if !self.is_player_playing(player_id) {
            return 0.0;
        }

        let self_major_score = major_scores_by_player_ids.get(player_id).unwrap_or(&0.0);

        return 32.0 + brightwell_constant * self_major_score;
    }

    fn extract_data(&self) -> MatchRowModel {
        MatchRowModel {
            id: -1,
            round_id: self.round_id.clone(),
            black_player_id: self.player_id.clone(),
            white_player_id: -1,
            black_score: SpecialConditionScore::Bye.to_i32(),
            white_score: SpecialConditionScore::Bye.to_i32(),
            meta_data: self.meta_data.clone(),
        }
    }
}
