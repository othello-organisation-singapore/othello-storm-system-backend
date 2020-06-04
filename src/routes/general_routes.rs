use rocket::http::Cookies;
use rocket_contrib::json::{JsonValue, Json};
use serde::Deserialize;

use super::response::create_response;
use super::super::account::AccountFactory;
use super::super::utils::get_pooled_connection;
use rocket::response::Redirect;

#[derive(Deserialize)]
pub struct UserLoginRequest {
    username: String,
    password: String,
}

#[post("/login", data = "<request>")]
pub fn login(request: Json<UserLoginRequest>) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    let account = match AccountFactory::login_from_password(
        &request.username, &request.password, &connection
    ) {
        Ok(account) => account,
        Err(message) => return create_response(Err(message))
    };

    info!("{} is logged in.", account.get_username());
    match account.generate_jwt() {
        Ok(jwt) => create_response(Ok(json!({"jwt": jwt}))),
        Err(message) => create_response(Err(message))
    }
}

#[get("/profile")]
pub fn get_current_user_profile(cookies: Cookies) -> Redirect {
    let connection = get_pooled_connection();
    let username = match AccountFactory::login_from_cookies(cookies, &connection) {
        Ok(account) => account.get_username(),
        Err(message) => message
    };
    Redirect::to(uri!(super::user_routes::get_user: username))
}