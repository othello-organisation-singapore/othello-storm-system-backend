use serde_json::Map;
use diesel::PgConnection;

use crate::account::Account;
use crate::database_models::TournamentRowModel;
use crate::joueurs::{Joueurs, JoueursParser};
use crate::properties::TournamentType;

pub struct TournamentCRUDHandler {
    tournament: TournamentRowModel
}

impl TournamentCRUDHandler {
    pub fn create_new_tournament(
        name: &String, country: &String, creator: &Account, tournament_type: TournamentType,
        connection: &PgConnection,
    ) -> Result<(), String> {
        let raw_joueurs = Joueurs::get(3)?;
        let parsed_joueurs = JoueursParser::parse(&raw_joueurs)?;

        TournamentRowModel::create(
            name,
            country,
            &creator.get_username(),
            parsed_joueurs,
            tournament_type,
            Map::new(),
            connection,
        )
    }

    pub fn from_existing(
        id: &i32, connection: &PgConnection,
    ) -> Result<TournamentCRUDHandler, String> {
        let tournament_model = TournamentRowModel::get(id, connection)?;
        Ok(TournamentCRUDHandler { tournament: tournament_model })
    }

    pub fn update(
        &mut self, updated_name: &String, updated_country: &String, connection: &PgConnection,
    ) -> Result<(), String> {
        self.tournament.name = updated_name.clone();
        self.tournament.country = updated_country.clone();
        self.tournament.update(connection)
    }

    pub fn delete(&self, connection: &PgConnection) -> Result<(), String> {
        self.tournament.delete(connection)
    }
}


#[cfg(test)]
mod tests {
    mod crud_handler {
        use diesel::PgConnection;
        use mocktopus::mocking::{Mockable, MockResult};

        use crate::account::Account;
        use crate::database_models::TournamentRowModel;
        use crate::properties::TournamentType;
        use crate::tournament_manager::TournamentCRUDHandler;
        use crate::utils;

        fn create_new_account(
            username: &String, password: &String, connection: &PgConnection,
        ) -> Account {
            let display_name = utils::generate_random_string(30);
            Account::create_new_admin(&username, &display_name, &password, connection).unwrap();
            Account::login_from_password(&username, &password, connection).unwrap()
        }

        #[test]
        fn test_create_tournament_success() {
            let test_connection = utils::get_test_connection();
            let creator_username = utils::generate_random_string(30);
            let creator_password = utils::generate_random_string(30);
            let creator = create_new_account(
                &creator_username, &creator_password, &test_connection,
            );

            let mock_joueurs = String::from(
                "% Liste des joueurs par pays\n\npays = ARG\n\n280016 ACUNA, Ricardo                       %_<1484>\n280045 ALOATTI, Matias                      %_<1072>\n280028 ANANOS, Sergio                      "
            );
            utils::http_get_text.mock_safe(move |_x| MockResult::Return(Ok(mock_joueurs.clone())));
            let name = utils::generate_random_string(20);
            let country = utils::generate_random_string(10);
            let result = TournamentCRUDHandler::create_new_tournament(
                &name, &country, &creator, TournamentType::SwissPairing,
                &test_connection,
            );
            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn test_create_tournament_failed() {
            let test_connection = utils::get_test_connection();
            let creator_username = utils::generate_random_string(30);
            let creator_password = utils::generate_random_string(30);
            let creator = create_new_account(
                &creator_username, &creator_password, &test_connection,
            );

            utils::http_get_text.mock_safe(|_x| MockResult::Return(Err(String::from("err"))));
            let name = utils::generate_random_string(20);
            let country = utils::generate_random_string(10);
            let result = TournamentCRUDHandler::create_new_tournament(
                &name, &country, &creator, TournamentType::SwissPairing,
                &test_connection,
            );
            assert_eq!(result.is_err(), true);
        }

        fn create_new_tournament(connection: &PgConnection) {
            let creator_username = utils::generate_random_string(30);
            let creator_password = utils::generate_random_string(30);
            let creator = create_new_account(
                &creator_username, &creator_password, connection,
            );

            let mock_joueurs = String::from(
                "% Liste des joueurs par pays\n\npays = ARG\n\n280016 ACUNA, Ricardo                       %_<1484>\n280045 ALOATTI, Matias                      %_<1072>\n280028 ANANOS, Sergio                      "
            );
            utils::http_get_text.mock_safe(move |_x| MockResult::Return(Ok(mock_joueurs.clone())));
            let name = utils::generate_random_string(20);
            let country = utils::generate_random_string(10);
            TournamentCRUDHandler::create_new_tournament(
                &name, &country, &creator, TournamentType::SwissPairing, connection,
            ).unwrap();
        }

        #[test]
        fn test_crud_handler_of_existing_tournament() {
            let test_connection = utils::get_test_connection();
            create_new_tournament(&test_connection);

            let all_tournaments = TournamentRowModel::get_all(&test_connection).unwrap();
            let created_tournament = all_tournaments.first().unwrap();
            let tournament_id = created_tournament.id;


            let result = TournamentCRUDHandler::from_existing(
                &tournament_id, &test_connection,
            );
            assert_eq!(result.is_ok(), true);

            let failed_result = TournamentCRUDHandler::from_existing(
                &(tournament_id + 1), &test_connection,
            );
            assert_eq!(failed_result.is_err(), true);
        }

        #[test]
        fn test_update_tournament() {
            let test_connection = utils::get_test_connection();
            create_new_tournament(&test_connection);

            let all_tournaments = TournamentRowModel::get_all(&test_connection).unwrap();
            let created_tournament = all_tournaments.first().unwrap();
            let tournament_id = created_tournament.id;
            let mut crud_handler = TournamentCRUDHandler::from_existing(
                &tournament_id, &test_connection,
            ).unwrap();

            let updated_name = utils::generate_random_string(30);
            let updated_country = String::from("IDN");
            let result = crud_handler.update(&updated_name, &updated_country, &test_connection);
            assert_eq!(result.is_ok(), true);

            let all_tournaments_after_update = TournamentRowModel::get_all(
                &test_connection
            ).unwrap();
            let updated_tournament = all_tournaments_after_update.first().unwrap();
            assert_eq!(updated_tournament.id, tournament_id);
            assert_eq!(updated_tournament.name, updated_name);
            assert_eq!(updated_tournament.country, updated_country);
            assert_eq!(updated_tournament.creator, created_tournament.creator);
        }

        #[test]
        fn test_delete_tournament() {
            let test_connection = utils::get_test_connection();
            create_new_tournament(&test_connection);

            let all_tournaments = TournamentRowModel::get_all(&test_connection).unwrap();
            let created_tournament = all_tournaments.first().unwrap();
            let tournament_id = created_tournament.id;
            let crud_handler = TournamentCRUDHandler::from_existing(
                &tournament_id, &test_connection,
            ).unwrap();

            crud_handler.delete(&test_connection).unwrap();
            let all_tournaments_after_delete = TournamentRowModel::get_all(
                &test_connection
            ).unwrap();
            assert_eq!(all_tournaments_after_delete.is_empty(), true);

            let result = TournamentCRUDHandler::from_existing(
                &tournament_id, &test_connection
            );
            assert_eq!(result.is_err(), true);
        }
    }
}
