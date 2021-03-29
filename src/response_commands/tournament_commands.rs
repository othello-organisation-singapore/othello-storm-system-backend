use diesel::PgConnection;
use rocket::http::Cookies;
use rocket_contrib::json::JsonValue;
use serde_json::Map;

use crate::account::Account;
use crate::database_models::{TournamentRowModel, UserRowModel};
use crate::errors::ErrorType;
use crate::joueurs::{Joueurs, JoueursParser};
use crate::meta_generator::{MetaGenerator, TournamentDetailsMetaGenerator};
use crate::properties::TournamentType;
use crate::utils::string_to_date;

use super::generate_tournaments_meta;
use super::ResponseCommand;

pub struct GetTournamentCommand {
    pub id: i32,
}

impl ResponseCommand for GetTournamentCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let tournament_model = TournamentRowModel::get(&self.id, connection)?;
        let creator_username = &tournament_model.creator;
        let creator = UserRowModel::get(creator_username, connection)?;
        let meta_generator =
            TournamentDetailsMetaGenerator::from_tournament(tournament_model, creator);
        Ok(json!(meta_generator.generate_meta()))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!("GetTournament for {}", &self.id))
    }
}

pub struct GetAllTournamentsCommand {}

impl ResponseCommand for GetAllTournamentsCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let tournament_models = TournamentRowModel::get_all(connection)?;
        let tournament_meta_list = generate_tournaments_meta(tournament_models);
        Ok(json!({ "tournaments": tournament_meta_list }))
    }

    fn get_request_summary(&self) -> String {
        String::from("GetAllTournaments")
    }
}

pub struct GetAllCreatedTournamentsCommand<'a> {
    pub cookies: Cookies<'a>,
}

impl ResponseCommand for GetAllCreatedTournamentsCommand<'_> {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let account = Account::login_from_cookies(&self.cookies, connection)?;
        let username = account.get_username();
        let tournament_models = TournamentRowModel::get_all_created_by(&username, connection)?;

        let tournament_meta_list = generate_tournaments_meta(tournament_models);
        Ok(json!({ "tournaments": tournament_meta_list }))
    }

    fn get_request_summary(&self) -> String {
        String::from("GetCreatedTournaments")
    }
}

pub struct CreateTournamentCommand<'a> {
    pub cookies: Cookies<'a>,
    pub name: String,
    pub country: String,
    pub tournament_type: String,
    pub start_date: String,
    pub end_date: String,
}

impl ResponseCommand for CreateTournamentCommand<'_> {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let account = Account::login_from_cookies(&self.cookies, connection)?;

        if !account.has_admin_access() {
            return Err(ErrorType::PermissionDenied);
        }

        let tournament_type = TournamentType::from_string(self.tournament_type.clone());

        let raw_joueurs = Joueurs::get(3)?;
        let parsed_joueurs = JoueursParser::parse(&raw_joueurs)?;

        let start_date = string_to_date(self.start_date.clone())?;
        let end_date = string_to_date(self.end_date.clone())?;
        TournamentRowModel::create(
            &self.name,
            &self.country,
            &start_date,
            &end_date,
            &account.get_username(),
            parsed_joueurs,
            tournament_type,
            Map::new(),
            connection,
        )?;
        Ok(json!({"message": "Tournament created."}))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!("CreateTournament for {}", &self.name))
    }
}

pub struct UpdateTournamentCommand<'a> {
    pub cookies: Cookies<'a>,
    pub id: i32,
    pub updated_name: String,
    pub updated_country: String,
    pub updated_start_date: String,
    pub updated_end_date: String,
}

impl UpdateTournamentCommand<'_> {
    fn is_able_to_update_tournament(
        &self,
        tournament_model: &TournamentRowModel,
        current_account: &Account,
    ) -> bool {
        let username = current_account.get_username();
        current_account.has_superuser_access() || tournament_model.is_created_by(&username)
    }
}

impl ResponseCommand for UpdateTournamentCommand<'_> {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let account = Account::login_from_cookies(&self.cookies, connection)?;
        let mut tournament_model = TournamentRowModel::get(&self.id, connection)?;

        if !self.is_able_to_update_tournament(&tournament_model, &account) {
            return Err(ErrorType::PermissionDenied);
        }

        tournament_model.name = self.updated_name.clone();
        tournament_model.country = self.updated_country.clone();
        tournament_model.start_date = string_to_date(self.updated_start_date.clone())?;
        tournament_model.end_date = string_to_date(self.updated_end_date.clone())?;
        tournament_model.update(connection)?;
        Ok(json!({"message": "Tournament updated."}))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!("UpdateTournament for {}", &self.id))
    }
}

pub struct DeleteTournamentCommand<'a> {
    pub cookies: Cookies<'a>,
    pub id: i32,
}

impl DeleteTournamentCommand<'_> {
    fn is_able_to_delete_tournament(
        &self,
        tournament_model: &TournamentRowModel,
        current_account: &Account,
    ) -> bool {
        let username = current_account.get_username();
        current_account.has_superuser_access() || tournament_model.is_created_by(&username)
    }
}

impl ResponseCommand for DeleteTournamentCommand<'_> {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let account = Account::login_from_cookies(&self.cookies, connection)?;
        let tournament_model = TournamentRowModel::get(&self.id, connection)?;

        if !self.is_able_to_delete_tournament(&tournament_model, &account) {
            return Err(ErrorType::PermissionDenied);
        }

        tournament_model.delete(connection)?;
        Ok(json!({"message": "Tournament deleted."}))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!("DeleteTournament for {}", &self.id))
    }
}
