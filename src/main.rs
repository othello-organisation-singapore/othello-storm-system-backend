#[macro_use]
extern crate diesel;
extern crate oss_backend;

pub mod schema;
pub mod models;
use oss_backend::establish_connection;
use diesel::prelude::*;

fn main() {

    let connection = establish_connection();
    let results = schema::users::table.load::<models::User>(&connection).expect("Error");
    println!("{}", results.len());
    for user in results {
        println!("{}", user.username);
        println!("{}", user.display_name);
    }
}
