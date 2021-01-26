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

        use crate::database_models::{MatchDAO, MatchRowModel};
        use crate::utils;
        use crate::utils::{
            create_mock_match_from_round,
            create_mock_player_from_tournament,
            create_mock_round_from_tournament,
            create_mock_tournament_with_creator,
            create_mock_user,
        };

        #[test]
        fn test_create_match() {
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

            let black_player = create_mock_player_from_tournament(
                &tournament.id,
                &test_connection,
            );
            let black_score = 20;
            let white_player = create_mock_player_from_tournament(
                &tournament.id,
                &test_connection,
            );
            let white_score = 44;

            let result = MatchRowModel::create(
                &round.id,
                &black_player.id,
                &white_player.id,
                &black_score,
                &white_score,
                Map::new(),
                &test_connection,
            );
            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn test_get_all_matches() {
            let test_connection = utils::get_test_connection();

            let user = create_mock_user(&test_connection);
            let tournament = create_mock_tournament_with_creator(
                &user.username,
                &test_connection,
            );
            let round_1 = create_mock_round_from_tournament(
                &tournament.id,
                &test_connection,
            );
            let round_2 = create_mock_round_from_tournament(
                &tournament.id,
                &test_connection,
            );

            let match_1 = create_mock_match_from_round(
                &tournament.id,
                &round_1.id,
                &test_connection,
            );
            let match_2 = create_mock_match_from_round(
                &tournament.id,
                &round_1.id,
                &test_connection,
            );
            let _match_3 = create_mock_match_from_round(
                &tournament.id,
                &round_2.id,
                &test_connection,
            );

            let round_1_matches = MatchRowModel::get_all_from_round(
                &round_1.id,
                &test_connection,
            ).unwrap();
            assert_eq!(round_1_matches, vec![match_1, match_2]);
        }

        #[test]
        fn test_get_match() {
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
            let othello_match = create_mock_match_from_round(
                &tournament.id,
                &round.id,
                &test_connection,
            );

            let match_obtained = MatchRowModel::get(
                &othello_match.id,
                &test_connection,
            ).unwrap();
            assert_eq!(match_obtained, othello_match);
        }

        #[test]
        fn test_update_match() {
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
            let mut othello_match = create_mock_match_from_round(
                &tournament.id,
                &round.id,
                &test_connection,
            );

            let new_white_score = utils::generate_random_number();
            let new_black_score = utils::generate_random_number();
            othello_match.white_score = new_white_score.clone();
            othello_match.black_score = new_black_score.clone();
            othello_match.update(&test_connection).unwrap();

            let match_obtained = MatchRowModel::get(
                &othello_match.id,
                &test_connection,
            ).unwrap();
            assert_eq!(match_obtained.black_score, new_black_score);
            assert_eq!(match_obtained.white_score, new_white_score);
        }

        #[test]
        fn test_delete_match() {
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
            let othello_match = create_mock_match_from_round(
                &tournament.id,
                &round.id,
                &test_connection,
            );
            othello_match.delete(&test_connection).unwrap();
            let matches = MatchRowModel::get_all_from_round(
                &round.id,
                &test_connection,
            ).unwrap();
            assert_eq!(matches, vec![]);
        }
    }
}
