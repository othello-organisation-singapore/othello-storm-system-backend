use serde_json::{Map, Value};

use crate::database_models::{UserRowModel, TournamentRowModel};
use super::{MetaGenerator};

pub struct TournamentPreviewMetaGenerator {
    tournament: TournamentRowModel
}

pub struct TournamentDetailsMetaGenerator {
    tournament: TournamentRowModel,
    creator: UserRowModel,
}

impl TournamentPreviewMetaGenerator {
    pub fn from_tournament(tournament: TournamentRowModel) -> TournamentPreviewMetaGenerator {
        TournamentPreviewMetaGenerator { tournament }
    }
}

impl MetaGenerator for TournamentPreviewMetaGenerator {
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

impl TournamentDetailsMetaGenerator {
    pub fn from_tournament(
        tournament: TournamentRowModel,
        creator: UserRowModel,
    ) -> TournamentDetailsMetaGenerator {
        TournamentDetailsMetaGenerator { tournament, creator }
    }
}

impl MetaGenerator for TournamentDetailsMetaGenerator {
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

        let mut creator_meta = Map::new();
        creator_meta.insert(String::from("username"), Value::from(self.creator.username.clone()));
        creator_meta.insert(
            String::from("display_name"),
            Value::from(self.creator.display_name.clone()))
        ;
        meta.insert(String::from("creator"), Value::from(creator_meta));

        meta
    }
}

