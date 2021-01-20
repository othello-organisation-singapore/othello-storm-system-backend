use diesel::PgConnection;
use serde_json::{Map, Value};

use crate::account::Account;
use crate::database_models::{PlayerRowModel, TournamentRowModel, UserRowModel};
use crate::errors::ErrorType;
use crate::meta_generator::{
    MetaGenerator,
    PlayerMetaGenerator,
    TournamentPreviewMetaGenerator,
    UserMetaGenerator,
};

pub fn generate_players_meta(player_models: Vec<PlayerRowModel>) -> Vec<Map<String, Value>> {
    player_models
        .into_iter()
        .map(|player| {
            let meta_generator = PlayerMetaGenerator::from_player_model(player);
            meta_generator.generate_meta()
        })
        .collect()
}

pub fn generate_tournaments_meta(
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

pub fn generate_users_meta(user_models: Vec<UserRowModel>) -> Vec<Map<String, Value>> {
    user_models
        .into_iter()
        .map(|user| {
            let meta_generator = UserMetaGenerator::from_user(user);
            meta_generator.generate_meta()
        })
        .collect()
}

pub fn is_allowed_to_manage_tournament(
    account: &Account,
    tournament: &TournamentRowModel,
    connection: &PgConnection,
) -> Result<bool, ErrorType> {
    if account.has_superuser_access() {
        return Ok(true);
    }

    let username = account.get_username();
    let is_created_by_account = tournament.is_created_by(&username);
    let is_managed_by_account = tournament.is_managed_by(&username, connection)?;
    return  Ok(is_created_by_account || is_managed_by_account);
}
