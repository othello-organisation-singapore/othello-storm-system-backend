use diesel::PgConnection;
use serde_json::{Map, Value};

use crate::database_models::{UserRowModel, TournamentRowModel};
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

    fn generate_detailed_meta(
        &self, connection: &PgConnection
    ) -> Result<Map<String, Value>, String> {
        let mut meta = self.generate_meta();
        let creator_username = meta
            .get("creator_username")
            .ok_or("Invalid tournament meta format.")?
            .as_str()
            .ok_or("Something is wrong.")?
            .to_string();
        let creator = UserRowModel::get(&creator_username, connection)?;

        let mut creator_meta = Map::new();
        creator_meta.insert(String::from("username"), Value::from(creator_username));
        creator_meta.insert(
            String::from("display_name"),
            Value::from(creator.display_name.clone()))
        ;

        meta.insert(String::from("creator"), Value::from(creator_meta));
        Ok(meta)
    }
}
