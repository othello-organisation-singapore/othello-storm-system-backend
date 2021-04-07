use super::joueurs_name_parser::NameParser;
use crate::tournament_manager::Player;

const DEFAULT_RATING: i32 = 1200;

#[derive(Debug, PartialEq)]
enum PlayerParserState {
    ID,
    Name,
    Rating,
}

pub struct PlayerParser {
    state: PlayerParserState,
    id: String,
    name: String,
    rating: String,
}

impl PlayerParser {
    pub fn create() -> PlayerParser {
        PlayerParser {
            state: PlayerParserState::ID,
            id: String::new(),
            name: String::new(),
            rating: String::new(),
        }
    }

    pub fn parse(&mut self, player: &String, country: &String) -> Player {
        self.reset();

        let player = player.trim();
        for char in player.chars() {
            self.parse_char(char);
        }

        let mut name_parser = NameParser::create();
        let parsed_name = name_parser.parse(&self.name);

        Player {
            joueurs_id: self.id.clone(),
            first_name: parsed_name.first_name,
            last_name: parsed_name.last_name,
            country: country.clone(),
            rating: self.rating.parse::<i32>().unwrap_or(DEFAULT_RATING),
        }
    }

    fn reset(&mut self) {
        self.state = PlayerParserState::ID;
        self.id = String::new();
        self.name = String::new();
        self.rating = String::new();
    }

    fn parse_char(&mut self, char: char) {
        if char == '%' || char == '_' || char == '>' {
            return;
        }

        match self.state {
            PlayerParserState::ID => {
                if char == ' ' {
                    self.move_to_next_state();
                    return;
                }
                self.id.push(char);
            }
            PlayerParserState::Name => {
                if char == '<' {
                    self.move_to_next_state();
                    return;
                }
                self.name.push(char);
            }
            PlayerParserState::Rating => self.rating.push(char),
        }
    }

    fn move_to_next_state(&mut self) {
        match self.state {
            PlayerParserState::ID => self.state = PlayerParserState::Name,
            PlayerParserState::Name => self.state = PlayerParserState::Rating,
            PlayerParserState::Rating => self.state = PlayerParserState::ID,
        }
    }
}

#[cfg(test)]
mod tests {
    mod test_player_parser {
        use crate::joueurs::joueurs_player_parser::{PlayerParser, DEFAULT_RATING};

        fn test_parse_player(
            player: &String,
            expected_id: &String,
            expected_first_name: &String,
            expected_last_name: &String,
            expected_rating: &i32,
        ) -> bool {
            let mut parser = PlayerParser::create();
            let country = String::from("test_country");
            let parsed_player = parser.parse(player, &country);

            &parsed_player.joueurs_id == expected_id
                && &parsed_player.first_name == expected_first_name
                && &parsed_player.last_name == expected_last_name
                && &parsed_player.rating == expected_rating
        }

        #[test]
        fn test_parse_comma_rated() {
            let player = String::from("280216 ACUNA SSX, Ricardo                       %_<1484>");
            let expected_first_name = String::from("Ricardo");
            let expected_last_name = String::from("ACUNA SSX");
            let expected_id = String::from("280216");
            let expected_rating = 1484;
            assert_eq!(
                test_parse_player(
                    &player,
                    &expected_id,
                    &expected_first_name,
                    &expected_last_name,
                    &expected_rating,
                ),
                true
            );
        }

        #[test]
        fn test_parse_comma_negative_rated() {
            let player = String::from("280216 ACUNA SSX, Ricardo                       %_<-1484>");
            let expected_first_name = String::from("Ricardo");
            let expected_last_name = String::from("ACUNA SSX");
            let expected_id = String::from("280216");
            let expected_rating = -1484;
            assert_eq!(
                test_parse_player(
                    &player,
                    &expected_id,
                    &expected_first_name,
                    &expected_last_name,
                    &expected_rating,
                ),
                true
            );
        }

        #[test]
        fn test_parse_comma_unrated() {
            let player = String::from("281018 GRECO, Alejandra de                    ");
            let expected_first_name = String::from("Alejandra de");
            let expected_last_name = String::from("GRECO");
            let expected_id = String::from("281018");
            let expected_rating = DEFAULT_RATING;
            assert_eq!(
                test_parse_player(
                    &player,
                    &expected_id,
                    &expected_first_name,
                    &expected_last_name,
                    &expected_rating,
                ),
                true
            );
        }

        #[test]
        fn test_parse_comma_no_first_name() {
            let player = String::from("281018 GRECO,                    ");
            let expected_first_name = String::from("");
            let expected_last_name = String::from("GRECO");
            let expected_id = String::from("281018");
            let expected_rating = DEFAULT_RATING;
            assert_eq!(
                test_parse_player(
                    &player,
                    &expected_id,
                    &expected_first_name,
                    &expected_last_name,
                    &expected_rating,
                ),
                true
            );
        }

        #[test]
        fn test_parse_bracket_rated() {
            let player = String::from("   957 BRUTUS S (Geoffroy-Piotte)             %_<2410>");
            let expected_first_name = String::from("Geoffroy-Piotte");
            let expected_last_name = String::from("BRUTUS S");
            let expected_id = String::from("957");
            let expected_rating = 2410;
            assert_eq!(
                test_parse_player(
                    &player,
                    &expected_id,
                    &expected_first_name,
                    &expected_last_name,
                    &expected_rating,
                ),
                true
            );
        }

        #[test]
        fn test_parse_bracket_negative_rated() {
            let player = String::from("   957 BRUTUS S (Geoffroy-Piotte)             %_<-2410>");
            let expected_first_name = String::from("Geoffroy-Piotte");
            let expected_last_name = String::from("BRUTUS S");
            let expected_id = String::from("957");
            let expected_rating = -2410;
            assert_eq!(
                test_parse_player(
                    &player,
                    &expected_id,
                    &expected_first_name,
                    &expected_last_name,
                    &expected_rating,
                ),
                true
            );
        }

        #[test]
        fn test_parse_bracket_unrated() {
            let player = String::from("   349 EXPERT5 (Reversi Othello)                   ");
            let expected_first_name = String::from("Reversi Othello");
            let expected_last_name = String::from("EXPERT5");
            let expected_id = String::from("349");
            let expected_rating = DEFAULT_RATING;
            assert_eq!(
                test_parse_player(
                    &player,
                    &expected_id,
                    &expected_first_name,
                    &expected_last_name,
                    &expected_rating,
                ),
                true
            );
        }

        #[test]
        fn test_parse_bracket_no_first_name() {
            let player = String::from("   349 EXPERT5 ()                   ");
            let expected_first_name = String::from("");
            let expected_last_name = String::from("EXPERT5");
            let expected_id = String::from("349");
            let expected_rating = DEFAULT_RATING;
            assert_eq!(
                test_parse_player(
                    &player,
                    &expected_id,
                    &expected_first_name,
                    &expected_last_name,
                    &expected_rating,
                ),
                true
            );
        }
    }
}
