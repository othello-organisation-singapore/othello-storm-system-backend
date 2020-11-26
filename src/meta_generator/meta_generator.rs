use serde_json::{Map, Value};

pub trait MetaGenerator {
    fn generate_meta(&self) -> Map<String, Value>;
}