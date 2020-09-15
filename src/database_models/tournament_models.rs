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
        name: &String, country: &String, creator_username: &String, joueurs: Vec<Player>,
        tournament_type: TournamentType, meta_data: Map<String, Value>, connection: &PgConnection,
    ) -> Result<(), String> {
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
    ) -> Result<(), String> {
        let tournament_name = new_tournament.name.clone();
        let result = diesel::insert_into(tournaments::table)
            .values(new_tournament)
            .execute(connection);
        match result {
            Ok(_) => {
                info!("Tournament {} is created.", tournament_name);
                Ok(())
            }
            Err(e) => {
                error!("{}", e);
                Err(String::from("Cannot create new tournament."))
            }
        }
    }

    pub fn get(id: &i32, connection: &PgConnection) -> Result<TournamentRowModel, String> {
        let result = tournaments::table
            .find(id)
            .first(connection);

        match result {
            Ok(tournament) => Ok(tournament),
            Err(e) => {
                error!("{}", e);
                Err(String::from("Cannot get the tournament."))
            }
        }
    }

    pub fn get_all(connection: &PgConnection) -> Result<Vec<TournamentRowModel>, String> {
        let result = tournaments::table.load::<TournamentRowModel>(connection);
        match result {
            Ok(tournaments) => Ok(tournaments),
            Err(e) => {
                error!("{}", e);
                Err(String::from("Cannot obtain all tournaments."))
            }
        }
    }

    pub fn get_all_created_by(
        username: &String, connection: &PgConnection,
    ) -> Result<Vec<TournamentRowModel>, String> {
        let user = UserRowModel::get(username, connection)?;
        let result = TournamentRowModel::belonging_to(&user).load::<TournamentRowModel>(connection);
        match result {
            Ok(tournaments) => Ok(tournaments),
            Err(e) => {
                error!("{}", e);
                Err(String::from("Cannot obtain tournaments from user."))
            }
        }
    }

    pub fn is_created_by(&self, username: &String) -> bool {
        &self.creator == username
    }

    pub fn update(&self, connection: &PgConnection) -> Result<(), String> {
        let result = diesel::update(self)
            .set(self)
            .execute(connection);

        match result {
            Ok(_) => {
                info!("Tournament {} ({}) is updated.", &self.id, &self.name);
                Ok(())
            }
            Err(e) => {
                error!("{}", e);
                Err(String::from("Tournament failed to update."))
            }
        }
    }

    pub fn delete(&self, connection: &PgConnection) -> Result<(), String> {
        let result = diesel::delete(self)
            .execute(connection);

        match result {
            Ok(_) => {
                info!("Tournament {} ({}) is deleted.", &self.id, &self.name);
                Ok(())
            }
            Err(e) => {
                error!("{}", e);
                Err(String::from("Tournament failed to delete."))
            }
        }
    }
}


#[cfg(test)]
mod tests {
    mod crud {
        use serde_json::Map;
        use crate::database_models::{UserRowModel, TournamentRowModel};
        use crate::properties::{UserRole, TournamentType};
        use crate::tournament_manager::Player;
        use crate::utils;
        use diesel::PgConnection;

        fn create_mock_user_with_username(
            username: &String, connection: &PgConnection,
        ) -> UserRowModel {
            let display_name = utils::generate_random_string(20);
            let password = utils::generate_random_string(30);
            let hashed_password = utils::hash(&password);
            let _ = UserRowModel::create(
                username,
                &display_name,
                &hashed_password,
                UserRole::Superuser,
                connection,
            );
            UserRowModel::get(username, connection).unwrap()
        }

        #[test]
        fn test_create_tournament() {
            let test_connection = utils::get_test_connection();

            let creator_username = utils::generate_random_string(20);
            let _ = create_mock_user_with_username(
                &creator_username, &test_connection,
            );

            let name = utils::generate_random_string(20);
            let country = utils::generate_random_string(10);
            let joueurs: Vec<Player> = Vec::new();
            let tournament_type = TournamentType::RoundRobin;

            let result = TournamentRowModel::create(
                &name,
                &country,
                &creator_username,
                joueurs,
                tournament_type,
                Map::new(),
                &test_connection,
            );
            assert_eq!(result.is_ok(), true)
        }

        fn create_mock_tournament_with_creator(username: &String, connection: &PgConnection) {
            let name = utils::generate_random_string(20);
            let country = utils::generate_random_string(10);
            let joueurs: Vec<Player> = Vec::new();
            let tournament_type = TournamentType::RoundRobin;

            let _ = TournamentRowModel::create(
                &name,
                &country,
                &username,
                joueurs,
                tournament_type,
                Map::new(),
                connection,
            );
        }

        #[test]
        fn test_get_tournaments() {
            let test_connection = utils::get_test_connection();

            let creator_username = utils::generate_random_string(20);
            let _ = create_mock_user_with_username(&creator_username, &test_connection);

            let second_creator_username = utils::generate_random_string(20);
            let _ = create_mock_user_with_username(&second_creator_username, &test_connection);

            let _ = create_mock_tournament_with_creator(&creator_username, &test_connection);
            let _ = create_mock_tournament_with_creator(&creator_username, &test_connection);
            let _ = create_mock_tournament_with_creator(&second_creator_username, &test_connection);

            let all_tournaments_result = TournamentRowModel::get_all(
                &test_connection
            );
            assert_eq!(all_tournaments_result.is_ok(), true);
            let all_tournaments = all_tournaments_result.unwrap();
            assert_eq!(all_tournaments.len(), 3);

            let all_first_creator_tournaments_result = TournamentRowModel::get_all_created_by(
                &creator_username, &test_connection,
            );
            assert_eq!(all_first_creator_tournaments_result.is_ok(), true);
            assert_eq!(all_first_creator_tournaments_result.unwrap().len(), 2);

            let first_tournament = all_tournaments.first().unwrap();
            let first_tournament_from_get = TournamentRowModel::get(
                &first_tournament.id, &test_connection,
            ).unwrap();
            assert_eq!(first_tournament.id, first_tournament_from_get.id);
            assert_eq!(first_tournament.name, first_tournament_from_get.name);
            assert_eq!(first_tournament.creator, first_tournament_from_get.creator);
            assert_eq!(first_tournament.is_created_by(&creator_username), true);
        }

        #[test]
        fn test_update() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);
            let _ = create_mock_user_with_username(&username, &test_connection);
            let _ = create_mock_tournament_with_creator(&username, &test_connection);

            let mut all_tournaments = TournamentRowModel::get_all(
                &test_connection
            ).unwrap();
            let mut tournament = all_tournaments.first_mut().unwrap();

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
            let username = utils::generate_random_string(20);
            let _ = create_mock_user_with_username(&username, &test_connection);
            let _ = create_mock_tournament_with_creator(&username, &test_connection);

            let mut all_tournaments = TournamentRowModel::get_all(
                &test_connection
            ).unwrap();
            let tournament = all_tournaments.first_mut().unwrap();

            let _ = tournament.delete(&test_connection);
            let updated_get_result = TournamentRowModel::get(
                &tournament.id, &test_connection,
            );
            assert_eq!(updated_get_result.is_err(), true);

            let updated_all_tournaments = TournamentRowModel::get_all(
                &test_connection
            ).unwrap();
            assert_eq!(updated_all_tournaments.is_empty(), true);
        }
    }
}
