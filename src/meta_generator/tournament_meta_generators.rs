use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::database_models::{TournamentRowModel, UserRowModel};
use crate::utils::date_to_string;

use super::MetaGenerator;

pub trait TournamentMetaGenerator {
    fn generate_meta_for(&self, tournament: &TournamentRowModel) -> Map<String, Value>;
}

pub struct TournamentPreviewMetaGenerator<'a> {
    pub users_by_username: HashMap<&'a str, &'a UserRowModel>,
}

pub struct TournamentDetailsMetaGenerator {
    tournament: TournamentRowModel,
    creator: UserRowModel,
}

impl TournamentMetaGenerator for TournamentPreviewMetaGenerator<'_> {
    fn generate_meta_for(&self, tournament: &TournamentRowModel) -> Map<String, Value> {
        let mut meta = Map::new();
        meta.insert(String::from("id"), Value::from(tournament.id.clone()));
        meta.insert(String::from("name"), Value::from(tournament.name.clone()));
        meta.insert(
            String::from("tournament_type"),
            Value::from(tournament.tournament_type.clone()),
        );
        meta.insert(
            String::from("country"),
            Value::from(tournament.country.clone()),
        );
        meta.insert(
            String::from("creator_username"),
            Value::from(tournament.creator.clone()),
        );

        let creator = self.users_by_username.get(tournament.creator.as_str());
        let creator_display_name = match creator {
            Some(user) => user.display_name.clone(),
            None => String::from("Unknown User"),
        };
        meta.insert(
            String::from("creator_display_name"),
            Value::from(creator_display_name.clone()),
        );

        meta.insert(
            String::from("start_date"),
            Value::from(date_to_string(tournament.start_date.clone())),
        );
        meta.insert(
            String::from("end_date"),
            Value::from(date_to_string(tournament.end_date.clone())),
        );
        meta
    }
}

impl TournamentDetailsMetaGenerator {
    pub fn from_tournament(
        tournament: TournamentRowModel,
        creator: UserRowModel,
    ) -> TournamentDetailsMetaGenerator {
        TournamentDetailsMetaGenerator {
            tournament,
            creator,
        }
    }
}

impl MetaGenerator for TournamentDetailsMetaGenerator {
    fn generate_meta(&self) -> Map<String, Value> {
        let mut meta = Map::new();
        meta.insert(String::from("id"), Value::from(self.tournament.id.clone()));
        meta.insert(
            String::from("name"),
            Value::from(self.tournament.name.clone()),
        );
        meta.insert(
            String::from("tournament_type"),
            Value::from(self.tournament.tournament_type.clone()),
        );
        meta.insert(
            String::from("country"),
            Value::from(self.tournament.country.clone()),
        );
        meta.insert(
            String::from("start_date"),
            Value::from(date_to_string(self.tournament.start_date.clone())),
        );
        meta.insert(
            String::from("end_date"),
            Value::from(date_to_string(self.tournament.end_date.clone())),
        );

        let mut creator_meta = Map::new();
        creator_meta.insert(
            String::from("username"),
            Value::from(self.creator.username.clone()),
        );
        creator_meta.insert(
            String::from("display_name"),
            Value::from(self.creator.display_name.clone()),
        );
        meta.insert(String::from("creator"), Value::from(creator_meta));

        meta
    }
}
