use diesel::PgConnection;

use crate::account::Account;
use crate::database_models::TournamentRowModel;
use crate::errors::ErrorType;

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
    return Ok(is_created_by_account || is_managed_by_account);
}
