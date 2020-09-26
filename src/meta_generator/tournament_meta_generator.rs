use diesel::PgConnection;
use serde_json::{Map, Value};

use crate::database_models::TournamentRowModel;
use super::MetaGenerator;

pub struct TournamentMetaGenerator {
    tournament: TournamentRowModel
}

impl TournamentMetaGenerator {
    pub fn from_tournament_id(
        id: &i32, connection: &PgConnection,
    ) -> Result<TournamentMetaGenerator, String> {
        let tournament = TournamentRowModel::get(id, connection)?;
        Ok(TournamentMetaGenerator::from_tournament(tournament))
    }

    pub fn from_tournament(tournament: TournamentRowModel) -> TournamentMetaGenerator {
        TournamentMetaGenerator { tournament }
    }
}

impl MetaGenerator for TournamentMetaGenerator {
    fn generate_meta(&self) -> Map<String, Value> {
        let mut meta = Map::new();
        meta.insert(String::from("id"), Value::from(self.tournament.id.to_string()));
        meta.insert(String::from("name"), Value::from(self.tournament.name.clone()));
        meta.insert(
            String::from("tournament_type"),
            Value::from(self.tournament.tournament_type.clone()),
        );
        meta.insert(
            String::from("country"), Value::from(self.tournament.country.clone()),
        );
        meta.insert(
            String::from("creator_username"),
            Value::from(self.tournament.creator.clone()),
        );
        meta
    }
}
