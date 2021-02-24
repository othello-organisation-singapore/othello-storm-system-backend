use std::collections::HashMap;

use serde_json::Value;

use crate::database_models::MatchRowModel;
use crate::properties::PlayerColor;

use super::IGameMatch;

#[derive(Clone, Debug)]
pub struct NormalGameMatch {
    pub round_id: i32,
    pub black_player_id: i32,
    pub white_player_id: i32,
    pub black_score: i32,
    pub white_score: i32,
    pub meta_data: Value,
}

impl IGameMatch for NormalGameMatch {
    fn is_player_playing(&self, player_id: &i32) -> bool {
        player_id == &self.black_player_id || player_id == &self.white_player_id
    }

    fn get_player_color(&self, player_id: &i32) -> Option<PlayerColor> {
        if !self.is_player_playing(player_id) {
            return None;
        }
        if player_id == &self.black_player_id {
            return Some(PlayerColor::Black);
        }
        Some(PlayerColor::White)
    }

    fn get_players_id(&self) -> (Option<i32>, Option<i32>) {
        (Some(self.black_player_id), Some(self.white_player_id))
    }

    fn get_opponent_id(&self, player_id: &i32) -> Option<i32> {
        if !self.is_player_playing(player_id) {
            return None;
        }
        if player_id == &self.black_player_id {
            return Some(self.white_player_id);
        }
        Some(self.black_player_id)
    }

    fn calculate_major_score(&self, player_id: &i32) -> f64 {
        if !self.is_player_playing(player_id) {
            return 0.0;
        }

        if self.black_score == self.white_score {
            return 0.5;
        }

        if player_id == &self.black_player_id && self.black_score > self.white_score {
            return 1.0;
        }

        if player_id == &self.white_player_id && self.white_score > self.black_score {
            return 1.0;
        }

        0.0
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

        let opponent_player_id = match &self.black_player_id == player_id {
            true => self.white_player_id,
            false => self.black_player_id
        };
        let opponent_major_score = major_scores_by_player_ids
            .get(&opponent_player_id)
            .unwrap_or(&0.0);

        let disc_count = match &self.black_player_id == player_id {
            true => f64::from(self.black_score),
            false => f64::from(self.white_score)
        };

        disc_count + brightwell_constant * opponent_major_score
    }

    fn extract_data(&self) -> MatchRowModel {
        MatchRowModel {
            id: -1,
            round_id: self.round_id.clone(),
            black_player_id: self.black_player_id.clone(),
            white_player_id: self.white_player_id.clone(),
            black_score: self.black_score.clone(),
            white_score: self.white_score.clone(),
            meta_data: self.meta_data.clone(),
        }
    }
}
