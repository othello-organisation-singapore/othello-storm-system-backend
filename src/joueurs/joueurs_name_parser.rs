#[derive(Debug, PartialEq)]
enum NameParserState {
    FirstName,
    LastName
}

pub struct NameParser {
    to_parse: String,
    state: NameParserState,
}

impl NameParser {
    pub fn create() -> NameParser {
        NameParser {
            to_parse: String::new(),
            state: NameParserState::LastName
        }
    }
}
