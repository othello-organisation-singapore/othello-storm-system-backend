use diesel::PgConnection;
use serde_json::Map;

use crate::database_models::{
    MatchDAO, MatchRowModel, PlayerRowModel, RoundDAO, RoundRowModel, TournamentRowModel,
    UserRowModel,
};
use crate::properties::{RoundType, TournamentType, UserRole};
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
    )
    .unwrap()
}

pub fn create_mock_tournament_with_creator(
    username: &String,
    connection: &PgConnection,
) -> TournamentRowModel {
    let name = utils::generate_random_string(20);
    let country = utils::generate_random_string(10);
    let joueurs: Vec<Player> = Vec::new();
    let tournament_type = TournamentType::RoundRobin;
    let date = utils::create_date_format(2020, 1, 1);

    TournamentRowModel::create(
        &name,
        &country,
        &date,
        &date,
        &username,
        joueurs,
        tournament_type,
        Map::new(),
        connection,
    )
    .unwrap()
}

pub fn create_mock_tournament_with_creator_and_joueurs(
    creator_username: &String,
    joueurs: Vec<Player>,
    connection: &PgConnection,
) -> TournamentRowModel {
    let name = utils::generate_random_string(20);
    let country = utils::generate_random_string(10);
    let tournament_type = TournamentType::RoundRobin;
    let date = utils::create_date_format(2020, 1, 1);

    TournamentRowModel::create(
        &name,
        &country,
        &date,
        &date,
        &creator_username,
        joueurs,
        tournament_type,
        Map::new(),
        connection,
    )
    .unwrap()
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
    )
    .unwrap()
}

pub fn create_mock_player_from_tournament(
    tournament_id: &i32,
    connection: &PgConnection,
) -> PlayerRowModel {
    let first_name = utils::generate_random_string(5);
    let last_name = utils::generate_random_string(5);
    let country = utils::generate_random_string(3);
    let joueurs_id = utils::generate_random_string(10);
    let rating = utils::generate_random_number();

    let player = Player {
        joueurs_id,
        first_name,
        last_name,
        country,
        rating,
    };
    PlayerRowModel::create(&tournament_id, &player, Map::new(), connection).unwrap()
}

pub fn create_mock_match_from_round(
    tournament_id: &i32,
    round_id: &i32,
    connection: &PgConnection,
) -> MatchRowModel {
    let black_player = create_mock_player_from_tournament(tournament_id, connection);
    let black_score = utils::generate_random_number();

    let white_player = create_mock_player_from_tournament(tournament_id, connection);
    let white_score = utils::generate_random_number();
    MatchRowModel::create(
        &round_id,
        &black_player.id,
        &white_player.id,
        &black_score,
        &white_score,
        Map::new(),
        connection,
    )
    .unwrap()
}
