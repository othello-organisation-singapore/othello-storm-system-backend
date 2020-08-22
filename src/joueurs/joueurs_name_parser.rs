#[derive(Debug, PartialEq)]
enum NameParserState {
    FirstName,
    LastName,
}

pub struct NameParser {
    state: NameParserState,
    first_name: String,
    last_name: String,
}

pub struct PlayerName {
    pub first_name: String,
    pub last_name: String,
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
            return;
        }
        if char == '(' || char == ',' {
            self.move_to_next_state();
            return;
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

#[cfg(test)]
mod tests {
    mod test_name_parser {
        use crate::joueurs::joueurs_name_parser::NameParser;

        fn test_parse_name(
            name: &String, expected_first_name: &String, expected_last_name: &String,
        ) -> bool {
            let mut parser = NameParser::create();
            let player_name = parser.parse(name);

            &player_name.first_name == expected_first_name
                && &player_name.last_name == expected_last_name
        }

        #[test]
        fn test_parse_comma_normal() {
            let name = String::from("CAMPO, Sebastian");
            let expected_first_name = String::from("Sebastian");
            let expected_last_name = String::from("CAMPO");
            assert_eq!(test_parse_name(&name, &expected_first_name, &expected_last_name), true);
        }

        #[test]
        fn test_parse_comma_name_multiple_words() {
            let name = String::from("DE JONG, Thiago De Oliveira");
            let expected_first_name = String::from("Thiago De Oliveira");
            let expected_last_name = String::from("DE JONG");
            assert_eq!(test_parse_name(&name, &expected_first_name, &expected_last_name), true);
        }

        #[test]
        fn test_parse_comma_name_with_dots() {
            let name = String::from("DE. GRAAF, Jan C.");
            let expected_first_name = String::from("Jan C.");
            let expected_last_name = String::from("DE. GRAAF");
            assert_eq!(test_parse_name(&name, &expected_first_name, &expected_last_name), true);
        }

        #[test]
        fn test_parse_comma_name_with_hyphen() {
            let name = String::from("DAHL-PAULSEN, Per-Helge");
            let expected_first_name = String::from("Per-Helge");
            let expected_last_name = String::from("DAHL-PAULSEN");
            assert_eq!(test_parse_name(&name, &expected_first_name, &expected_last_name), true);
        }

        #[test]
        fn test_parse_comma_no_first_name() {
            let name = String::from("SOBEA,");
            let expected_first_name = String::from("");
            let expected_last_name = String::from("SOBEA");
            assert_eq!(test_parse_name(&name, &expected_first_name, &expected_last_name), true);
        }

        #[test]
        fn test_parse_bracket_normal() {
            let name = String::from("ATHALIE (Calien)");
            let expected_first_name = String::from("Calien");
            let expected_last_name = String::from("ATHALIE");
            assert_eq!(test_parse_name(&name, &expected_first_name, &expected_last_name), true);
        }

        #[test]
        fn test_parse_bracket_no_first_name() {
            let name = String::from("ATHALIE ()");
            let expected_first_name = String::from("");
            let expected_last_name = String::from("ATHALIE");
            assert_eq!(test_parse_name(&name, &expected_first_name, &expected_last_name), true);
        }

        #[test]
        fn test_parse_bracket_multiple_words_with_hyphen() {
            let name = String::from("HANNIBAL TEST (Piotte-Geoffroy / De Haan)");
            let expected_first_name = String::from("Piotte-Geoffroy / De Haan");
            let expected_last_name = String::from("HANNIBAL TEST");
            assert_eq!(test_parse_name(&name, &expected_first_name, &expected_last_name), true);
        }
    }
}

