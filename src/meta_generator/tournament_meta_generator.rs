use std::collections::HashMap;

use diesel::PgConnection;

use crate::database_models::TournamentRowModel;
use super::MetaGenerator;

pub struct TournamentMetaGenerator {
    tournament: TournamentRowModel,
}

impl TournamentMetaGenerator {
    pub fn from_tournament_id(
        id: &i32, connection: &PgConnection
    ) -> Result<TournamentMetaGenerator, String> {
        let tournament = TournamentRowModel::get(id, connection)?;
        Ok(TournamentMetaGenerator::from_tournament(tournament))
    }

    pub fn from_tournament(tournament: TournamentRowModel) -> TournamentMetaGenerator {
        TournamentMetaGenerator { tournament }
    }
}

impl MetaGenerator for TournamentMetaGenerator {
    fn generate_meta(&self) -> HashMap<String, String> {
        let mut meta: HashMap<String, String> = HashMap::new();
        meta.insert(String::from("id"), self.tournament.id.to_string());
        meta.insert(String::from("name"), self.tournament.name.clone());
        meta.insert(
            String::from("tournament_type"), self.tournament.tournament_type.clone()
        );
        meta.insert(String::from("country"), self.tournament.country.clone());
        meta.insert(String::from("creator"), self.tournament.creator.clone());
        meta
    }
}
