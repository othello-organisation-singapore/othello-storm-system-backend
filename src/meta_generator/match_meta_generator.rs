use serde_json::{Map, Value};

use crate::database_models::MatchRowModel;

use super::MetaGenerator;

pub struct MatchMetaGenerator {
    othello_match: MatchRowModel
}

impl MatchMetaGenerator {
    pub fn from_match_model(othello_match: MatchRowModel) -> MatchMetaGenerator {
        MatchMetaGenerator { othello_match }
    }
}

impl MetaGenerator for MatchMetaGenerator {
    fn generate_meta(&self) -> Map<String, Value> {
        let mut meta = Map::new();
        meta.insert(String::from("id"), Value::from(self.othello_match.id.clone()));
        meta.insert(
            String::from("black_player_id"),
            Value::from(self.othello_match.black_player_id.clone())
        );
        meta.insert(
            String::from("white_player_id"),
            Value::from(self.othello_match.white_player_id.clone())
        );
        meta.insert(
            String::from("black_score"),
            Value::from(self.othello_match.black_score.clone())
        );
        meta.insert(
            String::from("white_score"),
            Value::from(self.othello_match.white_score.clone())
        );
        meta
    }
}
