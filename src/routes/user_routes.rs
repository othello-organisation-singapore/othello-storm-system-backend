use rocket::http::Cookies;
use rocket_contrib::json::{JsonValue, Json};
use serde::Deserialize;

use crate::response_commands;
use crate::response_commands::ResponseCommand;
use crate::utils::get_pooled_connection;

#[derive(Deserialize)]
pub struct UserCreationRequest {
    username: String,
    display_name: String,
    password: String,
}

#[derive(Deserialize)]
pub struct UserUpdateRequest {
    display_name: Option<String>,
    password: Option<String>,
}

#[get("/<username>")]
pub fn get_user(username: String) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::GetUserCommand { username }.execute(&connection)
}

#[post("/", data = "<request>")]
pub fn create_user(cookies: Cookies, request: Json<UserCreationRequest>) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::CreateUserCommand {
        cookies,
        username: request.username.clone(),
        display_name: request.display_name.clone(),
        password: request.password.clone(),
    }.execute(&connection)
}

#[patch("/<username>", data = "<request>")]
pub fn update_user(cookies: Cookies, username: String, request: Json<UserUpdateRequest>)
                   -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::UpdateUserCommand {
        cookies,
        username,
        display_name: request.display_name.clone(),
        password: request.password.clone(),
    }.execute(&connection)
}
