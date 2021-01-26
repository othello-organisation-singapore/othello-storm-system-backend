use diesel::prelude::*;
use diesel::result::Error;
use serde_json::{Map, Value};

use crate::errors::ErrorType;
use crate::properties::RoundType;
use crate::schema::rounds;

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

pub trait RoundDAO where Self: Sized {
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
    fn delete(&self, connection: &PgConnection) -> Result<(), ErrorType>;
}

impl RoundRowModel {
    fn insert_to_database(
        new_round: NewRoundRowModel,
        connection: &PgConnection,
    ) -> Result<RoundRowModel, ErrorType> {
        let result: Result<RoundRowModel, Error> = diesel::insert_into(rounds::table)
            .values(new_round)
            .get_result(connection);

        match result {
            Ok(round) => {
                let round_id = round.id.clone();
                let round_name = round.name.clone();
                let tournament_id = round.tournament_id.clone();

                info!(
                    "Round id {} (round {}) is added to tournament {}",
                    round_id,
                    round_name,
                    tournament_id,
                );
                Ok(round)
            }
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }
}

impl RoundDAO for RoundRowModel {
    fn create(
        tournament_id: &i32,
        name: &String,
        round_type: RoundType,
        meta_data: Map<String, Value>,
        connection: &PgConnection,
    ) -> Result<Self, ErrorType> {
        let meta_data_json = Value::from(meta_data);
        let new_round = NewRoundRowModel {
            tournament_id,
            name,
            round_type: &round_type.to_i32(),
            meta_data: &meta_data_json,
        };
        RoundRowModel::insert_to_database(new_round, connection)
    }

    fn get(id: &i32, connection: &PgConnection) -> Result<Self, ErrorType> {
        let result = rounds::table
            .find(id)
            .first(connection);

        match result {
            Ok(round) => Ok(round),
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }

    fn get_all_from_tournament(
        tournament_id: &i32,
        connection: &PgConnection,
    ) -> Result<Vec<Self>, ErrorType> {
        let result = rounds::table
            .filter(rounds::tournament_id.eq(tournament_id))
            .load::<RoundRowModel>(connection);

        match result {
            Ok(rounds) => Ok(rounds),
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
                    "Round id {} (round {}) is deleted from tournament {}",
                    &self.id,
                    &self.name,
                    &self.tournament_id
                );
                Ok(())
            }
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    mod crud {
        use serde_json::Map;

        use crate::database_models::{RoundDAO, RoundRowModel};
        use crate::properties::RoundType;
        use crate::utils;
        use crate::utils::{
            create_mock_round_from_tournament,
            create_mock_tournament_with_creator,
            create_mock_user,
        };

        #[test]
        fn test_create_round() {
            let test_connection = utils::get_test_connection();

            let user = create_mock_user(&test_connection);
            let tournament = create_mock_tournament_with_creator(
                &user.username,
                &test_connection,
            );

            let name = utils::generate_random_string(10);

            let new_round = RoundRowModel::create(
                &tournament.id,
                &name,
                RoundType::ManualNormal,
                Map::new(),
                &test_connection,
            );

            assert_eq!(new_round.is_ok(), true);
        }

        #[test]
        fn test_get_all_rounds() {
            let test_connection = utils::get_test_connection();

            let user = create_mock_user(&test_connection);
            let tournament_1 = create_mock_tournament_with_creator(
                &user.username,
                &test_connection,
            );
            let tournament_2 = create_mock_tournament_with_creator(
                &user.username,
                &test_connection,
            );

            let round_1 = create_mock_round_from_tournament(
                &tournament_1.id,
                &test_connection,
            );
            let round_2 = create_mock_round_from_tournament(
                &tournament_1.id,
                &test_connection,
            );
            let _round_3 = create_mock_round_from_tournament(
                &tournament_2.id,
                &test_connection,
            );

            let tournament_1_rounds = RoundRowModel::get_all_from_tournament(
                &tournament_1.id,
                &test_connection,
            ).unwrap();
            assert_eq!(tournament_1_rounds, vec![round_1, round_2]);
        }

        #[test]
        fn test_get_round() {
            let test_connection = utils::get_test_connection();
            let user = create_mock_user(&test_connection);
            let tournament = create_mock_tournament_with_creator(
                &user.username,
                &test_connection,
            );
            let round = create_mock_round_from_tournament(
                &tournament.id,
                &test_connection,
            );

            let round_obtained = RoundRowModel::get(
                &round.id,
                &test_connection,
            ).unwrap();
            assert_eq!(round_obtained, round);
        }

        #[test]
        fn test_delete_round() {
            let test_connection = utils::get_test_connection();
            let user = create_mock_user(&test_connection);
            let tournament = create_mock_tournament_with_creator(
                &user.username,
                &test_connection,
            );
            let round = create_mock_round_from_tournament(
                &tournament.id,
                &test_connection,
            );
            round.delete(&test_connection).unwrap();

            let tournament_rounds = RoundRowModel::get_all_from_tournament(
                &tournament.id,
                &test_connection,
            ).unwrap();
            assert_eq!(tournament_rounds, vec![]);
        }
    }
}
