use diesel::prelude::*;
use diesel::expression::dsl::any;

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

impl UserRowModel {
    pub fn get_all_admin_of(
        tournament_id: &i32, connection: &PgConnection,
    ) -> Result<Vec<UserRowModel>, String> {
        let admin_usernames_query_result = tournaments_admin::table
            .filter(tournaments_admin::tournament_id.eq(tournament_id))
            .select(tournaments_admin::admin_username)
            .distinct()
            .load::<String>(connection);

        let admin_usernames = match admin_usernames_query_result {
            Ok(usernames) => Ok(usernames),
            Err(e) => {
                error!("{}", e);
                Err(String::from("Cannot get all admin usernames of the tournament."))
            }
        }?;

        let users_query_result = users::table
            .filter(users::username.eq(any(admin_usernames)))
            .load::<UserRowModel>(connection);

        match users_query_result {
            Ok(admins) => Ok(admins),
            Err(e) => {
                error!("{}", e);
                Err(String::from("Cannot get all admins of the tournament."))
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
            .filter(tournaments::id.eq(any(tournament_ids)))
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
