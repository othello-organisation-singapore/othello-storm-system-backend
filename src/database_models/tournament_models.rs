use serde_json::{Value, Map};
use diesel::prelude::*;

use crate::schema::tournaments;
use crate::properties::TournamentType;
use crate::tournament_manager::Player;

use super::UserRowModel;

#[derive(AsChangeset, PartialEq, Debug, Queryable, Associations, Identifiable)]
#[belongs_to(UserRowModel, foreign_key = "creator")]
#[table_name = "tournaments"]
pub struct TournamentRowModel {
    pub id: i32,
    pub name: String,
    tournament_type: String,
    pub country: String,
    pub creator: String,
    pub joueurs: Value,
    pub meta_data: Value,
}


#[derive(Insertable)]
#[table_name = "tournaments"]
struct NewTournamentRowModel<'a> {
    pub name: &'a String,
    pub tournament_type: &'a String,
    pub country: &'a String,
    pub creator: &'a String,
    pub joueurs: &'a Value,
    pub meta_data: &'a Value,
}


impl TournamentRowModel {
    pub fn create(
        name: &String, country: &String, creator_username: &String, joueurs: Vec<Player>,
        tournament_type: TournamentType, connection: &PgConnection,
    ) -> Result<(), String> {
        let joueurs_to_store = Value::Array(joueurs
            .iter()
            .map(|x| Value::Object(x.to_serdemap()))
            .collect()
        );

        let new_tournament = NewTournamentRowModel {
            name,
            tournament_type: &tournament_type.to_string(),
            country,
            creator: creator_username,
            joueurs: &joueurs_to_store,
            meta_data: &Value::Object(Map::new())
        };
        TournamentRowModel::insert_to_database(new_tournament, connection)
    }

    fn insert_to_database(
        new_tournament: NewTournamentRowModel, connection: &PgConnection
    ) -> Result<(), String> {
        let tournament_name = new_tournament.name.clone();
        let result = diesel::insert_into(tournaments::table)
            .values(new_tournament)
            .execute(connection);
        match result {
            Ok(_) => {
                info!("Tournament {} is created", tournament_name);
                Ok(())
            },
            Err(e) => {
                error!("{}", e);
                Err(String::from("Cannot create new tournament."))
            },
        }
    }
}
