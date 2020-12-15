use rocket::http::Cookies;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

use crate::response_commands;
use crate::response_commands::ResponseCommand;
use crate::utils::get_pooled_connection;

#[get("/")]
pub fn get_tournaments() -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::GetAllTournamentsCommand {}.execute(&connection)
}

#[get("/created_by_me")]
pub fn get_all_created_tournaments(cookies: Cookies) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::GetAllCreatedTournamentsCommand { cookies }.execute(&connection)
}

#[get("/managed_by_me")]
pub fn get_all_managed_tournaments(cookies: Cookies) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    response_commands::GetAllManagedTournamentsCommand { cookies }.execute(&connection)
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
pub fn add_admin(cookies: Cookies, id: i32, request: Json<AddAdminRequest>) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    let command = response_commands::AddAdminCommand {
        cookies,
        tournament_id: id,
        admin_username: request.username.clone(),
    };
    command.execute(&connection)
}

#[delete("/<id>/admins/<username>")]
pub fn remove_admin(cookies: Cookies, id: i32, username: String) -> Json<JsonValue> {
    let connection = get_pooled_connection();
    let command = response_commands::RemoveAdminCommand {
        cookies,
        tournament_id: id,
        admin_username: username,
    };
    command.execute(&connection)
}
