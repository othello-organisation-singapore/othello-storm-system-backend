use rocket::http::Cookies;
use rocket_contrib::json::{JsonValue, Json};
use serde::Deserialize;

use super::response::create_response;
use super::super::account::Account;
use super::super::utils::{get_pooled_connection, hash};

#[derive(Deserialize)]
pub struct UserCreationRequest {
    username: String,
    display_name: String,
    password: String,
}

#[derive(Deserialize)]
pub struct UserUpdateRequest {
    display_name: Option<String>,
    password: Option<String>
}

#[get("/<username>")]
pub fn get_user(username: String) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    match Account::get(&username, &connection) {
        Ok(account) => create_response(Ok(json!(account.generate_meta()))),
        Err(message) => create_response(Err(message))
    }
}

#[post("/", data = "<request>")]
pub fn create_user(cookies: Cookies, request: Json<UserCreationRequest>) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    let account = match Account::login_from_cookies(cookies, &connection) {
        Ok(account) => account,
        Err(message) => return create_response(Err(message))
    };

    if !account.has_superuser_access() {
        return create_response(Err(String::from(
            "You are not authorized to create an admin account."))
        )
    }
    let hashed_password = hash(&request.password);
    match account.create_new_admin(
        &request.username, &request.display_name, &hashed_password, &connection
    ) {
        Ok(()) => create_response(Ok(json!({"message": "User created"}))),
        Err(message) => create_response(Err(message))
    }
}

#[patch("/<username>", data = "<request>")]
pub fn update_user(cookies: Cookies, username: String, request: Json<UserUpdateRequest>)
    -> Json<JsonValue> {
    let connection = get_pooled_connection();
    let mut account = match Account::login_from_cookies(cookies, &connection) {
        Ok(account) => account,
        Err(message) => return create_response(Err(message))
    };

    if !(account.has_superuser_access() || account.get_username() == username) {
        return create_response(Err(
            String::from("You are not authorized to change other people profile."))
        )
    }

    let display_name = match &request.display_name {
        Some(name) => Option::from(name),
        None => Option::None
    };
    let password = match &request.password {
        Some(password) => Option::from(password),
        None => Option::None
    };

    match account.update(display_name, password, &connection) {
        Ok(()) => create_response(Ok(json!({"message": "User updated"}))),
        Err(message) => create_response(Err(message))
    }

}
