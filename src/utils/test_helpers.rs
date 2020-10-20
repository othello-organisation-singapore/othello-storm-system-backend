use serde_json::Map;
use crate::database_models::{UserRowModel, TournamentRowModel};
use crate::properties::{UserRole, TournamentType};
use crate::tournament_manager::Player;
use crate::utils;
use diesel::PgConnection;

pub fn create_mock_user(connection: &PgConnection) -> UserRowModel {
    let username = utils::generate_random_string(10);
    let display_name = utils::generate_random_string(20);
    let password = utils::generate_random_string(30);
    let hashed_password = utils::hash(&password);
    UserRowModel::create(
        &username,
        &display_name,
        &hashed_password,
        UserRole::Superuser,
        connection,
    ).unwrap()
}

pub fn create_mock_tournament_with_creator(
    username: &String, connection: &PgConnection,
) -> TournamentRowModel {
    let name = utils::generate_random_string(20);
    let country = utils::generate_random_string(10);
    let joueurs: Vec<Player> = Vec::new();
    let tournament_type = TournamentType::RoundRobin;

    TournamentRowModel::create(
        &name,
        &country,
        &username,
        joueurs,
        tournament_type,
        Map::new(),
        connection,
    ).unwrap()
}
