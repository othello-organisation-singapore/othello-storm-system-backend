use rocket::http::Cookies;
use rocket_contrib::json::{JsonValue, Json};
use serde::Deserialize;

use crate::response_commands;
use crate::response_commands::ResponseCommand;
use crate::utils::get_pooled_connection;

#[get("/<id>")]
pub fn get_tournament(id: i32) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::GetTournamentCommand { id }.execute(&connection)
}

#[derive(Deserialize)]
pub struct TournamentCreationRequest {
    name: String,
    country: String,
    tournament_type: String,
}

#[post("/", data = "<request>")]
pub fn create_tournament(
    cookies: Cookies, request: Json<TournamentCreationRequest>,
) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::CreateTournamentCommand {
        cookies,
        name: request.name.clone(),
        country: request.country.clone(),
        tournament_type: request.tournament_type.clone(),
    }.execute(&connection)
}

#[derive(Deserialize)]
pub struct TournamentUpdateRequest {
    name: String,
    country: String,
}

#[patch("/<id>", data = "<request>")]
pub fn update_tournament(
    cookies: Cookies, id: i32, request: Json<TournamentUpdateRequest>,
) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::UpdateTournamentCommand {
        cookies,
        id,
        updated_name: request.name.clone(),
        updated_country: request.country.clone(),
    }.execute(&connection)
}

#[delete("/<id>")]
pub fn delete_tournament(cookies: Cookies, id: i32) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::DeleteTournamentCommand {
        cookies,
        id,
    }.execute(&connection)
}
