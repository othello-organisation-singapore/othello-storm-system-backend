use serde_json::{Map, Value};

use crate::database_models::MatchRowModel;
use crate::tournament_manager::{GameMatch, IGameMatch};

use super::MetaGenerator;

pub struct MatchMetaGenerator {
    game_match: MatchRowModel
}

impl MatchMetaGenerator {
    pub fn from_match_model(game_match: MatchRowModel) -> MatchMetaGenerator {
        MatchMetaGenerator { game_match }
    }
}

impl MetaGenerator for MatchMetaGenerator {
    fn generate_meta(&self) -> Map<String, Value> {
        let mut meta = Map::new();
        meta.insert(String::from("id"), Value::from(self.game_match.id.clone()));
        meta.insert(
            String::from("black_player_id"),
            Value::from(self.game_match.black_player_id.clone()),
        );
        meta.insert(
            String::from("white_player_id"),
            Value::from(self.game_match.white_player_id.clone()),
        );
        meta.insert(
            String::from("black_score"),
            Value::from(self.game_match.black_score.clone()),
        );
        meta.insert(
            String::from("white_score"),
            Value::from(self.game_match.white_score.clone()),
        );

        let game_match = GameMatch::from_match_model(&self.game_match);
        meta.insert(
            String::from("is_finished"),
            Value::from(game_match.is_finished()),
        );
        meta.insert(
            String::from("is_bye"),
            Value::from(game_match.is_bye()),
        );
        meta
    }
}
