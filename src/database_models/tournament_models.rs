use serde_json::Value;

use crate::schema::tournaments;

use super::UserRowModel;

#[derive(AsChangeset, PartialEq, Debug, Queryable, Associations, Identifiable)]
#[belongs_to(UserRowModel, foreign_key="creator")]
#[table_name = "tournaments"]
pub struct TournamentRowModel {
    pub id: i32,
    pub name: String,
    pub country: String,
    pub creator: String,
    pub joueurs: Value,
    pub meta_data: Value
}
