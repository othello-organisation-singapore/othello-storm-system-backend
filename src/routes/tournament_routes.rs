use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

use crate::response_commands;
use crate::response_commands::ResponseCommand;
use crate::utils::get_pooled_connection;

use super::Token;

#[get("/")]
pub fn get_tournaments() -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::GetAllTournamentsCommand {}.execute(&connection)
}

#[get("/created_by_me")]
pub fn get_all_created_tournaments(token: Token) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::GetAllCreatedTournamentsCommand { jwt: token.jwt }.execute(&connection)
}

#[get("/managed_by_me")]
pub fn get_all_managed_tournaments(token: Token) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::GetAllManagedTournamentsCommand { jwt: token.jwt }.execute(&connection)
}

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
    start_date: String,
    end_date: String,
}

#[post("/", data = "<request>")]
pub fn create_tournament(
    token: Token,
    request: Json<TournamentCreationRequest>,
) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::CreateTournamentCommand {
        jwt: token.jwt,
        name: request.name.clone(),
        country: request.country.clone(),
        tournament_type: request.tournament_type.clone(),
        start_date: request.start_date.clone(),
        end_date: request.end_date.clone(),
    }
    .execute(&connection)
}

#[derive(Deserialize)]
pub struct TournamentUpdateRequest {
    name: String,
    country: String,
    start_date: String,
    end_date: String,
}

#[patch("/<id>", data = "<request>")]
pub fn update_tournament(
    token: Token,
    id: i32,
    request: Json<TournamentUpdateRequest>,
) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::UpdateTournamentCommand {
        jwt: token.jwt,
        id,
        updated_name: request.name.clone(),
        updated_country: request.country.clone(),
        updated_start_date: request.start_date.clone(),
        updated_end_date: request.end_date.clone(),
    }
    .execute(&connection)
}

#[delete("/<id>")]
pub fn delete_tournament(token: Token, id: i32) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::DeleteTournamentCommand { jwt: token.jwt, id }.execute(&connection)
}

#[get("/<id>/summary")]
pub fn get_tournament_summary(id: i32) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::GetTournamentSummaryCommand { id }.execute(&connection)
}
