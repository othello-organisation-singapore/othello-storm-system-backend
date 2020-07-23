use rocket::http::Cookies;
use rocket_contrib::json::{JsonValue, Json};
use serde::Deserialize;

use crate::response_commands;
use crate::response_commands::ResponseCommand;
use crate::utils::get_pooled_connection;

#[derive(Deserialize)]
pub struct UserLoginRequest {
    username: String,
    password: String,
}

#[post("/login", data = "<request>")]
pub fn login(request: Json<UserLoginRequest>) -> Json<JsonValue> {
    let connection = get_pooled_connection();

    response_commands::LoginCommand {
        username: request.username.clone(),
        password: request.password.clone(),
    }.execute(&connection)
}

#[get("/profile")]
pub fn get_current_user_profile(cookies: Cookies) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::CurrentUserCommand { cookies }.execute(&connection)
}