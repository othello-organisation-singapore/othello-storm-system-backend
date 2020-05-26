#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

extern crate mocktopus;

pub mod account;
pub mod database_models;
pub mod utils;

pub mod properties;
pub mod schema;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


fn main() {
    env_logger::init();
    info!("Starting the program");
    // let connection = utils::get_pooled_connection();
    // let username = String::from("ChrisMaxheart");
    // let hashed_password =  String::from("random");
    // let display_name =  String::from("");
    // database_models::User::create(&username, &hashed_password, &display_name, properties::UserRole::Superuser, &connection);
    rocket::ignite().mount("/", routes![index]).launch();
}
