#[derive(Debug, PartialEq)]
pub enum UserRole {
    Superuser,
    Admin,
    Visitor,
}

impl UserRole {
    pub fn from_string(role: String) -> UserRole {
        match &(role[..]) {
            "superuser" => UserRole::Superuser,
            "admin" => UserRole::Admin,
            _ => UserRole::Visitor,
        }
    }
}

#[cfg(test)]
mod tests {
    mod test_user_role {
        use crate::properties::UserRole;

        #[test]
        fn test_from_string() {
            assert_eq!(UserRole::from_string(String::from("superuser")), UserRole::Superuser);
            assert_eq!(UserRole::from_string(String::from("admin")), UserRole::Admin);
            assert_eq!(UserRole::from_string(String::from("visitor")), UserRole::Visitor);
            assert_eq!(UserRole::from_string(String::from("")), UserRole::Visitor);
            assert_eq!(UserRole::from_string(String::from("random junk")), UserRole::Visitor);
        }
    }
}
