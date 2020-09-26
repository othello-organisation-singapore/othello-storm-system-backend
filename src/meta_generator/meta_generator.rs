use serde_json::{Map, Value};
use diesel::PgConnection;

pub trait MetaGenerator {
    fn generate_meta(&self) -> Map<String, Value>;
    fn generate_detailed_meta(&self, connection: &PgConnection) -> Result<Map<String, Value>, String>;
}