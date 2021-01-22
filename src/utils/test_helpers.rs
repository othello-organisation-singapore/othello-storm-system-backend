use diesel::PgConnection;
use serde_json::Map;

use crate::database_models::{TournamentRowModel, UserRowModel, RoundDAO, RoundRowModel};
use crate::properties::{TournamentType, UserRole, RoundType};
use crate::tournament_manager::Player;
use crate::utils;

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

pub fn create_mock_tournament_with_creator_and_joueurs(
    creator_username: &String,
    joueurs: Vec<Player>,
    connection: &PgConnection,
) -> TournamentRowModel {
    let name = utils::generate_random_string(20);
    let country = utils::generate_random_string(10);
    let tournament_type = TournamentType::RoundRobin;

    TournamentRowModel::create(
        &name,
        &country,
        &creator_username,
        joueurs,
        tournament_type,
        Map::new(),
        connection,
    ).unwrap()
}

pub fn create_mock_round_from_tournament(
    tournament_id: &i32,
    connection: &PgConnection,
) -> RoundRowModel {
    let name = utils::generate_random_string(10);
    RoundRowModel::create(
        tournament_id,
        &name,
        RoundType::ManualNormal,
        Map::new(),
        connection,
    ).unwrap()
}
