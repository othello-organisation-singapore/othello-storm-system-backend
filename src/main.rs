#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate mocktopus;
extern crate reqwest;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde_json;

use std::env;
use std::str::FromStr;

use rocket_cors::{AllowedOrigins, CorsOptions};

pub mod account;
pub mod database_models;
pub mod errors;
pub mod game_match;
pub mod joueurs;
pub mod meta_generator;
pub mod pairings_generator;
pub mod properties;
pub mod response_commands;
pub mod routes;
pub mod schema;
pub mod tournament_manager;
pub mod utils;

fn create_default_superuser() {
    let connection = utils::get_pooled_connection();
    let username = env::var("SUPERUSER_ID").unwrap();
    let display_name = env::var("SUPERUSER_DISPLAY_NAME").unwrap();
    let password = env::var("SUPERUSER_PASS").unwrap();
    let hashed_password = utils::hash(&password);
    let _ = database_models::UserRowModel::create(
        &username,
        &display_name,
        &hashed_password,
        properties::UserRole::Superuser,
        &connection,
    );
}

fn main() {
    env_logger::init();
    info!("Starting the program");
    create_default_superuser();

    let allowed_methods = ["Get", "Post", "Patch", "Delete"]
        .iter()
        .map(|s| FromStr::from_str(s).unwrap())
        .collect();

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::some_exact(&[
            env::var("FRONTEND_URL").unwrap()
        ]))
        .allowed_methods(allowed_methods)
        .allow_credentials(true);

    rocket::ignite()
        .attach(cors.to_cors().unwrap())
        .mount(
            "/api/users",
            routes![
                routes::user_routes::get_user,
                routes::user_routes::create_user,
                routes::user_routes::update_user,
            ],
        )
        .mount(
            "/api/tournaments",
            routes![
                routes::tournament_routes::get_tournaments,
                routes::tournament_routes::get_all_created_tournaments,
                routes::tournament_routes::get_all_managed_tournaments,
                routes::tournament_routes::get_tournament,
                routes::tournament_routes::create_tournament,
                routes::tournament_routes::update_tournament,
                routes::tournament_routes::delete_tournament,
                routes::tournament_routes::get_tournament_summary,
                routes::tournament_admin_routes::get_tournament_admins,
                routes::tournament_admin_routes::get_tournament_potential_admins,
                routes::tournament_admin_routes::add_admin,
                routes::tournament_admin_routes::remove_admin,
                routes::player_routes::get_players,
                routes::player_routes::get_joueurs_players,
                routes::player_routes::add_player,
                routes::player_routes::add_player_new,
                routes::player_routes::delete_player,
                routes::round_match_routes::get_tournament_rounds,
                routes::round_match_routes::create_manual_normal_round,
                routes::round_match_routes::create_manual_special_round,
                routes::round_match_routes::create_automatic_round,
                routes::round_match_routes::get_round,
                routes::round_match_routes::get_standings,
                routes::round_match_routes::update_round,
                routes::round_match_routes::delete_round,
                routes::round_match_routes::get_round_matches,
                routes::round_match_routes::update_match,
            ],
        )
        .mount(
            "/api/",
            routes![
                routes::general_routes::login,
                routes::general_routes::get_current_user_profile,
            ],
        )
        .launch();
}
