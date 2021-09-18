use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

use crate::response_commands;
use crate::response_commands::ResponseCommand;
use crate::utils::get_pooled_connection;

use super::Token;

#[get("/<id>/admins")]
pub fn get_tournament_admins(id: i32) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::GetAllAdminsCommand { tournament_id: id }.execute(&connection)
}

#[get("/<id>/potential_admins")]
pub fn get_tournament_potential_admins(id: i32) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::GetPotentialAdminsCommand { tournament_id: id }.execute(&connection)
}

#[derive(Deserialize)]
pub struct AddAdminRequest {
    username: String,
}

#[post("/<id>/admins", data = "<request>")]
pub fn add_admin(token: Token, id: i32, request: Json<AddAdminRequest>) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    let command = response_commands::AddAdminCommand {
        jwt: token.jwt,
        tournament_id: id,
        admin_username: request.username.clone(),
    };
    command.execute(&connection)
}

#[delete("/<id>/admins/<username>")]
pub fn remove_admin(token: Token, id: i32, username: String) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    let command = response_commands::RemoveAdminCommand {
        jwt: token.jwt,
        tournament_id: id,
        admin_username: username,
    };
    command.execute(&connection)
}
