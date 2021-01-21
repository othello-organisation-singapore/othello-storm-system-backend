use diesel::dsl::{exists, select};
use diesel::prelude::*;
use serde_json::{Map, Value};

use crate::errors::ErrorType;
use crate::properties::RoundType;
use crate::schema::rounds;
use crate::tournament_manager::Player;

use super::TournamentRowModel;

#[derive(AsChangeset, PartialEq, Debug, Queryable, Associations, Identifiable)]
#[belongs_to(TournamentRowModel, foreign_key = "tournament_id")]
#[table_name = "rounds"]
pub struct RoundRowModel {
    pub id: i32,
    pub tournament_id: i32,
    pub name: String,
    pub round_type: i32,
    pub meta_data: Value,
}

#[derive(Insertable)]
#[table_name = "rounds"]
struct NewRoundRowModel<'a> {
    pub tournament_id: &'a i32,
    pub name: &'a String,
    pub round_type: &'a i32,
    pub meta_data: &'a Value,
}

trait RoundDAO {
    fn create(
        tournament_id: &i32,
        name: &String,
        round_type: RoundType,
        meta_data: Map<String, Value>,
        connection: &PgConnection,
    ) -> Result<Self, ErrorType>;

    fn get(id: &i32, connection: &PgConnection) -> Result<Self, ErrorType>;
    fn get_all_from_tournament(
        tournament_id: &i32,
        connection: &PgConnection,
    ) -> Result<Vec<Self>, ErrorType>;
}

impl RoundDAO for RoundRowModel {
    fn create(
        tournament_id: &i32,
        name: &String,
        round_type: RoundType,
        meta_data: Map<String, Value>,
        connection: &PgConnection,
    ) -> Result<Self, ErrorType> {
        unimplemented!()
    }

    fn get(id: &i32, connection: &PgConnection) -> Result<Self, ErrorType> {
        unimplemented!()
    }

    fn get_all_from_tournament(
        tournament_id: &i32,
        connection: &PgConnection,
    ) -> Result<Vec<Self>, ErrorType> {
        unimplemented!()
    }
}
