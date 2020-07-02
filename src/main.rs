#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

extern crate mocktopus;
extern crate reqwest;

use std::env;

pub mod account;
pub mod database_models;
pub mod properties;
pub mod schema;
pub mod routes;
pub mod response_commands;
pub mod utils;

fn create_default_superuser() {
    let connection = utils::get_pooled_connection();
    let username = env::var("SUPERUSER_ID").unwrap();
    let display_name = env::var("SUPERUSER_DISPLAY_NAME").unwrap();
    let password = env::var("SUPERUSER_PASS").unwrap();
    let hashed_password = utils::hash(&password);
    let _ = database_models::User::create(
        &username, &display_name, &hashed_password, properties::UserRole::Superuser, &connection,
    );
}

fn main() {
    env_logger::init();
    info!("Starting the program");
    create_default_superuser();

    // let body = reqwest::blocking::get("https://www.worldothello.org/files/joueurs.txt").unwrap().text().unwrap();
    //
    // let mut test = body.split("\n");
    // for elem in test {
    //     println!("{}", elem);
    // }

    rocket::ignite()
        .mount("/user", routes![
            routes::user_routes::get_user,
            routes::user_routes::create_user,
            routes::user_routes::update_user,
        ])
        .mount("/", routes![
            routes::general_routes::login,
            routes::general_routes::get_current_user_profile,
        ])
        .launch();
}
