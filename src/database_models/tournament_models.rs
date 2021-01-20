use diesel::prelude::*;
use serde_json::{Map, Value};

use crate::errors::ErrorType;
use crate::properties::TournamentType;
use crate::schema::tournaments;
use crate::tournament_manager::Player;

use super::UserRowModel;

#[derive(AsChangeset, PartialEq, Debug, Queryable, Associations, Identifiable)]
#[belongs_to(UserRowModel, foreign_key = "creator")]
#[table_name = "tournaments"]
pub struct TournamentRowModel {
    pub id: i32,
    pub name: String,
    pub tournament_type: String,
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
        name: &String,
        country: &String,
        creator_username: &String,
        joueurs: Vec<Player>,
        tournament_type: TournamentType,
        meta_data: Map<String, Value>,
        connection: &PgConnection,
    ) -> Result<TournamentRowModel, ErrorType> {
        let joueurs_to_store = Value::Array(
            joueurs
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
            meta_data: &Value::Object(meta_data),
        };
        TournamentRowModel::insert_to_database(new_tournament, connection)
    }

    fn insert_to_database(
        new_tournament: NewTournamentRowModel, connection: &PgConnection,
    ) -> Result<TournamentRowModel, ErrorType> {
        let tournament_name = new_tournament.name.clone();
        let result = diesel::insert_into(tournaments::table)
            .values(new_tournament)
            .get_result(connection);
        match result {
            Ok(tournament) => {
                info!("Tournament {} is created.", tournament_name);
                Ok(tournament)
            }
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }

    pub fn get(id: &i32, connection: &PgConnection) -> Result<TournamentRowModel, ErrorType> {
        let result = tournaments::table
            .find(id)
            .first(connection);

        match result {
            Ok(tournament) => Ok(tournament),
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }

    pub fn get_all(connection: &PgConnection) -> Result<Vec<TournamentRowModel>, ErrorType> {
        let result = tournaments::table.load::<TournamentRowModel>(connection);
        match result {
            Ok(tournaments) => Ok(tournaments),
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }

    pub fn get_all_created_by(
        username: &String, connection: &PgConnection,
    ) -> Result<Vec<TournamentRowModel>, ErrorType> {
        let result = tournaments::table
            .filter(tournaments::creator.eq(username))
            .load::<TournamentRowModel>(connection);
        match result {
            Ok(tournaments) => Ok(tournaments),
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }

    pub fn is_created_by(&self, username: &String) -> bool {
        &self.creator == username
    }

    pub fn update(&self, connection: &PgConnection) -> Result<TournamentRowModel, ErrorType> {
        let result = diesel::update(self)
            .set(self)
            .get_result(connection);

        match result {
            Ok(tournament) => {
                info!("Tournament {} ({}) is updated.", &self.id, &self.name);
                Ok(tournament)
            }
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }

    pub fn delete(&self, connection: &PgConnection) -> Result<(), ErrorType> {
        let result = diesel::delete(self)
            .execute(connection);

        match result {
            Ok(_) => {
                info!("Tournament {} ({}) is deleted.", &self.id, &self.name);
                Ok(())
            }
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }

    pub fn get_player_with_joueurs_id(&self, joueurs_id: &String) -> Result<Player, ErrorType> {
        let players = self.get_players_from_joueurs()?;
        let player = match players
            .iter()
            .find(|&player| &player.joueurs_id == joueurs_id) {
            Some(player) => Ok(player),
            None => Err(ErrorType::BadRequestError(String::from("Invalid joueurs id")))
        }?;
        Ok(Player{
            joueurs_id: player.joueurs_id.clone(),
            first_name: player.first_name.clone(),
            last_name: player.last_name.clone(),
            country: player.country.clone(),
            rating: player.rating.clone(),
        })
    }

    pub fn get_players_from_joueurs(&self) -> Result<Vec<Player>, ErrorType> {
        let joueurs = self.joueurs.as_array().unwrap();
        let players: Vec<Player> = joueurs
            .iter()
            .map(|player_json| {
                let player_data = player_json.as_object().unwrap().clone();
                match Player::from_serdemap(player_data) {
                    Ok(player) => Some(player),
                    Err(_err) => None,
                }
            })
            .filter(|player| player.is_some())
            .map(|player| player.unwrap())
            .collect();
        Ok(players)
    }
}


#[cfg(test)]
mod tests {
    mod crud {
        use serde_json::Map;

        use crate::database_models::TournamentRowModel;
        use crate::properties::TournamentType;
        use crate::tournament_manager::Player;
        use crate::utils;
        use crate::utils::{create_mock_tournament_with_creator, create_mock_user};

        #[test]
        fn test_create_tournament() {
            let test_connection = utils::get_test_connection();

            let user = create_mock_user(&test_connection);

            let name = utils::generate_random_string(20);
            let country = utils::generate_random_string(10);
            let joueurs: Vec<Player> = Vec::new();
            let tournament_type = TournamentType::RoundRobin;

            let result = TournamentRowModel::create(
                &name,
                &country,
                &user.username,
                joueurs,
                tournament_type,
                Map::new(),
                &test_connection,
            );
            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn test_get_tournaments() {
            let test_connection = utils::get_test_connection();
            let initial_tournaments = TournamentRowModel::get_all(
                &test_connection
            ).unwrap();
            let initial_count = initial_tournaments.len();

            let user = create_mock_user(&test_connection);
            let creator_username = user.username.clone();

            let second_user = create_mock_user(&test_connection);
            let second_creator_username = second_user.username.clone();

            let tournament = create_mock_tournament_with_creator(
                &creator_username, &test_connection,
            );
            let _ = create_mock_tournament_with_creator(&creator_username, &test_connection);
            let _ = create_mock_tournament_with_creator(&second_creator_username, &test_connection);

            let all_tournaments_result = TournamentRowModel::get_all(
                &test_connection
            );
            assert_eq!(all_tournaments_result.is_ok(), true);
            let all_tournaments = all_tournaments_result.unwrap();
            assert_eq!(all_tournaments.len() - initial_count, 3);

            let all_first_creator_tournaments_result = TournamentRowModel::get_all_created_by(
                &creator_username, &test_connection,
            );
            assert_eq!(all_first_creator_tournaments_result.is_ok(), true);
            assert_eq!(all_first_creator_tournaments_result.unwrap().len(), 2);

            let tournament_from_get = TournamentRowModel::get(
                &tournament.id, &test_connection,
            ).unwrap();
            assert_eq!(tournament.id, tournament_from_get.id);
            assert_eq!(tournament.name, tournament_from_get.name);
            assert_eq!(tournament.creator, tournament_from_get.creator);
            assert_eq!(tournament.is_created_by(&creator_username), true);
        }

        #[test]
        fn test_update() {
            let test_connection = utils::get_test_connection();
            let user = create_mock_user(&test_connection);
            let mut tournament = create_mock_tournament_with_creator(
                &user.username, &test_connection,
            );

            let updated_name = String::from("new name");
            let updated_country = String::from("SGP");
            tournament.name = updated_name.clone();
            tournament.country = updated_country.clone();
            let _ = tournament.update(&test_connection);

            let updated_tournament = TournamentRowModel::get(
                &tournament.id, &test_connection,
            ).unwrap();
            assert_eq!(updated_tournament.name, updated_name);
            assert_eq!(updated_tournament.country, updated_country);
        }

        #[test]
        fn test_delete() {
            let test_connection = utils::get_test_connection();
            let initial_tournaments = TournamentRowModel::get_all(
                &test_connection
            ).unwrap();
            let initial_count = initial_tournaments.len();

            let user = create_mock_user(&test_connection);
            let tournament = create_mock_tournament_with_creator(
                &user.username, &test_connection,
            );

            let _ = tournament.delete(&test_connection);
            let updated_get_result = TournamentRowModel::get(
                &tournament.id, &test_connection,
            );
            assert_eq!(updated_get_result.is_err(), true);

            let updated_all_tournaments = TournamentRowModel::get_all(
                &test_connection
            ).unwrap();
            assert_eq!(updated_all_tournaments.len() - initial_count, 0);
        }
    }

    mod players {
        use crate::tournament_manager::Player;
        use crate::utils;
        use crate::utils::{create_mock_tournament_with_creator_and_joueurs, create_mock_user};

        #[test]
        fn test_get_player() {
            let test_connection = utils::get_test_connection();
            let user = create_mock_user(&test_connection);

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

            let tournament = create_mock_tournament_with_creator_and_joueurs(
                &user.username,
                vec![player],
                &test_connection,
            );
            let player = tournament.get_player_with_joueurs_id(&joueurs_id).unwrap();

            assert_eq!(player.joueurs_id, joueurs_id);
            assert_eq!(player.first_name, first_name);
            assert_eq!(player.last_name, last_name);
            assert_eq!(player.country, country);
            assert_eq!(player.rating, rating);
        }

        #[test]
        fn test_get_player_not_found() {
            let test_connection = utils::get_test_connection();
            let user = create_mock_user(&test_connection);
            let joueurs_id = utils::generate_random_string(10);

            let tournament = create_mock_tournament_with_creator_and_joueurs(
                &user.username,
                vec![],
                &test_connection,
            );
            let player = tournament.get_player_with_joueurs_id(&joueurs_id);
            assert_eq!(player.is_err(), true);
        }
    }
}
