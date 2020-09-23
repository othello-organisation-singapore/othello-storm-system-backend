use diesel::PgConnection;
use serde_json::Map;
use rocket::http::Cookies;
use rocket_contrib::json::JsonValue;

use super::ResponseCommand;
use crate::account::Account;
use crate::database_models::TournamentRowModel;
use crate::meta_generator::{MetaGenerator, TournamentMetaGenerator};
use crate::properties::TournamentType;
use crate::joueurs::{Joueurs, JoueursParser};

pub struct GetTournamentCommand {
    pub id: i32,
}

impl ResponseCommand for GetTournamentCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, String> {
        let meta_generator = TournamentMetaGenerator::from_tournament_id(
            &self.id, connection
        )?;
        Ok(json!(meta_generator.generate_meta()))
    }
}

pub struct CreateTournamentCommand<'a> {
    pub cookies: Cookies<'a>,
    pub name: String,
    pub country: String,
    pub tournament_type: String,
}

impl ResponseCommand for CreateTournamentCommand<'_> {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, String> {
        let account = Account::login_from_cookies(&self.cookies, connection)?;

        if !account.has_admin_access() {
            return Err(String::from(
                "You are not authorized to create a tournament."
            ));
        }

        let tournament_type = TournamentType::from_string(
            self.tournament_type.clone()
        );

        let raw_joueurs = Joueurs::get(3)?;
        let parsed_joueurs = JoueursParser::parse(&raw_joueurs)?;
        TournamentRowModel::create(
            &self.name,
            &self.country,
            &account.get_username(),
            parsed_joueurs,
            tournament_type,
            Map::new(),
            connection
        )?;
        Ok(json!({"message": "Tournament created."}))
    }
}

pub struct UpdateTournamentCommand<'a> {
    pub cookies: Cookies<'a>,
    pub id: i32,
    pub updated_name: String,
    pub updated_country: String,
}

impl UpdateTournamentCommand<'_> {
    fn is_able_to_update_tournament(
        &self, tournament_model: &TournamentRowModel, current_account: &Account
    ) -> bool {
        let username = current_account.get_username();
        current_account.has_superuser_access() || tournament_model.is_created_by(&username)
    }
}

impl ResponseCommand for UpdateTournamentCommand<'_> {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, String> {
        let account = Account::login_from_cookies(&self.cookies, connection)?;
        let mut tournament_model = TournamentRowModel::get(
            &self.id, connection
        )?;

        if !self.is_able_to_update_tournament(&tournament_model, &account) {
            return Err(
                String::from("You are not authorized to edit this tournament.")
            );
        }

        tournament_model.name = self.updated_name.clone();
        tournament_model.country = self.updated_country.clone();
        tournament_model.update(connection)?;
        Ok(json!({"message": "Tournament updated."}))
    }
}

pub struct DeleteTournamentCommand<'a> {
    pub cookies: Cookies<'a>,
    pub id: i32,
}

impl DeleteTournamentCommand<'_> {
    fn is_able_to_delete_tournament(
        &self, tournament_model: &TournamentRowModel, current_account: &Account
    ) -> bool {
        let username = current_account.get_username();
        current_account.has_superuser_access() || tournament_model.is_created_by(&username)
    }
}

impl ResponseCommand for DeleteTournamentCommand<'_> {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, String> {
        let account = Account::login_from_cookies(&self.cookies, connection)?;
        let tournament_model = TournamentRowModel::get(&self.id, connection)?;

        if !self.is_able_to_delete_tournament(&tournament_model, &account) {
            return Err(
                String::from("You are not authorized to edit this tournament.")
            );
        }

        tournament_model.delete(connection)?;
        Ok(json!({"message": "Tournament deleted."}))
    }
}
