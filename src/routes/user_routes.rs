use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

use crate::response_commands;
use crate::response_commands::ResponseCommand;
use crate::utils::get_pooled_connection;

use super::Token;

#[get("/<username>")]
pub fn get_user(username: String) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::GetUserCommand { username }.execute(&connection)
}

#[derive(Deserialize)]
pub struct UserCreationRequest {
    username: String,
    display_name: String,
    password: String,
}

#[post("/", data = "<request>")]
pub fn create_user(token: Token, request: Json<UserCreationRequest>) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::CreateUserCommand {
        jwt: token.jwt,
        username: request.username.clone(),
        display_name: request.display_name.clone(),
        password: request.password.clone(),
    }
    .execute(&connection)
}

#[derive(Deserialize)]
pub struct UserUpdateRequest {
    display_name: Option<String>,
    password: Option<String>,
}

#[patch("/<username>", data = "<request>")]
pub fn update_user(
    token: Token,
    username: String,
    request: Json<UserUpdateRequest>,
) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::UpdateUserCommand {
        jwt: token.jwt,
        username,
        display_name: request.display_name.clone(),
        password: request.password.clone(),
    }
    .execute(&connection)
}
