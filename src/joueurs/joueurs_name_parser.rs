#[derive(Debug, PartialEq)]
enum NameParserState {
    FirstName,
    LastName
}

pub struct NameParser {
    state: NameParserState,
    first_name: String,
    last_name: String
}

pub struct PlayerName {
    pub first_name: String,
    pub last_name: String
}

impl NameParser {
    pub fn create() -> NameParser {
        NameParser {
            state: NameParserState::LastName,
            first_name: String::new(),
            last_name: String::new(),
        }
    }

    pub fn parse(&mut self, name: &String) -> PlayerName {
        self.reset();
        let name = name.trim();
        for char in name.chars() {
            self.parse_char(char);
        }
        PlayerName {
            first_name: String::from(self.first_name.trim()),
            last_name: String::from(self.last_name.trim()),
        }
    }

    fn reset(&mut self) {
        self.state = NameParserState::LastName;
        self.first_name = String::new();
        self.last_name = String::new();
    }

    fn parse_char(&mut self, char: char) {
        if char == ')' {
            return
        }
        if char == '(' || char == ',' {
            self.move_to_next_state();
            return
        }

        match self.state {
            NameParserState::FirstName => self.first_name.push(char),
            NameParserState::LastName => self.last_name.push(char),
        }
    }

    fn move_to_next_state(&mut self) {
        match self.state {
            NameParserState::FirstName => self.state = NameParserState::LastName,
            NameParserState::LastName => self.state = NameParserState::FirstName,
        }
    }
}
