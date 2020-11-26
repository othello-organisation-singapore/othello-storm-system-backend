use diesel::PgConnection;
use serde_json::{Map, Value};
use rocket::http::Cookies;
use rocket_contrib::json::JsonValue;

use super::ResponseCommand;
use crate::account::Account;
use crate::database_models::{UserRowModel, TournamentRowModel};
use crate::meta_generator::{
    MetaGenerator,
    TournamentPreviewMetaGenerator,
    TournamentDetailsMetaGenerator,
};
use crate::properties::TournamentType;
use crate::joueurs::{Joueurs, JoueursParser};

pub struct GetTournamentCommand {
    pub id: i32,
}

impl ResponseCommand for GetTournamentCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, String> {
        let tournament_model = TournamentRowModel::get(&self.id, connection)?;
        let creator_username = &tournament_model.creator;
        let creator = UserRowModel::get(creator_username, connection)?;
        let meta_generator = TournamentDetailsMetaGenerator::from_tournament(
            tournament_model,
            creator,
        );
        Ok(json!(meta_generator.generate_meta()))
    }
}

pub struct GetAllTournamentsCommand {}

impl ResponseCommand for GetAllTournamentsCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, String> {
        let tournament_models = TournamentRowModel::get_all(connection)?;
        let tournament_meta_list = generate_tournaments_meta(tournament_models);
        Ok(json!({"tournaments": tournament_meta_list}))
    }
}

pub struct GetUserTournamentsCommand<'a> {
    pub cookies: Cookies<'a>,
}

impl ResponseCommand for GetUserTournamentsCommand<'_> {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, String> {
        let account = Account::login_from_cookies(&self.cookies, connection)?;
        let username = account.get_username();
        let tournament_models = TournamentRowModel::get_all_created_by(
            &username, connection,
        )?;

        let tournament_meta_list = generate_tournaments_meta(tournament_models);
        Ok(json!({"tournaments": tournament_meta_list}))
    }
}

fn generate_tournaments_meta(
    tournament_models: Vec<TournamentRowModel>
) -> Vec<Map<String, Value>> {
    tournament_models
        .into_iter()
        .map(|tournament| {
            let meta_generator = TournamentPreviewMetaGenerator::from_tournament(
                tournament
            );
            meta_generator.generate_meta()
        })
        .collect()
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
            connection,
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
        &self, tournament_model: &TournamentRowModel, current_account: &Account,
    ) -> bool {
        let username = current_account.get_username();
        current_account.has_superuser_access() || tournament_model.is_created_by(&username)
    }
}

impl ResponseCommand for UpdateTournamentCommand<'_> {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, String> {
        let account = Account::login_from_cookies(&self.cookies, connection)?;
        let mut tournament_model = TournamentRowModel::get(
            &self.id, connection,
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
        &self, tournament_model: &TournamentRowModel, current_account: &Account,
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
                String::from("You are not authorized to delete this tournament.")
            );
        }

        tournament_model.delete(connection)?;
        Ok(json!({"message": "Tournament deleted."}))
    }
}
