use diesel::dsl::{exists, select};
use diesel::prelude::*;
use serde_json::{Map, Value};

use crate::errors::ErrorType;
use crate::schema::players;
use crate::tournament_manager::Player;

use super::TournamentRowModel;

#[derive(AsChangeset, PartialEq, Debug, Queryable, Associations, Identifiable)]
#[belongs_to(TournamentRowModel, foreign_key = "tournament_id")]
#[table_name = "players"]
pub struct PlayerRowModel {
    pub id: i32,
    pub tournament_id: i32,
    pub joueurs_id: String,
    pub first_name: String,
    pub last_name: String,
    pub country: String,
    pub rating: i32,
    pub meta_data: Value,
}

#[derive(Insertable)]
#[table_name = "players"]
struct NewPlayerRowModel<'a> {
    pub tournament_id: &'a i32,
    pub joueurs_id: &'a String,
    pub first_name: &'a String,
    pub last_name: &'a String,
    pub country: &'a String,
    pub rating: &'a i32,
    pub meta_data: &'a Value,
}

impl PlayerRowModel {
    pub fn get_all_from_tournament(
        tournament_id: &i32,
        connection: &PgConnection,
    ) -> Result<Vec<PlayerRowModel>, ErrorType> {
        let result = players::table
            .filter(players::tournament_id.eq(tournament_id))
            .load::<PlayerRowModel>(connection);
        match result {
            Ok(players) => Ok(players),
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }

    pub fn create(
        tournament_id: &i32,
        player: &Player,
        meta_data: Map<String, Value>,
        connection: &PgConnection,
    ) -> Result<PlayerRowModel, ErrorType> {
        let meta_data_json = Value::from(meta_data);

        if PlayerRowModel::is_player_exists(player, tournament_id, connection)? {
            return Err(
                ErrorType::BadRequestError(String::from("Player exists in the tournament."))
            );
        }

        let new_player = NewPlayerRowModel {
            tournament_id,
            joueurs_id: &player.joueurs_id,
            first_name: &player.first_name,
            last_name: &player.last_name,
            country: &player.country,
            rating: &player.rating,
            meta_data: &meta_data_json,
        };

        PlayerRowModel::insert_to_database(new_player, connection)
    }

    pub fn get(id: &i32, connection: &PgConnection) -> Result<PlayerRowModel, ErrorType> {
        let result = players::table
            .find(id)
            .first(connection);

        match result {
            Ok(player) => Ok(player),
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }

    pub fn get_from_joueurs_id(
        joueurs_id: &String,
        tournament_id: &i32,
        connection: &PgConnection,
    ) -> Result<PlayerRowModel, ErrorType> {
        let result = players::table
            .filter(players::tournament_id.eq(tournament_id))
            .filter(players::joueurs_id.eq(joueurs_id))
            .first(connection);

        match result {
            Ok(player) => Ok(player),
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }

    pub fn delete(&self, connection: &PgConnection) -> Result<(), ErrorType> {
        let result = diesel::delete(self).execute(connection);
        match result {
            Ok(_) => {
                info!(
                    "Player id={} with joueurs_id={} is deleted from tournament {} ",
                    &self.id,
                    &self.joueurs_id,
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

    pub fn to_player(&self) -> Player {
        Player {
            joueurs_id: self.joueurs_id.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            country: self.country.clone(),
            rating: self.rating.clone(),
        }
    }

    fn is_player_exists(
        player: &Player,
        tournament_id: &i32,
        connection: &PgConnection,
    ) -> Result<bool, ErrorType> {
        let result = select(
            exists(
                players::table
                    .filter(players::tournament_id.eq(tournament_id))
                    .filter(players::joueurs_id.eq(&player.joueurs_id))
            )
        ).get_result(connection);

        match result {
            Ok(exist) => Ok(exist),
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }

    fn insert_to_database(
        new_player: NewPlayerRowModel, connection: &PgConnection,
    ) -> Result<PlayerRowModel, ErrorType> {
        let tournament_id = new_player.tournament_id.clone();
        let player_name = format!("{}{}", &new_player.first_name, &new_player.last_name);
        let player_joueurs_id = new_player.joueurs_id.clone();

        let result = diesel::insert_into(players::table)
            .values(new_player)
            .get_result(connection);
        match result {
            Ok(player) => {
                info!(
                    "Player {} with joueurs id {} is added to tournament {}",
                    player_name,
                    player_joueurs_id,
                    tournament_id
                );
                Ok(player)
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

        use crate::database_models::PlayerRowModel;
        use crate::tournament_manager::Player;
        use crate::utils;
        use crate::utils::{create_mock_tournament_with_creator, create_mock_user};

        #[test]
        fn test_create_player() {
            let test_connection = utils::get_test_connection();
            let user = create_mock_user(&test_connection);
            let tournament = create_mock_tournament_with_creator(
                &user.username,
                &test_connection,
            );

            let first_name = utils::generate_random_string(5);
            let last_name = utils::generate_random_string(5);
            let country = utils::generate_random_string(3);
            let joueurs_id = utils::generate_random_string(10);
            let rating = 100;
            let player = Player {
                joueurs_id: joueurs_id.clone(),
                first_name: first_name.clone(),
                last_name: last_name.clone(),
                country: country.clone(),
                rating: rating.clone(),
            };
            let meta_data = Map::new();

            let player = PlayerRowModel::create(
                &tournament.id,
                &player,
                meta_data,
                &test_connection,
            ).unwrap();

            assert_eq!(player.joueurs_id, joueurs_id);
            assert_eq!(player.first_name, first_name);
            assert_eq!(player.last_name, last_name);
            assert_eq!(player.country, country);
            assert_eq!(player.rating, rating);
        }

        #[test]
        fn test_create_existed_player() {
            let test_connection = utils::get_test_connection();
            let user = create_mock_user(&test_connection);
            let tournament = create_mock_tournament_with_creator(
                &user.username,
                &test_connection,
            );

            let first_name = utils::generate_random_string(5);
            let last_name = utils::generate_random_string(5);
            let country = utils::generate_random_string(3);
            let joueurs_id = utils::generate_random_string(10);
            let rating = 100;
            let player = Player {
                joueurs_id: joueurs_id.clone(),
                first_name: first_name.clone(),
                last_name: last_name.clone(),
                country: country.clone(),
                rating: rating.clone(),
            };
            PlayerRowModel::create(
                &tournament.id,
                &player,
                Map::new(),
                &test_connection,
            ).unwrap();

            let result = PlayerRowModel::create(
                &tournament.id,
                &player,
                Map::new(),
                &test_connection,
            );
            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn test_get_all_from_tournament() {
            let test_connection = utils::get_test_connection();
            let user = create_mock_user(&test_connection);
            let tournament = create_mock_tournament_with_creator(
                &user.username,
                &test_connection,
            );

            let player_1 = Player {
                joueurs_id: utils::generate_random_string(10),
                first_name: utils::generate_random_string(5),
                last_name: utils::generate_random_string(5),
                country: utils::generate_random_string(2),
                rating: 100,
            };
            let _player_model_1 = PlayerRowModel::create(
                &tournament.id,
                &player_1,
                Map::new(),
                &test_connection,
            ).unwrap();

            let player_2 = Player {
                joueurs_id: utils::generate_random_string(10),
                first_name: utils::generate_random_string(5),
                last_name: utils::generate_random_string(5),
                country: utils::generate_random_string(2),
                rating: 200,
            };
            let _player_model_2 = PlayerRowModel::create(
                &tournament.id,
                &player_2,
                Map::new(),
                &test_connection,
            );

            let players = PlayerRowModel::get_all_from_tournament(
                &tournament.id,
                &test_connection,
            ).unwrap();
            assert_eq!(players.len(), 2);
        }

        #[test]
        fn test_get() {
            let test_connection = utils::get_test_connection();
            let user = create_mock_user(&test_connection);
            let tournament = create_mock_tournament_with_creator(
                &user.username,
                &test_connection,
            );

            let first_name = utils::generate_random_string(5);
            let last_name = utils::generate_random_string(5);
            let country = utils::generate_random_string(3);
            let joueurs_id = utils::generate_random_string(10);
            let rating = 100;
            let player = Player {
                joueurs_id: joueurs_id.clone(),
                first_name: first_name.clone(),
                last_name: last_name.clone(),
                country: country.clone(),
                rating: rating.clone(),
            };
            let player = PlayerRowModel::create(
                &tournament.id,
                &player,
                Map::new(),
                &test_connection,
            ).unwrap();

            let player_obtained = PlayerRowModel::get(
                &player.id,
                &test_connection,
            ).unwrap();
            assert_eq!(player_obtained.joueurs_id, joueurs_id);
            assert_eq!(player_obtained.first_name, first_name);
            assert_eq!(player_obtained.last_name, last_name);
            assert_eq!(player_obtained.country, country);
            assert_eq!(player_obtained.rating, rating);
        }

        #[test]
        fn test_get_not_available() {
            let test_connection = utils::get_test_connection();
            let mock_id = 1;
            let result = PlayerRowModel::get(&mock_id, &test_connection);
            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn test_get_joueurs() {
            let test_connection = utils::get_test_connection();
            let user = create_mock_user(&test_connection);
            let tournament = create_mock_tournament_with_creator(
                &user.username,
                &test_connection,
            );

            let first_name = utils::generate_random_string(5);
            let last_name = utils::generate_random_string(5);
            let country = utils::generate_random_string(3);
            let joueurs_id = utils::generate_random_string(10);
            let rating = 100;
            let player = Player {
                joueurs_id: joueurs_id.clone(),
                first_name: first_name.clone(),
                last_name: last_name.clone(),
                country: country.clone(),
                rating: rating.clone(),
            };
            let player = PlayerRowModel::create(
                &tournament.id,
                &player,
                Map::new(),
                &test_connection,
            ).unwrap();

            let player_obtained = PlayerRowModel::get_from_joueurs_id(
                &player.joueurs_id,
                &player.tournament_id,
                &test_connection,
            ).unwrap();
            assert_eq!(player_obtained.joueurs_id, joueurs_id);
            assert_eq!(player_obtained.first_name, first_name);
            assert_eq!(player_obtained.last_name, last_name);
            assert_eq!(player_obtained.country, country);
            assert_eq!(player_obtained.rating, rating);
        }

        #[test]
        fn test_get_joueurs_not_available() {
            let test_connection = utils::get_test_connection();
            let mock_joueurs_id = String::from("Mock joueurs id");
            let mock_tournament_id = 1;
            let result = PlayerRowModel::get_from_joueurs_id(
                &mock_joueurs_id,
                &mock_tournament_id,
                &test_connection,
            );
            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn test_delete() {
            let test_connection = utils::get_test_connection();
            let user = create_mock_user(&test_connection);
            let tournament = create_mock_tournament_with_creator(
                &user.username,
                &test_connection,
            );

            let first_name = utils::generate_random_string(5);
            let last_name = utils::generate_random_string(5);
            let country = utils::generate_random_string(3);
            let joueurs_id = utils::generate_random_string(10);
            let rating = 100;
            let player = Player {
                joueurs_id: joueurs_id.clone(),
                first_name: first_name.clone(),
                last_name: last_name.clone(),
                country: country.clone(),
                rating: rating.clone(),
            };
            let player = PlayerRowModel::create(
                &tournament.id,
                &player,
                Map::new(),
                &test_connection,
            ).unwrap();

            let result = player.delete(&test_connection);
            assert_eq!(result.is_ok(), true);

            let get_result = PlayerRowModel::get(&player.id, &test_connection);
            assert_eq!(get_result.is_err(), true);
        }
    }
}
