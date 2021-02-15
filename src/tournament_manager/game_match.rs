use serde_json::{Map, Value};

use crate::database_models::MatchRowModel;
use crate::properties::SpecialConditionScore;

pub trait IGameMatch where Self: Sized {
    fn from_match_model(match_model: &MatchRowModel) -> Self;
    fn create_new(
        round_id: i32,
        black_player_id: i32,
        white_player_id: i32,
        meta_data: Map<String, Value>,
    ) -> Self;
    fn create_new_bye(round_id: i32, player_id: i32, meta_data: Map<String, Value>) -> Self;
    fn is_finished(&self) -> bool;
    fn is_bye(&self) -> bool;
    fn get_players_id(&self) -> (Option<i32>, Option<i32>);
}

#[derive(Clone, Debug, PartialEq)]
pub struct GameMatch {
    pub round_id: i32,
    pub black_player_id: i32,
    pub white_player_id: i32,
    pub black_score: i32,
    pub white_score: i32,
    pub meta_data: Value,
}

impl IGameMatch for GameMatch {
    fn from_match_model(match_model: &MatchRowModel) -> Self {
        return GameMatch {
            round_id: match_model.round_id,
            black_player_id: match_model.black_player_id,
            white_player_id: match_model.white_player_id,
            black_score: match_model.black_score,
            white_score: match_model.white_score,
            meta_data: match_model.meta_data.clone(),
        };
    }

    fn create_new(
        round_id: i32,
        black_player_id: i32,
        white_player_id: i32,
        meta_data: Map<String, Value>,
    ) -> Self {
        GameMatch {
            round_id,
            black_player_id,
            white_player_id,
            black_score: SpecialConditionScore::NotFinished.to_i32(),
            white_score: SpecialConditionScore::NotFinished.to_i32(),
            meta_data: Value::from(meta_data),
        }
    }

    fn create_new_bye(round_id: i32, player_id: i32, meta_data: Map<String, Value>) -> Self {
        GameMatch {
            round_id,
            black_player_id: player_id,
            white_player_id: -1,
            black_score: SpecialConditionScore::Bye.to_i32(),
            white_score: SpecialConditionScore::Bye.to_i32(),
            meta_data: Value::from(meta_data),
        }
    }

    fn is_finished(&self) -> bool {
        !(self.black_score == -1 && self.white_score == -1)
    }

    fn is_bye(&self) -> bool {
        let bye_score = SpecialConditionScore::Bye.to_i32();
        self.black_score == bye_score && self.white_score == bye_score
    }

    fn get_players_id(&self) -> (Option<i32>, Option<i32>) {
        if self.is_bye() {
            return (Some(self.black_player_id), None);
        }
        (Some(self.black_player_id), Some(self.white_player_id))
    }
}
