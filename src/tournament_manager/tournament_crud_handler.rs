use serde_json::{Value, Map};
use diesel::PgConnection;

use crate::account::Account;
use crate::database_models::TournamentRowModel;
use crate::joueurs::{Joueurs, JoueursParser};
use crate::properties::TournamentType;

pub struct TournamentCRUDHandler {
    tournament: TournamentRowModel
}

impl TournamentCRUDHandler {
    pub fn create_new_tournament(
        name: &String, country: &String, creator: &Account, tournament_type: TournamentType,
        connection: &PgConnection,
    ) -> Result<(), String> {
        let raw_joueurs = Joueurs::get(3)?;
        let parsed_joueurs = JoueursParser::parse(&raw_joueurs)?;

        TournamentRowModel::create(
            name,
            country,
            &creator.get_username(),
            parsed_joueurs,
            tournament_type,
            Map::new(),
            connection,
        )
    }

    pub fn create_from_existing(
        id: &i32, name: &String, connection: &PgConnection,
    ) -> Result<TournamentCRUDHandler, String> {
        let tournament_model = TournamentRowModel::get(id, connection)?;
        if &tournament_model.name != name {
            return Err(String::from("Mismatched tournament details."));
        }
        Ok(TournamentCRUDHandler { tournament: tournament_model })
    }

    pub fn update(
        &mut self, updated_name: &String, updated_country: &String, connection: &PgConnection,
    ) -> Result<(), String> {
        self.tournament.name = updated_name.clone();
        self.tournament.country = updated_country.clone();
        self.tournament.update(connection)
    }

    pub fn delete(&self, connection: &PgConnection) -> Result<(), String> {
        self.tournament.delete(connection)
    }
}
