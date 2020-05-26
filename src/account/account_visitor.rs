use super::account::Account;

pub struct VisitorAccount {}

impl Account for VisitorAccount {
    fn has_superuser_access(&self) -> bool {
        false
    }

    fn has_admin_access(&self) -> bool {
        false
    }

    fn get_username(&self) -> String {
        String::from("Standard visitor.")
    }
}

#[cfg(test)]
mod tests {
    mod test_admin_creation {
        use crate::account::account::Account;
        use crate::account::account_visitor::VisitorAccount;
        use crate::utils;

        #[test]
        fn test_visitor_create_admin() {
            let test_connection = utils::get_test_connection();
            let username = utils::generate_random_string(20);
            let display_name = utils::generate_random_string(20);
            let password = utils::generate_random_string(30);
            let hashed_password = utils::hash(&password);

            let account = VisitorAccount{};

            let result = account.create_new_admin(&username, &display_name, &hashed_password, &test_connection);
            assert_eq!(result.is_err(), true);
        }
    }
}
