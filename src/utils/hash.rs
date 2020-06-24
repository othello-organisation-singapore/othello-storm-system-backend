use pwhash::bcrypt;

pub fn hash(string: &String) -> String {
    bcrypt::hash(string).unwrap()
}

pub fn verify(string: &String, hashed_string: &String) -> bool {
    bcrypt::verify(string, hashed_string)
}
