use diesel::PgConnection;
use rocket_contrib::json::JsonValue;
use serde_json::{Map, Value};

use crate::account::Account;
use crate::database_models::{PlayerRowModel, RoundDAO, RoundRowModel, TournamentRowModel};
use crate::errors::ErrorType;
use crate::meta_generator::generate_players_meta;
use crate::tournament_manager::Player;
use crate::utils::generate_random_string;

use super::{is_allowed_to_manage_tournament, ResponseCommand};

pub struct GetTournamentPlayersCommand {
    pub tournament_id: i32,
}

impl ResponseCommand for GetTournamentPlayersCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let players = PlayerRowModel::get_all_from_tournament(&self.tournament_id, connection)?;
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
        let tournament_model = TournamentRowModel::get(&self.tournament_id, connection)?;
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
        String::from(format!(
            "GetTournamentJoueursPlayers for {}",
            &self.tournament_id
        ))
    }
}

pub struct AddTournamentPlayerCommand {
    pub jwt: String,
    pub tournament_id: i32,
    pub joueurs_id: String,
}

impl ResponseCommand for AddTournamentPlayerCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let account = Account::login_from_jwt(&self.jwt, connection)?;
        let tournament_model = TournamentRowModel::get(&self.tournament_id, connection)?;

        let is_allowed_to_manage =
            is_allowed_to_manage_tournament(&account, &tournament_model, connection)?;
        if !is_allowed_to_manage {
            return Err(ErrorType::PermissionDenied);
        }

        let tournament_rounds =
            RoundRowModel::get_all_from_tournament(&self.tournament_id, connection)?;
        if tournament_rounds.len() > 0 {
            return Err(ErrorType::BadRequestError(String::from(
                "You cannot add a player in an ongoing tournament",
            )));
        }

        let player = tournament_model.get_player_with_joueurs_id(&self.joueurs_id)?;
        PlayerRowModel::create(&self.tournament_id, &player, Map::new(), connection)?;
        Ok(json!({"message": "Player added to tournament."}))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!(
            "AddTournamentPlayer with joueurs id {} for tournament id {}",
            &self.joueurs_id, &self.tournament_id
        ))
    }
}

pub struct AddTournamentPlayerNewCommand {
    pub jwt: String,
    pub tournament_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub country: String,
}

impl AddTournamentPlayerNewCommand {
    fn try_create_player(&self, connection: &PgConnection, retry: i32) -> Result<(), ErrorType> {
        let joueurs_id = String::from("+") + &generate_random_string(4);
        let player = Player {
            joueurs_id,
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            country: self.country.clone(),
            rating: 1200,
        };
        match PlayerRowModel::create(&self.tournament_id, &player, Map::new(), connection) {
            Ok(_player_model) => Ok(()),
            Err(error) => {
                if retry == 0 {
                    return Err(error);
                }
                self.try_create_player(connection, retry - 1)
            }
        }
    }
}

impl ResponseCommand for AddTournamentPlayerNewCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let account = Account::login_from_jwt(&self.jwt, connection)?;
        let tournament_model = TournamentRowModel::get(&self.tournament_id, connection)?;

        let is_allowed_to_manage =
            is_allowed_to_manage_tournament(&account, &tournament_model, connection)?;
        if !is_allowed_to_manage {
            return Err(ErrorType::PermissionDenied);
        }

        let tournament_rounds =
            RoundRowModel::get_all_from_tournament(&self.tournament_id, connection)?;
        if tournament_rounds.len() > 0 {
            return Err(ErrorType::BadRequestError(String::from(
                "You cannot add a player in an ongoing tournament",
            )));
        }

        self.try_create_player(connection, 3)?;
        Ok(json!({"message": "Player (new) added to tournament"}))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!(
            "AddTournamentPlayerNew \
            with first_name {}, last_name {}, country {} for tournament id {}",
            &self.first_name, &self.last_name, &self.country, &self.tournament_id
        ))
    }
}

pub struct DeleteTournamentPlayerCommand {
    pub jwt: String,
    pub tournament_id: i32,
    pub player_id: i32,
}

impl ResponseCommand for DeleteTournamentPlayerCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let account = Account::login_from_jwt(&self.jwt, connection)?;
        let tournament_model = TournamentRowModel::get(&self.tournament_id, connection)?;

        let is_allowed_to_manage =
            is_allowed_to_manage_tournament(&account, &tournament_model, connection)?;
        if !is_allowed_to_manage {
            return Err(ErrorType::PermissionDenied);
        }

        let tournament_rounds =
            RoundRowModel::get_all_from_tournament(&self.tournament_id, connection)?;
        if tournament_rounds.len() > 0 {
            return Err(ErrorType::BadRequestError(String::from(
                "You cannot delete a player in an ongoing tournament",
            )));
        }

        let player_model = PlayerRowModel::get(&self.player_id, connection)?;
        player_model.delete(connection)?;

        Ok(json!({"message": "Player deleted"}))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!(
            "DeleteTournamentPlayer with player id {} for tournament id {}",
            &self.player_id, &self.tournament_id,
        ))
    }
}
