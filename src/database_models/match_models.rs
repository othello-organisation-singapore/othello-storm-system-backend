use diesel::prelude::*;
use diesel::result::Error;
use serde_json::{Map, Value};

use crate::errors::ErrorType;
use crate::schema::matches;

use super::RoundRowModel;

#[derive(AsChangeset, PartialEq, Debug, Queryable, Associations, Identifiable)]
#[belongs_to(RoundRowModel, foreign_key = "round_id")]
#[table_name = "matches"]
pub struct MatchRowModel {
    pub id: i32,
    pub round_id: i32,
    pub black_player_id: i32,
    pub white_player_id: i32,
    pub black_score: i32,
    pub white_score: i32,
    pub meta_data: Value,
}

#[derive(Insertable)]
#[table_name = "matches"]
struct NewMatchRowModel<'a> {
    pub round_id: &'a i32,
    pub black_player_id: &'a i32,
    pub white_player_id: &'a i32,
    pub black_score: &'a i32,
    pub white_score: &'a i32,
    pub meta_data: &'a Value,
}

pub trait MatchDAO where Self: Sized {
    fn create(
        round_id: &i32,
        black_player_id: &i32,
        white_player_id: &i32,
        black_score: &i32,
        white_score: &i32,
        meta_data: Map<String, Value>,
        connection: &PgConnection,
    ) -> Result<Self, ErrorType>;
    fn get(id: &i32, connection: &PgConnection) -> Result<Self, ErrorType>;
    fn get_all_from_round(
        round_id: &i32,
        connection: &PgConnection,
    ) -> Result<Vec<Self>, ErrorType>;
    fn delete(&self, connection: &PgConnection) -> Result<(), ErrorType>;
    fn update(&self, connection: &PgConnection) -> Result<Self, ErrorType>;
}

impl MatchRowModel {
    fn insert_to_database(
        new_round: NewMatchRowModel,
        connection: &PgConnection,
    ) -> Result<MatchRowModel, ErrorType> {
        let result: Result<MatchRowModel, Error> = diesel::insert_into(matches::table)
            .values(new_round)
            .get_result(connection);

        match result {
            Ok(othello_match) => {
                let match_id = othello_match.id.clone();
                let round_id = othello_match.round_id.clone();
                let black_player_id = othello_match.black_player_id.clone();
                let white_player_id = othello_match.white_player_id.clone();

                info!(
                    "Match id {} ({} vs {}) is added in round id {}",
                    match_id,
                    black_player_id,
                    white_player_id,
                    round_id,
                );
                Ok(othello_match)
            }
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }
}

impl MatchDAO for MatchRowModel {
    fn create(
        round_id: &i32,
        black_player_id: &i32,
        white_player_id: &i32,
        black_score: &i32,
        white_score: &i32,
        meta_data: Map<String, Value>,
        connection: &PgConnection,
    ) -> Result<Self, ErrorType> {
        let meta_data_json = Value::from(meta_data);
        let new_match = NewMatchRowModel {
            round_id,
            black_player_id,
            white_player_id,
            black_score,
            white_score,
            meta_data: &meta_data_json,
        };
        MatchRowModel::insert_to_database(new_match, connection)
    }

    fn get(id: &i32, connection: &PgConnection) -> Result<Self, ErrorType> {
        let result = matches::table
            .find(id)
            .first(connection);

        match result {
            Ok(othello_match) => Ok(othello_match),
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }

    fn get_all_from_round(
        round_id: &i32,
        connection: &PgConnection,
    ) -> Result<Vec<Self>, ErrorType> {
        let result = matches::table
            .filter(matches::round_id.eq(round_id))
            .load::<MatchRowModel>(connection);

        match result {
            Ok(matches) => Ok(matches),
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }

    fn delete(&self, connection: &PgConnection) -> Result<(), ErrorType> {
        let result = diesel::delete(self).execute(connection);
        match result {
            Ok(_) => {
                info!(
                    "Match id {} ({} vs {}) is deleted from round id {}",
                    &self.id,
                    &self.black_player_id,
                    &self.white_player_id,
                    &self.round_id,
                );
                Ok(())
            }
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }

    fn update(&self, connection: &PgConnection) -> Result<Self, ErrorType> {
        let result = diesel::update(self)
            .set(self)
            .get_result::<MatchRowModel>(connection);
        match result {
            Ok(othello_match) => {
                info!("Match {} is updated.", &self.id);
                Ok(othello_match)
            },
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }
}
