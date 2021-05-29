use diesel::dsl::{exists, select};
use diesel::prelude::*;

use crate::account::Account;
use crate::errors::ErrorType;
use crate::properties::UserRole;
use crate::schema::{tournaments, tournaments_admin, users};

use super::{TournamentRowModel, UserRowModel};

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
    pub fn get_all_admins_of(
        tournament_id: &i32,
        connection: &PgConnection,
    ) -> Result<Vec<UserRowModel>, ErrorType> {
        let admin_usernames = get_all_admin_usernames_of(tournament_id, connection)?;
        let users_query_result = users::table
            .filter(users::role.eq_any(vec![
                UserRole::Superuser.to_string(),
                UserRole::Admin.to_string(),
            ]))
            .filter(users::username.eq_any(admin_usernames))
            .load::<UserRowModel>(connection);

        match users_query_result {
            Ok(admins) => Ok(admins),
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }

    pub fn get_all_potential_admins_of(
        tournament_id: &i32,
        connection: &PgConnection,
    ) -> Result<Vec<UserRowModel>, ErrorType> {
        let admin_usernames = get_all_admin_usernames_of(tournament_id, connection)?;
        let users_query_result = users::table
            .filter(users::role.eq_any(vec![
                UserRole::Superuser.to_string(),
                UserRole::Admin.to_string(),
            ]))
            .filter(users::username.ne_all(admin_usernames))
            .load::<UserRowModel>(connection);

        match users_query_result {
            Ok(potential_admins) => Ok(potential_admins),
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }
}

impl TournamentRowModel {
    pub fn get_all_managed_by(
        username: &String,
        connection: &PgConnection,
    ) -> Result<Vec<TournamentRowModel>, ErrorType> {
        let tournament_ids_query_result = tournaments_admin::table
            .filter(tournaments_admin::admin_username.eq(username))
            .select(tournaments_admin::tournament_id)
            .distinct()
            .load::<i32>(connection);

        let tournament_ids = match tournament_ids_query_result {
            Ok(ids) => Ok(ids),
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }?;

        let tournaments_query_result = tournaments::table
            .filter(tournaments::id.eq_any(tournament_ids))
            .or_filter(tournaments::creator.eq(username))
            .load::<TournamentRowModel>(connection);

        match tournaments_query_result {
            Ok(tournaments) => Ok(tournaments),
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }

    pub fn add_admin(&self, username: &String, connection: &PgConnection) -> Result<(), ErrorType> {
        let added_admin_account = Account::get(username, connection)?;
        if !added_admin_account.has_admin_access() {
            return Err(ErrorType::PermissionDenied);
        }

        if self.is_managed_by(username, connection)? {
            return Err(ErrorType::BadRequestError(String::from(
                "The user is already the admin of this tournament.",
            )));
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
                info!(
                    "{} is added as admin to tournament {} ({})",
                    username, &self.id, &self.name
                );
                Ok(())
            }
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }

    pub fn remove_admin(
        &self,
        username: &String,
        connection: &PgConnection,
    ) -> Result<(), ErrorType> {
        if !self.is_managed_by(username, connection)? {
            return Err(ErrorType::BadRequestError(String::from(
                "The user is not the admin of this tournament.",
            )));
        }

        let result = diesel::delete(
            tournaments_admin::table
                .filter(tournaments_admin::admin_username.eq(username))
                .filter(tournaments_admin::tournament_id.eq(self.id)),
        )
        .execute(connection);

        match result {
            Ok(_) => {
                info!(
                    "Admin {} is removed from tournament {} ({})",
                    username, &self.id, &self.name
                );
                Ok(())
            }
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }

    pub fn is_managed_by(
        &self,
        username: &String,
        connection: &PgConnection,
    ) -> Result<bool, ErrorType> {
        let result = select(exists(
            tournaments_admin::table
                .filter(tournaments_admin::tournament_id.eq(&self.id))
                .filter(tournaments_admin::admin_username.eq(username)),
        ))
        .get_result(connection);

        match result {
            Ok(exist) => Ok(exist),
            Err(e) => {
                error!("{}", e);
                Err(ErrorType::DatabaseError)
            }
        }
    }
}

fn get_all_admin_usernames_of(
    tournament_id: &i32,
    connection: &PgConnection,
) -> Result<Vec<String>, ErrorType> {
    let admin_usernames_query_result = tournaments_admin::table
        .filter(tournaments_admin::tournament_id.eq(tournament_id))
        .select(tournaments_admin::admin_username)
        .distinct()
        .load::<String>(connection);

    match admin_usernames_query_result {
        Ok(usernames) => Ok(usernames),
        Err(e) => {
            error!("{}", e);
            Err(ErrorType::DatabaseError)
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
            let tournament =
                utils::create_mock_tournament_with_creator(&user.username, &test_connection);

            let result = tournament.add_admin(&user.username, &test_connection);
            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn test_add_existed_admin() {
            let test_connection = utils::get_test_connection();
            let user = utils::create_mock_user(&test_connection);
            let tournament =
                utils::create_mock_tournament_with_creator(&user.username, &test_connection);

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
            let tournament =
                utils::create_mock_tournament_with_creator(&user.username, &test_connection);

            let _ = tournament.add_admin(&user.username, &test_connection);
            let result = tournament.remove_admin(&user.username, &test_connection);
            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn test_remove_not_admin() {
            let test_connection = utils::get_test_connection();
            let user = utils::create_mock_user(&test_connection);

            let tournament =
                utils::create_mock_tournament_with_creator(&user.username, &test_connection);

            let result = tournament.remove_admin(&user.username, &test_connection);
            assert_eq!(result.is_err(), true);
        }
    }

    mod test_getters {
        use crate::database_models::{TournamentRowModel, UserRowModel};
        use crate::utils;

        #[test]
        fn test_get_all_admins() {
            let test_connection = utils::get_test_connection();
            let user = utils::create_mock_user(&test_connection);
            let user_2 = utils::create_mock_user(&test_connection);
            let user_3 = utils::create_mock_user(&test_connection);
            let user_4 = utils::create_mock_user(&test_connection);
            let user_5 = utils::create_mock_user(&test_connection);

            let tournament =
                utils::create_mock_tournament_with_creator(&user.username, &test_connection);
            let other_tournament =
                utils::create_mock_tournament_with_creator(&user.username, &test_connection);

            tournament
                .add_admin(&user.username, &test_connection)
                .unwrap();
            tournament
                .add_admin(&user_2.username, &test_connection)
                .unwrap();
            tournament
                .add_admin(&user_3.username, &test_connection)
                .unwrap();
            other_tournament
                .add_admin(&user_4.username, &test_connection)
                .unwrap();

            tournament
                .remove_admin(&user_2.username, &test_connection)
                .unwrap();
            let all_admins =
                UserRowModel::get_all_admins_of(&tournament.id, &test_connection).unwrap();

            assert_eq!(all_admins.contains(&user), true);
            assert_eq!(all_admins.contains(&user_2), false);
            assert_eq!(all_admins.contains(&user_3), true);
            assert_eq!(all_admins.contains(&user_4), false);
            assert_eq!(all_admins.contains(&user_5), false);
        }

        #[test]
        fn test_get_all_potential_admins() {
            let test_connection = utils::get_test_connection();
            let user = utils::create_mock_user(&test_connection);
            let user_2 = utils::create_mock_user(&test_connection);
            let user_3 = utils::create_mock_user(&test_connection);
            let user_4 = utils::create_mock_user(&test_connection);
            let user_5 = utils::create_mock_user(&test_connection);

            let tournament =
                utils::create_mock_tournament_with_creator(&user.username, &test_connection);
            let other_tournament =
                utils::create_mock_tournament_with_creator(&user.username, &test_connection);

            tournament
                .add_admin(&user.username, &test_connection)
                .unwrap();
            tournament
                .add_admin(&user_2.username, &test_connection)
                .unwrap();
            tournament
                .add_admin(&user_3.username, &test_connection)
                .unwrap();
            other_tournament
                .add_admin(&user_4.username, &test_connection)
                .unwrap();

            tournament
                .remove_admin(&user_2.username, &test_connection)
                .unwrap();
            let all_potential_admins =
                UserRowModel::get_all_potential_admins_of(&tournament.id, &test_connection)
                    .unwrap();

            assert_eq!(all_potential_admins.contains(&user), false);
            assert_eq!(all_potential_admins.contains(&user_2), true);
            assert_eq!(all_potential_admins.contains(&user_3), false);
            assert_eq!(all_potential_admins.contains(&user_4), true);
            assert_eq!(all_potential_admins.contains(&user_5), true);
        }

        #[test]
        fn test_get_all_tournaments_managed() {
            let test_connection = utils::get_test_connection();
            let other_user = utils::create_mock_user(&test_connection);
            let user = utils::create_mock_user(&test_connection);

            let tournament_1 =
                utils::create_mock_tournament_with_creator(&other_user.username, &test_connection);
            let tournament_2 =
                utils::create_mock_tournament_with_creator(&other_user.username, &test_connection);
            let tournament_3 =
                utils::create_mock_tournament_with_creator(&other_user.username, &test_connection);
            let tournament_4 =
                utils::create_mock_tournament_with_creator(&other_user.username, &test_connection);
            let tournament_5 =
                utils::create_mock_tournament_with_creator(&other_user.username, &test_connection);
            let tournament_6 =
                utils::create_mock_tournament_with_creator(&user.username, &test_connection);

            tournament_1
                .add_admin(&user.username, &test_connection)
                .unwrap();
            tournament_2
                .add_admin(&user.username, &test_connection)
                .unwrap();
            tournament_3
                .add_admin(&user.username, &test_connection)
                .unwrap();
            tournament_4
                .add_admin(&other_user.username, &test_connection)
                .unwrap();

            tournament_2
                .remove_admin(&user.username, &test_connection)
                .unwrap();
            let all_tournaments_managed =
                TournamentRowModel::get_all_managed_by(&user.username, &test_connection).unwrap();

            assert_eq!(all_tournaments_managed.contains(&tournament_1), true);
            assert_eq!(all_tournaments_managed.contains(&tournament_2), false);
            assert_eq!(all_tournaments_managed.contains(&tournament_3), true);
            assert_eq!(all_tournaments_managed.contains(&tournament_4), false);
            assert_eq!(all_tournaments_managed.contains(&tournament_5), false);
            assert_eq!(all_tournaments_managed.contains(&tournament_6), true);
        }
    }

    mod test_is_managed_by {
        use crate::utils;

        #[test]
        fn test_is_managed_by() {
            let test_connection = utils::get_test_connection();
            let user = utils::create_mock_user(&test_connection);
            let user_2 = utils::create_mock_user(&test_connection);
            let user_3 = utils::create_mock_user(&test_connection);
            let user_4 = utils::create_mock_user(&test_connection);
            let user_5 = utils::create_mock_user(&test_connection);

            let tournament =
                utils::create_mock_tournament_with_creator(&user.username, &test_connection);
            let other_tournament =
                utils::create_mock_tournament_with_creator(&user.username, &test_connection);

            tournament
                .add_admin(&user.username, &test_connection)
                .unwrap();
            tournament
                .add_admin(&user_2.username, &test_connection)
                .unwrap();
            tournament
                .add_admin(&user_3.username, &test_connection)
                .unwrap();
            other_tournament
                .add_admin(&user_3.username, &test_connection)
                .unwrap();
            other_tournament
                .add_admin(&user_4.username, &test_connection)
                .unwrap();

            tournament
                .remove_admin(&user_2.username, &test_connection)
                .unwrap();

            assert_eq!(
                tournament
                    .is_managed_by(&user.username, &test_connection)
                    .unwrap(),
                true
            );
            assert_eq!(
                tournament
                    .is_managed_by(&user_2.username, &test_connection)
                    .unwrap(),
                false
            );
            assert_eq!(
                tournament
                    .is_managed_by(&user_3.username, &test_connection)
                    .unwrap(),
                true
            );
            assert_eq!(
                tournament
                    .is_managed_by(&user_4.username, &test_connection)
                    .unwrap(),
                false
            );
            assert_eq!(
                tournament
                    .is_managed_by(&user_5.username, &test_connection)
                    .unwrap(),
                false
            );

            assert_eq!(
                other_tournament
                    .is_managed_by(&user.username, &test_connection)
                    .unwrap(),
                false
            );
            assert_eq!(
                other_tournament
                    .is_managed_by(&user_2.username, &test_connection)
                    .unwrap(),
                false
            );
            assert_eq!(
                other_tournament
                    .is_managed_by(&user_3.username, &test_connection)
                    .unwrap(),
                true
            );
            assert_eq!(
                other_tournament
                    .is_managed_by(&user_4.username, &test_connection)
                    .unwrap(),
                true
            );
            assert_eq!(
                other_tournament
                    .is_managed_by(&user_5.username, &test_connection)
                    .unwrap(),
                false
            );
        }
    }
}
