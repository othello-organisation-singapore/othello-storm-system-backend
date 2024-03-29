use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

use crate::response_commands;
use crate::response_commands::ResponseCommand;
use crate::utils::get_pooled_connection;

use super::Token;

#[get("/<id>/players")]
pub fn get_players(id: i32) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::GetTournamentPlayersCommand { tournament_id: id }.execute(&connection)
}

#[get("/<id>/joueurs_players")]
pub fn get_joueurs_players(id: i32) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::GetTournamentJoueursPlayersCommand { tournament_id: id }.execute(&connection)
}

#[derive(Deserialize)]
pub struct AddPlayerRequest {
    pub joueurs_id: String,
}

#[post("/<id>/players", data = "<request>")]
pub fn add_player(token: Token, id: i32, request: Json<AddPlayerRequest>) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    let command = response_commands::AddTournamentPlayerCommand {
        jwt: token.jwt,
        tournament_id: id,
        joueurs_id: request.joueurs_id.clone(),
    };
    command.execute(&connection)
}

#[derive(Deserialize)]
pub struct AddPlayerNewRequest {
    pub first_name: String,
    pub last_name: String,
    pub country: String,
}

#[post("/<id>/players/new", data = "<request>")]
pub fn add_player_new(
    token: Token,
    id: i32,
    request: Json<AddPlayerNewRequest>,
) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    let command = response_commands::AddTournamentPlayerNewCommand {
        jwt: token.jwt,
        tournament_id: id,
        first_name: request.first_name.clone(),
        last_name: request.last_name.clone().to_uppercase(),
        country: request.country.clone(),
    };
    command.execute(&connection)
}

#[delete("/<tournament_id>/players/<player_id>")]
pub fn delete_player(token: Token, tournament_id: i32, player_id: i32) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    let command = response_commands::DeleteTournamentPlayerCommand {
        jwt: token.jwt,
        tournament_id,
        player_id,
    };
    command.execute(&connection)
}
