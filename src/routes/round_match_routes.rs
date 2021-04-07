use rocket::http::Cookies;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

use crate::response_commands;
use crate::response_commands::ResponseCommand;
use crate::utils::get_pooled_connection;

#[get("/<tournament_id>/rounds")]
pub fn get_tournament_rounds(tournament_id: i32) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::GetTournamentRoundsCommand { tournament_id }.execute(&connection)
}

#[derive(Deserialize)]
pub struct CreateManualNormalRoundRequest {
    name: String,
    match_data: Vec<(i32, i32)>,
    bye_match_data: Vec<i32>,
}

#[post("/<tournament_id>/rounds/create_manual_normal", data = "<request>")]
pub fn create_manual_normal_round(
    cookies: Cookies,
    tournament_id: i32,
    request: Json<CreateManualNormalRoundRequest>,
) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::CreateManualNormalRoundCommand {
        cookies,
        tournament_id,
        name: request.name.clone(),
        match_data: request.match_data.clone(),
        bye_match_data: request.bye_match_data.clone(),
    }
    .execute(&connection)
}

#[derive(Deserialize)]
pub struct CreateManualSpecialRoundRequest {
    name: String,
    match_data: Vec<(i32, i32)>,
    bye_match_data: Vec<i32>,
}

#[post("/<tournament_id>/rounds/create_manual_special", data = "<request>")]
pub fn create_manual_special_round(
    cookies: Cookies,
    tournament_id: i32,
    request: Json<CreateManualSpecialRoundRequest>,
) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::CreateManualSpecialRoundCommand {
        cookies,
        tournament_id,
        name: request.name.clone(),
        match_data: request.match_data.clone(),
        bye_match_data: request.bye_match_data.clone(),
    }
    .execute(&connection)
}

#[derive(Deserialize)]
pub struct CreateAutomaticRoundRequest {
    name: String,
}

#[post("/<tournament_id>/rounds/create_automatic", data = "<request>")]
pub fn create_automatic_round(
    cookies: Cookies,
    tournament_id: i32,
    request: Json<CreateAutomaticRoundRequest>,
) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::CreateAutomaticRoundCommand {
        cookies,
        tournament_id,
        name: request.name.clone(),
    }
    .execute(&connection)
}

#[get("/<_tournament_id>/rounds/<round_id>")]
pub fn get_round(_tournament_id: i32, round_id: i32) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::GetRoundCommand { round_id }.execute(&connection)
}

#[get("/<tournament_id>/rounds/<round_id>/standings")]
pub fn get_standings(tournament_id: i32, round_id: i32) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::GetStandingsCommand {
        round_id_limit: round_id,
        tournament_id,
    }
    .execute(&connection)
}

#[derive(Deserialize)]
pub struct UpdateRoundRequest {
    updated_name: String,
}

#[patch("/<tournament_id>/rounds/<round_id>", data = "<request>")]
pub fn update_round(
    cookies: Cookies,
    tournament_id: i32,
    round_id: i32,
    request: Json<UpdateRoundRequest>,
) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::UpdateRoundCommand {
        cookies,
        tournament_id,
        round_id,
        updated_name: request.updated_name.clone(),
    }
    .execute(&connection)
}

#[delete("/<tournament_id>/rounds/<round_id>")]
pub fn delete_round(cookies: Cookies, tournament_id: i32, round_id: i32) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::DeleteRoundCommand {
        cookies,
        tournament_id,
        round_id,
    }
    .execute(&connection)
}

#[get("/<_tournament_id>/rounds/<round_id>/matches")]
pub fn get_round_matches(_tournament_id: i32, round_id: i32) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::GetRoundMatchesCommand { round_id }.execute(&connection)
}

#[derive(Deserialize)]
pub struct UpdateMatchRequest {
    black_score: i32,
    white_score: i32,
}

#[patch(
    "/<tournament_id>/rounds/<_round_id>/matches/<match_id>",
    data = "<request>"
)]
pub fn update_match(
    cookies: Cookies,
    tournament_id: i32,
    _round_id: i32,
    match_id: i32,
    request: Json<UpdateMatchRequest>,
) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::UpdateMatchCommand {
        cookies,
        tournament_id,
        match_id,
        black_score: request.black_score.clone(),
        white_score: request.white_score.clone(),
    }
    .execute(&connection)
}
