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

#[derive(Debug, PartialEq)]
pub enum TournamentType {
    RoundRobin,
    SwissPairing,
    Unidentified,
}

impl TournamentType {
    pub fn from_string(tournament_type: String) -> TournamentType {
        match tournament_type.as_str() {
            "round_robin" => TournamentType::RoundRobin,
            "swiss_pairing" => TournamentType::SwissPairing,
            _ => TournamentType::Unidentified,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            TournamentType::RoundRobin => String::from("round_robin"),
            TournamentType::SwissPairing => String::from("swiss_pairing"),
            _ => String::from("unidentified"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RoundType {
    Unidentified,
    Automatic,
    ManualNormal,
    ManualSpecial,
}

impl RoundType {
    pub fn from_i32(round_type: i32) -> RoundType {
        match round_type {
            1 => RoundType::Automatic,
            2 => RoundType::ManualNormal,
            3 => RoundType::ManualSpecial,
            _ => RoundType::Unidentified,
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            RoundType::Unidentified => 0,
            RoundType::Automatic => 1,
            RoundType::ManualNormal => 2,
            RoundType::ManualSpecial => 3,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SpecialConditionScore {
    Unidentified,
    NotFinished,
    Bye,
}

impl SpecialConditionScore {
    pub fn from_i32(round_type: i32) -> SpecialConditionScore {
        match round_type {
            -1 => SpecialConditionScore::NotFinished,
            -2 => SpecialConditionScore::Bye,
            _ => SpecialConditionScore::Unidentified,
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            SpecialConditionScore::Unidentified => -100,
            SpecialConditionScore::NotFinished => -1,
            SpecialConditionScore::Bye => -2,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PlayerColor {
    Black,
    White,
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

    mod test_tournament_type {
        use crate::properties::TournamentType;

        #[test]
        fn test_from_string() {
            assert_eq!(
                TournamentType::from_string(String::from("round_robin")),
                TournamentType::RoundRobin
            );
            assert_eq!(
                TournamentType::from_string(String::from("swiss_pairing")),
                TournamentType::SwissPairing
            );
            assert_eq!(
                TournamentType::from_string(String::from("")),
                TournamentType::Unidentified
            );
            assert_eq!(
                TournamentType::from_string(String::from("random")),
                TournamentType::Unidentified
            );
        }

        #[test]
        fn test_to_string() {
            assert_eq!(TournamentType::RoundRobin.to_string(), String::from("round_robin"));
            assert_eq!(TournamentType::SwissPairing.to_string(), String::from("swiss_pairing"));
            assert_eq!(TournamentType::Unidentified.to_string(), String::from("unidentified"));
        }
    }

    mod test_round_type {
        use crate::properties::RoundType;

        #[test]
        fn test_from_i32() {
            assert_eq!(
                RoundType::from_i32(1),
                RoundType::Automatic
            );
            assert_eq!(
                RoundType::from_i32(2),
                RoundType::ManualNormal
            );
            assert_eq!(
                RoundType::from_i32(3),
                RoundType::ManualSpecial
            );
            assert_eq!(
                RoundType::from_i32(4),
                RoundType::Unidentified
            );
            assert_eq!(
                RoundType::from_i32(0),
                RoundType::Unidentified
            );
        }

        #[test]
        fn test_to_i32() {
            assert_eq!(RoundType::Automatic.to_i32(), 1);
            assert_eq!(RoundType::ManualNormal.to_i32(), 2);
            assert_eq!(RoundType::ManualSpecial.to_i32(), 3);
            assert_eq!(RoundType::Unidentified.to_i32(), 0);
        }
    }

    mod test_special_condition_score {
        use crate::properties::SpecialConditionScore;

        #[test]
        fn test_from_i32() {
            assert_eq!(
                SpecialConditionScore::from_i32(-1),
                SpecialConditionScore::NotFinished
            );
            assert_eq!(
                SpecialConditionScore::from_i32(-2),
                SpecialConditionScore::Bye
            );
            assert_eq!(
                SpecialConditionScore::from_i32(-100),
                SpecialConditionScore::Unidentified
            );
        }

        #[test]
        fn test_to_i32() {
            assert_eq!(SpecialConditionScore::NotFinished.to_i32(), -1);
            assert_eq!(SpecialConditionScore::Bye.to_i32(), -2);
            assert_eq!(SpecialConditionScore::Unidentified.to_i32(), -100);
        }
    }
}
