use serde_json::{Map, Value};

use crate::database_models::RoundRowModel;

use super::MetaGenerator;

pub struct RoundPreviewMetaGenerator {
    round: RoundRowModel
}

impl RoundPreviewMetaGenerator {
    pub fn from_round_model(round: RoundRowModel) -> RoundPreviewMetaGenerator {
        RoundPreviewMetaGenerator { round }
    }
}

impl MetaGenerator for RoundPreviewMetaGenerator {
    fn generate_meta(&self) -> Map<String, Value> {
        let mut meta = Map::new();
        meta.insert(String::from("id"), Value::from(self.round.id.clone()));
        meta.insert(String::from("name"), Value::from(self.round.name.clone()));
        meta
    }
}

pub struct RoundDetailsMetaGenerator {
    round: RoundRowModel
}

impl RoundDetailsMetaGenerator {
    pub fn from_round_model(round: RoundRowModel) -> RoundDetailsMetaGenerator {
        RoundDetailsMetaGenerator { round }
    }
}

impl MetaGenerator for RoundDetailsMetaGenerator {
    fn generate_meta(&self) -> Map<String, Value> {
        let mut meta = Map::new();
        meta.insert(String::from("id"), Value::from(self.round.id.clone()));
        meta.insert(String::from("name"), Value::from(self.round.name.clone()));
        meta.insert(String::from("type"), Value::from(self.round.round_type.clone()));
        meta
    }
}
