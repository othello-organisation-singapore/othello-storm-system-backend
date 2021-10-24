use serde_json::{Map, Value};

use crate::game_match::GameMatchTransformer;
use crate::tournament_manager::PlayerStanding;

use super::{generate_matches_meta};

pub trait StandingMetaGenerator {
    fn generate_meta_for(&self, standing: &PlayerStanding) -> Map<String, Value>;
}

pub struct DefaultStandingMetaGenerator {}


impl StandingMetaGenerator for DefaultStandingMetaGenerator {
    fn generate_meta_for(&self, standing: &PlayerStanding) -> Map<String, Value> {
        let mut meta = Map::new();
        meta.insert(
            String::from("player_id"),
            Value::from(standing.player_id),
        );
        meta.insert(
            String::from("major_score"),
            Value::from(standing.major_score),
        );
        meta.insert(
            String::from("minor_score"),
            Value::from(standing.minor_score),
        );

        let matches_meta = generate_matches_meta(
            standing
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
