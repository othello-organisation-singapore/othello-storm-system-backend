use serde_json::{Map, Value};

use crate::database_models::MatchRowModel;

pub trait MatchMetaGenerator {
    fn generate_meta_for(&self, game_match: &MatchRowModel) -> Map<String, Value>;
}

pub struct DefaultMatchMetaGenerator {}

impl MatchMetaGenerator for DefaultMatchMetaGenerator {
    fn generate_meta_for(&self, game_match: &MatchRowModel) -> Map<String, Value> {
        let mut meta = Map::new();
        meta.insert(String::from("id"), Value::from(game_match.id.clone()));
        meta.insert(
            String::from("black_player_id"),
            Value::from(game_match.black_player_id.clone()),
        );
        meta.insert(
            String::from("white_player_id"),
            Value::from(game_match.white_player_id.clone()),
        );
        meta.insert(
            String::from("black_score"),
            Value::from(game_match.black_score.clone()),
        );
        meta.insert(
            String::from("white_score"),
            Value::from(game_match.white_score.clone()),
        );
        meta
    }
}
