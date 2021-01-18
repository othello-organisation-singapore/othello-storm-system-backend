use diesel::PgConnection;
use rocket::http::Cookies;
use rocket_contrib::json::JsonValue;
use serde_json::{Map, Value};

use crate::account::Account;
use crate::database_models::{PlayerRowModel, TournamentRowModel};
use crate::errors::ErrorType;

use super::{generate_players_meta, is_allowed_to_manage_tournament};
use super::ResponseCommand;

pub struct GetTournamentPlayersCommand {
    pub tournament_id: i32,
}

impl ResponseCommand for GetTournamentPlayersCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let players = PlayerRowModel::get_all_from_tournament(
            &self.tournament_id,
            connection,
        )?;
        let players_meta = generate_players_meta(players);
        Ok(json!({
            "tournament_id": &self.tournament_id,
            "players": players_meta,
        }))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!("GetTournamentPlayers for {}", &self.tournament_id))
    }
}

pub struct GetTournamentJoueursPlayersCommand {
    pub tournament_id: i32,
}

impl ResponseCommand for GetTournamentJoueursPlayersCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let tournament_model = TournamentRowModel::get(
            &self.tournament_id,
            connection,
        )?;
        let joueurs_players = tournament_model.get_players_from_joueurs()?;
        let joueurs_players_meta: Vec<Map<String, Value>> = joueurs_players
            .iter()
            .map(|player| player.to_serdemap())
            .collect();
        Ok(json!({
            "tournament_id": &self.tournament_id,
            "joueurs_players": joueurs_players_meta,
        }))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!("GetTournamentJoueursPlayers for {}", &self.tournament_id))
    }
}

pub struct AddTournamentPlayerCommand<'a> {
    pub cookies: Cookies<'a>,
    pub tournament_id: i32,
    pub joueurs_id: String,
}

impl ResponseCommand for AddTournamentPlayerCommand<'_> {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let account = Account::login_from_cookies(&self.cookies, connection)?;
        let tournament_model = TournamentRowModel::get(
            &self.tournament_id,
            connection,
        )?;

        let is_allowed_to_manage = is_allowed_to_manage_tournament(
            &account,
            &tournament_model,
            connection,
        )?;
        if !is_allowed_to_manage {
            return Err(ErrorType::PermissionDenied);
        }


        let player = tournament_model.get_player_with_joueurs_id(&self.joueurs_id)?;
        PlayerRowModel::create(
            &self.tournament_id,
            &player,
            Map::new(),
            connection,
        )?;
        Ok(json!({"message": "Player added to tournament."}))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!(
            "AddTournamentPlayer with joueurs id {} for tournament id {}",
            &self.joueurs_id,
            &self.tournament_id
        ))
    }
}

pub struct DeleteTournamentPlayerCommand<'a> {
    pub cookies: Cookies<'a>,
    pub tournament_id: i32,
    pub player_id: i32,
}

impl ResponseCommand for DeleteTournamentPlayerCommand<'_> {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let account = Account::login_from_cookies(&self.cookies, connection)?;
        let tournament_model = TournamentRowModel::get(
            &self.tournament_id,
            connection,
        )?;

        let is_allowed_to_manage = is_allowed_to_manage_tournament(
            &account,
            &tournament_model,
            connection,
        )?;
        if !is_allowed_to_manage {
            return Err(ErrorType::PermissionDenied);
        }

        let player_model = PlayerRowModel::get(&self.player_id, connection)?;
        player_model.delete(connection)?;

        Ok(json!({"message": "Player deleted"}))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!(
            "DeleteTournamentPlayer with player id {} for tournament id {}",
            &self.player_id,
            &self.tournament_id,
        ))
    }
}
