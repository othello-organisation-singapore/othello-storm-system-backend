#[derive(Debug, PartialEq)]
pub enum UserRole {
    Superuser,
    Admin,
    Visitor,
}

impl UserRole {
    pub fn from_string(role: String) -> UserRole {
        match role.as_str() {
            "superuser" => UserRole::Superuser,
            "admin" => UserRole::Admin,
            _ => UserRole::Visitor,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            UserRole::Superuser => String::from("superuser"),
            UserRole::Admin => String::from("admin"),
            _ => String::from("visitor"),
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

        #[test]
        fn test_to_string() {
            assert_eq!(UserRole::Superuser.to_string(), String::from("superuser"));
            assert_eq!(UserRole::Admin.to_string(), String::from("admin"));
            assert_eq!(UserRole::Visitor.to_string(), String::from("visitor"));
        }

        #[test]
        fn test_from_and_to_string() {
            assert_eq!(UserRole::from_string(String::from("superuser")).to_string(), String::from("superuser"));
            assert_eq!(UserRole::from_string(String::from("admin")).to_string(), String::from("admin"));
            assert_eq!(UserRole::from_string(String::from("visitor")).to_string(), String::from("visitor"));
            assert_eq!(UserRole::from_string(String::from("junk string")).to_string(), String::from("visitor"));
            assert_eq!(UserRole::from_string(String::from("")).to_string(), String::from("visitor"));
        }
    }
}
