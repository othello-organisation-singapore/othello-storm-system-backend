use serde_json::{Map, Value};

use crate::database_models::MatchRowModel;

pub trait IGameMatch where Self: Sized {
    fn from_match_model(match_model: MatchRowModel) -> Self;
    fn create_new(
        round_id: i32,
        black_player_id: i32,
        white_player_id: i32,
        meta_data: Map<String, Value>,
    ) -> Self;
    fn create_new_bye(round_id: i32, player_id: i32, meta_data: Map<String, Value>) -> Self;
    fn is_finished(&self) -> bool;
    fn is_bye(&self) -> bool;
}

pub struct GameMatch {
    pub round_id: i32,
    pub black_player_id: i32,
    pub white_player_id: i32,
    pub black_score: i32,
    pub white_score: i32,
    pub meta_data: Value,
}

impl IGameMatch for GameMatch {
    fn from_match_model(match_model: MatchRowModel) -> Self {
        return GameMatch {
            round_id: match_model.round_id,
            black_player_id: match_model.black_player_id,
            white_player_id: match_model.white_player_id,
            black_score: match_model.black_score,
            white_score: match_model.white_score,
            meta_data: match_model.meta_data,
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
            black_score: 0,
            white_score: 0,
            meta_data: Value::from(meta_data),
        }
    }

    fn create_new_bye(round_id: i32, player_id: i32, meta_data: Map<String, Value>) -> Self {
        let mut updated_meta_data = meta_data.clone();
        updated_meta_data.insert(String::from("is_bye"), Value::from(true));
        GameMatch {
            round_id,
            black_player_id: player_id,
            white_player_id: -1,
            black_score: 33,
            white_score: 31,
            meta_data: Value::from(updated_meta_data),
        }
    }

    fn is_finished(&self) -> bool {
        !(self.black_score == 0 && self.white_score == 0)
    }

    fn is_bye(&self) -> bool {
        self.meta_data
            .as_object()
            .unwrap_or(&Map::new())
            .get("is_bye")
            .unwrap_or(&Value::from(false))
            .as_bool()
            .unwrap_or(false)
    }
}
