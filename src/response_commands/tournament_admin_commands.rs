use diesel::PgConnection;
use rocket::http::Cookies;
use rocket_contrib::json::JsonValue;

use crate::account::Account;
use crate::database_models::{TournamentRowModel, UserRowModel};
use crate::errors::ErrorType;
use crate::meta_generator::{generate_tournaments_meta, generate_users_meta};

use super::ResponseCommand;

pub struct GetAllAdminsCommand {
    pub tournament_id: i32,
}

impl ResponseCommand for GetAllAdminsCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let user_models = UserRowModel::get_all_admins_of(&self.tournament_id, connection)?;
        let user_meta_list = generate_users_meta(user_models);
        Ok(json!({ "admins": user_meta_list }))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!("GetAllAdmins for {}", &self.tournament_id))
    }
}

pub struct GetPotentialAdminsCommand {
    pub tournament_id: i32,
}

impl ResponseCommand for GetPotentialAdminsCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let user_models =
            UserRowModel::get_all_potential_admins_of(&self.tournament_id, connection)?;
        let user_meta_list = generate_users_meta(user_models);
        Ok(json!({ "potential_admins": user_meta_list }))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!("GetPotentialAdmins for {}", &self.tournament_id))
    }
}

pub struct GetAllManagedTournamentsCommand<'a> {
    pub cookies: Cookies<'a>,
}

impl ResponseCommand for GetAllManagedTournamentsCommand<'_> {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let account = Account::login_from_cookies(&self.cookies, connection)?;
        let username = account.get_username();
        let tournament_models = TournamentRowModel::get_all_managed_by(&username, connection)?;

        let tournament_meta_list = generate_tournaments_meta(tournament_models);
        Ok(json!({ "tournaments": tournament_meta_list }))
    }

    fn get_request_summary(&self) -> String {
        String::from("GetManagedTournaments")
    }
}

pub struct AddAdminCommand<'a> {
    pub cookies: Cookies<'a>,
    pub tournament_id: i32,
    pub admin_username: String,
}

impl ResponseCommand for AddAdminCommand<'_> {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let account = Account::login_from_cookies(&self.cookies, connection)?;
        let tournament_model = TournamentRowModel::get(&self.tournament_id, connection)?;

        if !is_allowed_to_manage_admin(&account, &tournament_model) {
            return Err(ErrorType::PermissionDenied);
        }
        tournament_model.add_admin(&self.admin_username, connection)?;
        Ok(json!({"message": "Admin added."}))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!(
            "AddAdmin {} for tournament id {}",
            &self.admin_username, &self.tournament_id
        ))
    }
}

pub struct RemoveAdminCommand<'a> {
    pub cookies: Cookies<'a>,
    pub tournament_id: i32,
    pub admin_username: String,
}

impl ResponseCommand for RemoveAdminCommand<'_> {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let account = Account::login_from_cookies(&self.cookies, connection)?;
        let tournament_model = TournamentRowModel::get(&self.tournament_id, connection)?;

        if !is_allowed_to_manage_admin(&account, &tournament_model) {
            return Err(ErrorType::PermissionDenied);
        }
        tournament_model.remove_admin(&self.admin_username, connection)?;
        Ok(json!({"message": "Admin removed."}))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!(
            "RemoveAdmin {} for tournament id {}",
            &self.admin_username, &self.tournament_id
        ))
    }
}

fn is_allowed_to_manage_admin(account: &Account, tournament_model: &TournamentRowModel) -> bool {
    let username = account.get_username();
    account.has_superuser_access() || tournament_model.is_created_by(&username)
}
