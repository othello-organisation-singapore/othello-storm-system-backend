#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;
pub mod utils;
pub mod properties;

use utils::establish_connection;
use diesel::prelude::*;

fn main() {

//    let connection = establish_connection();
//    let results = schema::users::table.load::<models::User>(&connection).expect("Error");
//    diesel::insert_into(schema::users::table)
//        .values(models::NewUser{username: "test", display_name: "", hashed_password: "test", role: "" }).get_result::<models::User>(&connection).expect("test");
//    println!("{}", results.len());
//    for user in results {
//        println!("{}", user.id);
//        println!("{}", user.username);
//        println!("{}", user.display_name);
//    }
}
