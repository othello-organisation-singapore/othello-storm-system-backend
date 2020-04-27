#[macro_use(lazy_static)]
extern crate lazy_static;

#[macro_use]
extern crate diesel;
extern crate mocktopus;

pub mod schema;
pub mod database_models;
pub mod utils;
pub mod properties;


fn main() {
    // let services = utils::ExternalServices::create_live_services();
    // let connection = utils::get_pooled_connection();
    // models::User::create_new_admin(String::from("test"), String::from("Test"), String::from("asasdf"), &connection);
    // let users = models::User::get_all_admin_or_higher(&connection);
    // for user in users {
    //     println!("{} {} {}", user.username, user.display_name, user.hashed_password);
    // }
}
