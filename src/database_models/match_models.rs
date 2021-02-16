use diesel::prelude::*;
use diesel::result::Error;
use serde_json::{Map, Value};

use crate::errors::ErrorType;
use crate::game_match::{GameMatchTransformer, IGameMatch};
use crate::schema::matches;

use super::{RoundDAO, RoundRowModel};

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
    fn create_from(game_match: &Box<dyn IGameMatch>, connection: &PgConnection) -> Result<Self, ErrorType>;
    fn bulk_create_from(
        game_matches: &Vec<Box<dyn IGameMatch>>,
        connection: &PgConnection,
    ) -> Result<Vec<Self>, ErrorType>;
    fn get(id: &i32, connection: &PgConnection) -> Result<Self, ErrorType>;
    fn get_all_from_round(
        round_id: &i32,
        connection: &PgConnection,
    ) -> Result<Vec<Self>, ErrorType>;
    fn get_all_from_tournament(
        tournament_id: &i32,
        connection: &PgConnection,
    ) -> Result<Vec<Self>, ErrorType>;
    fn delete(&self, connection: &PgConnection) -> Result<(), ErrorType>;
    fn update(&self, connection: &PgConnection) -> Result<Self, ErrorType>;
}

impl MatchRowModel {
    fn insert_to_database(
        new_match: NewMatchRowModel,
        connection: &PgConnection,
    ) -> Result<MatchRowModel, ErrorType> {
        let result: Result<MatchRowModel, Error> = diesel::insert_into(matches::table)
            .values(new_match)
            .get_result(connection);

        match result {
            Ok(game_match) => {
                let match_id = game_match.id.clone();
                let round_id = game_match.round_id.clone();
                let black_player_id = game_match.black_player_id.clone();
                let white_player_id = game_match.white_player_id.clone();

                info!(
                    "Match id {} ({} vs {}) is added in round id {}",
                    match_id,
                    black_player_id,
                    white_player_id,
                    round_id,
                );
                Ok(game_match)
            }
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }

    fn bulk_insert_to_database(
        new_matches: Vec<NewMatchRowModel>,
        connection: &PgConnection,
    ) -> Result<Vec<MatchRowModel>, ErrorType> {
        let result: Result<Vec<MatchRowModel>, Error> = diesel::insert_into(matches::table)
            .values(new_matches)
            .get_results(connection);

        match result {
            Ok(matches) => {
                matches[..]
                    .into_iter()
                    .for_each(
                        |game_match| {
                            info!(
                                "Match id {} ({} vs {}) is added in round id {}",
                                game_match.id.clone(),
                                game_match.black_player_id.clone(),
                                game_match.white_player_id.clone(),
                                game_match.round_id.clone(),
                            );
                        }
                    );
                Ok(matches)
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

    fn create_from(
        game_match: &Box<dyn IGameMatch>,
        connection: &PgConnection,
    ) -> Result<Self, ErrorType> {
        let match_data = GameMatchTransformer::transform_to_match_model_data(game_match);

        let new_match = NewMatchRowModel {
            round_id: &match_data.round_id,
            black_player_id: &match_data.black_player_id,
            white_player_id: &match_data.white_player_id,
            black_score: &match_data.black_score,
            white_score: &match_data.white_score,
            meta_data: &match_data.meta_data,
        };
        MatchRowModel::insert_to_database(new_match, connection)
    }

    fn bulk_create_from(
        game_matches: &Vec<Box<dyn IGameMatch>>,
        connection: &PgConnection,
    ) -> Result<Vec<Self>, ErrorType> {
        let new_matches_data: Vec<MatchRowModel> = game_matches
            .into_iter()
            .map(|game_match| GameMatchTransformer::transform_to_match_model_data(game_match))
            .collect();

        let new_matches = new_matches_data
            .iter()
            .map(|match_datum| {
                NewMatchRowModel {
                    round_id: &match_datum.round_id,
                    black_player_id: &match_datum.black_player_id,
                    white_player_id: &match_datum.white_player_id,
                    black_score: &match_datum.black_score,
                    white_score: &match_datum.white_score,
                    meta_data: &match_datum.meta_data,
                }
            })
            .collect();
        MatchRowModel::bulk_insert_to_database(new_matches, connection)
    }

    fn get(id: &i32, connection: &PgConnection) -> Result<Self, ErrorType> {
        let result = matches::table
            .find(id)
            .first(connection);

        match result {
            Ok(game_match) => Ok(game_match),
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

    fn get_all_from_tournament(
        tournament_id: &i32,
        connection: &PgConnection,
    ) -> Result<Vec<Self>, ErrorType> {
        let rounds = RoundRowModel::get_all_from_tournament(tournament_id, connection)?;
        let round_ids: Vec<i32> = rounds.iter().map(|round| round.id.clone()).collect();

        let result = matches::table
            .filter(matches::round_id.eq_any(round_ids))
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
            Ok(game_match) => {
                info!("Match {} is updated.", &self.id);
                Ok(game_match)
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
        use serde_json::{Map, Value};

        use crate::database_models::{MatchDAO, MatchRowModel};
        use crate::game_match::GameMatchTransformer;
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
        fn test_create_match_from_game_match() {
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

            let game_match = GameMatchTransformer::transform_to_game_match(
                &MatchRowModel {
                    id: -1,
                    round_id: round.id.clone(),
                    black_player_id: black_player.id.clone(),
                    white_player_id: white_player.id.clone(),
                    black_score: black_score.clone(),
                    white_score: white_score.clone(),
                    meta_data: Value::from(Map::new()),
                }
            );
            let result = MatchRowModel::create_from(&game_match, &test_connection);
            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn test_create_bulk_match() {
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
            let black_player_1 = create_mock_player_from_tournament(
                &tournament.id,
                &test_connection,
            );
            let black_score_1 = 20;
            let white_player_1 = create_mock_player_from_tournament(
                &tournament.id,
                &test_connection,
            );
            let white_score_1 = 44;

            let black_player_2 = create_mock_player_from_tournament(
                &tournament.id,
                &test_connection,
            );
            let black_score_2 = 20;
            let white_player_2 = create_mock_player_from_tournament(
                &tournament.id,
                &test_connection,
            );
            let white_score_2 = 44;

            let game_match_1 = GameMatchTransformer::transform_to_game_match(
                &MatchRowModel {
                    id: -1,
                    round_id: round.id.clone(),
                    black_player_id: black_player_1.id.clone(),
                    white_player_id: white_player_1.id.clone(),
                    black_score: black_score_1.clone(),
                    white_score: white_score_1.clone(),
                    meta_data: Value::from(Map::new()),
                }
            );
            let game_match_2 = GameMatchTransformer::transform_to_game_match(
                &MatchRowModel {
                    id: -1,
                    round_id: round.id.clone(),
                    black_player_id: black_player_2.id.clone(),
                    white_player_id: white_player_2.id.clone(),
                    black_score: black_score_2.clone(),
                    white_score: white_score_2.clone(),
                    meta_data: Value::from(Map::new()),
                }
            );
            let matches = vec![game_match_1, game_match_2];
            let result = MatchRowModel::bulk_create_from(&matches, &test_connection);
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
        fn test_get_all_tournament_matches() {
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
            let match_3 = create_mock_match_from_round(
                &tournament.id,
                &round_2.id,
                &test_connection,
            );

            let tournament_2 = create_mock_tournament_with_creator(
                &user.username,
                &test_connection,
            );
            let tournament_2_round = create_mock_round_from_tournament(
                &tournament_2.id,
                &test_connection,
            );
            let _tournament_2_match = create_mock_match_from_round(
                &tournament_2.id,
                &tournament_2_round.id,
                &test_connection,
            );

            let tournament_1_matches = MatchRowModel::get_all_from_tournament(
                &tournament.id,
                &test_connection,
            ).unwrap();
            assert_eq!(tournament_1_matches, vec![match_1, match_2, match_3]);
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
            let game_match = create_mock_match_from_round(
                &tournament.id,
                &round.id,
                &test_connection,
            );

            let match_obtained = MatchRowModel::get(
                &game_match.id,
                &test_connection,
            ).unwrap();
            assert_eq!(match_obtained, game_match);
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
            let mut game_match = create_mock_match_from_round(
                &tournament.id,
                &round.id,
                &test_connection,
            );

            let new_white_score = utils::generate_random_number();
            let new_black_score = utils::generate_random_number();
            game_match.white_score = new_white_score.clone();
            game_match.black_score = new_black_score.clone();
            game_match.update(&test_connection).unwrap();

            let match_obtained = MatchRowModel::get(
                &game_match.id,
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
            let game_match = create_mock_match_from_round(
                &tournament.id,
                &round.id,
                &test_connection,
            );
            game_match.delete(&test_connection).unwrap();
            let matches = MatchRowModel::get_all_from_round(
                &round.id,
                &test_connection,
            ).unwrap();
            assert_eq!(matches, vec![]);
        }
    }
}
