use crate::tournament_manager::Player;
use super::joueurs_name_parser::NameParser;

const DEFAULT_RATING: i32 = 1200;
const DEFAULT_ID: i32 = -1;

#[derive(Debug, PartialEq)]
enum PlayerParserState {
    ID,
    Name,
    Rating
}

pub struct PlayerParser {
    state: PlayerParserState,
    id: String,
    name: String,
    rating: String
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
            joueurs_id: self.id.parse::<i32>().unwrap_or(DEFAULT_ID),
            first_name: parsed_name.first_name,
            last_name: parsed_name.last_name,
            country: country.clone(),
            rating: self.rating.parse::<i32>().unwrap_or(DEFAULT_RATING)            
        }
    }

    fn reset(&mut self) {
        self.state =  PlayerParserState::ID;
        self.id = String::new();
        self.name = String::new();
        self.rating = String::new();
    }

    fn parse_char(&mut self, char: char) {
        if char == '%' || char == '_' || char == '>' {
            return
        }

        match self.state {
            PlayerParserState::ID => {
                if char == ' ' {
                    self.move_to_next_state();
                    return
                }
                self.id.push(char)
            },
            PlayerParserState::Name => {
                if char == '<' {
                    self.move_to_next_state();
                    return
                }
                self.name.push(char)
            },
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
