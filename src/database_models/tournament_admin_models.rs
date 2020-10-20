use diesel::prelude::*;

use crate::account::Account;
use crate::properties::UserRole;
use crate::schema::{tournaments_admin, users, tournaments};
use super::{UserRowModel, TournamentRowModel};

#[derive(AsChangeset, PartialEq, Debug, Queryable, Associations, Identifiable)]
#[belongs_to(UserRowModel, foreign_key = "admin_username")]
#[belongs_to(TournamentRowModel, foreign_key = "tournament_id")]
#[table_name = "tournaments_admin"]
pub struct TournamentAdminRowModel {
    pub id: i32,
    pub tournament_id: i32,
    pub admin_username: String,
}

#[derive(Insertable)]
#[table_name = "tournaments_admin"]
struct NewTournamentAdminRowModel<'a> {
    pub tournament_id: &'a i32,
    pub admin_username: &'a String,
}

fn get_all_admin_usernames_of(
    tournament_id: &i32, connection: &PgConnection,
) -> Result<Vec<String>, String> {
    let admin_usernames_query_result = tournaments_admin::table
        .filter(tournaments_admin::tournament_id.eq(tournament_id))
        .select(tournaments_admin::admin_username)
        .distinct()
        .load::<String>(connection);

    match admin_usernames_query_result {
        Ok(usernames) => Ok(usernames),
        Err(e) => {
            error!("{}", e);
            Err(String::from("Cannot get all admin usernames of the tournament."))
        }
    }
}


impl UserRowModel {
    pub fn get_all_admins_of(
        tournament_id: &i32, connection: &PgConnection,
    ) -> Result<Vec<UserRowModel>, String> {
        let admin_usernames = get_all_admin_usernames_of(tournament_id, connection)?;
        let users_query_result = users::table
            .filter(users::role.eq_any(vec![
                UserRole::Superuser.to_string(),
                UserRole::Admin.to_string()]
            ))
            .filter(users::username.eq_any(admin_usernames))
            .load::<UserRowModel>(connection);

        match users_query_result {
            Ok(admins) => Ok(admins),
            Err(e) => {
                error!("{}", e);
                Err(String::from("Cannot get all admins of the tournament."))
            }
        }
    }

    pub fn get_all_potential_admins_of(
        tournament_id: &i32, connection: &PgConnection,
    ) -> Result<Vec<UserRowModel>, String> {
        let admin_usernames = get_all_admin_usernames_of(tournament_id, connection)?;
        let users_query_result = users::table
            .filter(users::role.eq_any(vec![
                UserRole::Superuser.to_string(),
                UserRole::Admin.to_string()]
            ))
            .filter(users::username.ne_all(admin_usernames))
            .load::<UserRowModel>(connection);

        match users_query_result {
            Ok(potential_admins) => Ok(potential_admins),
            Err(e) => {
                error!("{}", e);
                Err(String::from("Cannot get all potential admins of the tournament."))
            }
        }
    }
}

impl TournamentRowModel {
    pub fn get_all_managed_by(
        username: &String, connection: &PgConnection,
    ) -> Result<Vec<TournamentRowModel>, String> {
        let tournament_ids_query_result = tournaments_admin::table
            .filter(tournaments_admin::admin_username.eq(username))
            .select(tournaments_admin::tournament_id)
            .distinct()
            .load::<i32>(connection);

        let tournament_ids = match tournament_ids_query_result {
            Ok(ids) => Ok(ids),
            Err(e) => {
                error!("{}", e);
                Err(String::from("Cannot get all tournament ids of the admin."))
            }
        }?;

        let tournaments_query_result = tournaments::table
            .filter(tournaments::id.eq_any(tournament_ids))
            .load::<TournamentRowModel>(connection);

        match tournaments_query_result {
            Ok(tournaments) => Ok(tournaments),
            Err(e) => {
                error!("{}", e);
                Err(String::from("Cannot get all tournaments of the admin."))
            }
        }
    }

    pub fn add_admin(&self, username: &String, connection: &PgConnection) -> Result<(), String> {
        let added_admin_account = Account::get(username, connection)?;
        if !added_admin_account.has_admin_access() {
            return Err(String::from("The user added does not have admin role or higher."));
        }

        if self.is_managed_by(username, connection)? {
            return Err(String::from("The user is already the admin of this tournament."))
        }

        let new_tournament_admin = NewTournamentAdminRowModel {
            tournament_id: &self.id,
            admin_username: username,
        };
        let result = diesel::insert_into(tournaments_admin::table)
            .values(new_tournament_admin)
            .execute(connection);
        match result {
            Ok(_) => {
                info!("{} is added as admin to tournament {} ({})", username, &self.id, &self.name);
                Ok(())
            }
            Err(e) => {
                error!("{}", e);
                Err(String::from("Cannot add new admin to the tournament."))
            }
        }
    }

    pub fn remove_admin(&self, username: &String, connection: &PgConnection) -> Result<(), String> {
        if !self.is_managed_by(username, connection)? {
            return Err(String::from("The user is not the admin of this tournament."))
        }

        let result = diesel::delete(
            tournaments_admin::table
                .filter(tournaments_admin::admin_username.eq(username))
        )
            .execute(connection);

        match result {
            Ok(_) => {
                info!("Admin {} is removed from tournament {} ({})", username, &self.id, &self.name);
                Ok(())
            }
            Err(e) => {
                error!("{}", e);
                Err(String::from("Cannot delete admin from the tournament."))
            }
        }
    }

    pub fn is_managed_by(
        &self, username: &String, connection: &PgConnection,
    ) -> Result<bool, String> {
        let result = tournaments_admin::table
            .filter(tournaments_admin::tournament_id.eq(&self.id))
            .filter(tournaments_admin::admin_username.eq(username))
            .load::<TournamentAdminRowModel>(connection);

        match result {
            Ok(tournaments_admin) => Ok(!tournaments_admin.is_empty()),
            Err(e) => {
                error!("{}", e);
                Err(String::from("Cannot check whether the tournament is managed by the admin."))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod test_add_admin {
        use crate::utils;

        #[test]
        fn test_add_admin() {
            let test_connection = utils::get_test_connection();
            let user = utils::create_mock_user(&test_connection);
            let tournament = utils::create_mock_tournament_with_creator(
                &user.username, &test_connection
            );

            let result = tournament.add_admin(&user.username, &test_connection);
            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn test_add_existed_admin() {
            let test_connection = utils::get_test_connection();
            let user = utils::create_mock_user(&test_connection);
            let tournament = utils::create_mock_tournament_with_creator(
                &user.username, &test_connection
            );

            let _ = tournament.add_admin(&user.username, &test_connection);
            let result = tournament.add_admin(&user.username, &test_connection);
            assert_eq!(result.is_err(), true);
        }
    }

    mod test_remove_admin {
        use crate::utils;

        #[test]
        fn test_remove_admin() {
            let test_connection = utils::get_test_connection();
            let user = utils::create_mock_user(&test_connection);
            let tournament = utils::create_mock_tournament_with_creator(
                &user.username, &test_connection
            );

            let _ = tournament.add_admin(&user.username, &test_connection);
            let result = tournament.remove_admin(&user.username, &test_connection);
            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn test_remove_not_admin() {
            let test_connection = utils::get_test_connection();
            let user = utils::create_mock_user(&test_connection);

            let tournament = utils::create_mock_tournament_with_creator(
                &user.username, &test_connection
            );

            let result = tournament.remove_admin(&user.username, &test_connection);
            assert_eq!(result.is_err(), true);
        }
    }
}
