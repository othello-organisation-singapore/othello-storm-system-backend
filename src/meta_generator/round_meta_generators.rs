use serde_json::{Map, Value};

use crate::database_models::RoundRowModel;

pub trait RoundMetaGenerator {
    fn generate_meta_for(&self, round: &RoundRowModel) -> Map<String, Value>;
}

pub struct RoundPreviewMetaGenerator {}

impl RoundMetaGenerator for RoundPreviewMetaGenerator {
    fn generate_meta_for(&self, round: &RoundRowModel) -> Map<String, Value> {
        let mut meta = Map::new();
        meta.insert(String::from("id"), Value::from(round.id.clone()));
        meta.insert(String::from("name"), Value::from(round.name.clone()));
        meta
    }
}

pub struct RoundDetailsMetaGenerator {}

impl RoundMetaGenerator for RoundDetailsMetaGenerator {
    fn generate_meta_for(&self, round: &RoundRowModel) -> Map<String, Value> {
        let mut meta = Map::new();
        meta.insert(String::from("id"), Value::from(round.id.clone()));
        meta.insert(String::from("name"), Value::from(round.name.clone()));
        meta.insert(String::from("type"), Value::from(round.round_type.clone()));
        meta
    }
}
