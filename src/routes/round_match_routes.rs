use rocket::http::Cookies;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

use crate::response_commands;
use crate::response_commands::ResponseCommand;
use crate::utils::get_pooled_connection;

#[get("/<id>/rounds")]
pub fn get_tournament_rounds(id: i32) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::GetTournamentRoundsCommand { tournament_id: id }.execute(&connection)
}

#[get("/<tournament_id>/rounds/<round_id>")]
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

//
// #[derive(Deserialize)]
// pub struct TournamentCreationRequest {
//     name: String,
//     country: String,
//     tournament_type: String,
//     start_date: String,
//     end_date: String,
// }
//
// #[post("/", data = "<request>")]
// pub fn create_tournament(
//     cookies: Cookies,
//     request: Json<TournamentCreationRequest>,
// ) -> Json<JsonValue> {
//     let connection = get_pooled_connection();
//     response_commands::CreateTournamentCommand {
//         cookies,
//         name: request.name.clone(),
//         country: request.country.clone(),
//         tournament_type: request.tournament_type.clone(),
//         start_date: request.start_date.clone(),
//         end_date: request.end_date.clone(),
//     }.execute(&connection)
// }
