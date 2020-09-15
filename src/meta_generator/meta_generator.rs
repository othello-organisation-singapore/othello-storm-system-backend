use std::collections::HashMap;

pub trait MetaGenerator {
    fn generate_meta(&self) -> HashMap<String, String>;
}