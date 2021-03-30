use serde_json::{Map, Value};

use super::{generate_matches_meta, MetaGenerator};
use crate::game_match::GameMatchTransformer;
use crate::tournament_manager::PlayerStanding;

pub struct StandingMetaGenerator {
    standing: PlayerStanding,
}

impl StandingMetaGenerator {
    pub fn from_standing(standing: PlayerStanding) -> StandingMetaGenerator {
        StandingMetaGenerator { standing }
    }
}

impl MetaGenerator for StandingMetaGenerator {
    fn generate_meta(&self) -> Map<String, Value> {
        let mut meta = Map::new();
        meta.insert(
            String::from("player_id"),
            Value::from(self.standing.player_id),
        );
        meta.insert(
            String::from("major_score"),
            Value::from(self.standing.major_score),
        );
        meta.insert(
            String::from("minor_score"),
            Value::from(self.standing.minor_score),
        );

        let matches_meta = generate_matches_meta(
            self.standing
                .match_history
                .clone()
                .into_iter()
                .map(|game_match| GameMatchTransformer::transform_to_match_model_data(&game_match))
                .collect(),
        );
        meta.insert(String::from("match_history"), Value::from(matches_meta));
        meta
    }
}
