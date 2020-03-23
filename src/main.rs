#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;
pub mod utils;
pub mod permissions;
pub mod properties;


fn main() {
//    let connection = establish_connection();
//    let results = schema::users::table.load::<models::UserRowWrapper>(&connection).expect("Error");
//    diesel::insert_into(schema::users::table)
//        .values(models::NewUserRowWrapper{username: "test", display_name: "", hashed_password: "test", role: "" }).get_result::<models::UserRowWrapper>(&connection).expect("test");
//    println!("{}", results.len());
//    for user in results {
//        println!("{}", user.id);
//        println!("{}", user.username);
//        println!("{}", user.display_name);
//    }
}
