use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::errors::ErrorType;

pub struct Player {
    pub joueurs_id: String,
    pub first_name: String,
    pub last_name: String,
    pub country: String,
    pub rating: i32,
}


impl Player {
    pub fn from_hashmap(player_data: HashMap<String, String>) -> Result<Player, ErrorType> {
        let joueurs_id = player_data
            .get("joueurs_id")
            .ok_or(ErrorType::UnknownError(String::from("Incomplete player data")))?
            .to_string();

        let first_name = player_data
            .get("first_name")
            .ok_or(ErrorType::UnknownError(String::from("Incomplete player data")))?
            .to_string();

        let last_name = player_data
            .get("last_name")
            .ok_or(ErrorType::UnknownError(String::from("Incomplete player data")))?
            .to_string();

        let country = player_data
            .get("country")
            .ok_or(ErrorType::UnknownError(String::from("Incomplete player data")))?
            .to_string();

        let rating = player_data
            .get("rating")
            .ok_or(ErrorType::UnknownError(String::from("Incomplete player data")))?
            .parse::<i32>()
            .map_err(|_| ErrorType::UnknownError(String::from("Rating is not integer")))?;

        Ok(Player{ joueurs_id, first_name, last_name, country, rating})
    }

    pub fn to_hashmap(&self) -> HashMap<String, String> {
        let mut player_data = HashMap::new();
        player_data.insert(String::from("joueurs_id"), self.joueurs_id.clone());
        player_data.insert(String::from("first_name"), self.first_name.clone());
        player_data.insert(String::from("last_name"), self.last_name.clone());
        player_data.insert(String::from("country"), self.country.clone());
        player_data.insert(String::from("rating"), self.rating.to_string());
        player_data
    }
}

impl Player {
    pub fn from_serdemap(player_data: Map<String, Value>) -> Result<Player, ErrorType> {
        let joueurs_id = player_data
            .get("joueurs_id")
            .ok_or(ErrorType::UnknownError(String::from("Incomplete player data")))?
            .as_str()
            .ok_or(ErrorType::UnknownError(String::from("Player data format is wrong.")))?
            .to_string();

        let first_name = player_data
            .get("first_name")
            .ok_or(ErrorType::UnknownError(String::from("Incomplete player data")))?
            .as_str()
            .ok_or(ErrorType::UnknownError(String::from("Player data format is wrong.")))?
            .to_string();

        let last_name = player_data
            .get("last_name")
            .ok_or(ErrorType::UnknownError(String::from("Incomplete player data")))?
            .as_str()
            .ok_or(ErrorType::UnknownError(String::from("Player data format is wrong.")))?
            .to_string();

        let country = player_data
            .get("country")
            .ok_or(ErrorType::UnknownError(String::from("Incomplete player data")))?
            .as_str()
            .ok_or(ErrorType::UnknownError(String::from("Player data format is wrong.")))?
            .to_string();

        let rating = player_data
            .get("rating")
            .ok_or(ErrorType::UnknownError(String::from("Incomplete player data")))?
            .as_str()
            .ok_or(ErrorType::UnknownError(String::from("Player data format is wrong.")))?
            .parse::<i32>()
            .map_err(|_| ErrorType::UnknownError(String::from("Rating is not integer")))?;

        Ok(Player{ joueurs_id, first_name, last_name, country, rating})
    }

    pub fn to_serdemap(&self) -> Map<String, Value> {
        let mut player_data = Map::new();
        player_data.insert(
            String::from("joueurs_id"), Value::from(self.joueurs_id.clone())
        );
        player_data.insert(
            String::from("first_name"), Value::from(self.first_name.clone())
        );
        player_data.insert(
            String::from("last_name"), Value::from(self.last_name.clone())
        );
        player_data.insert(
            String::from("country"), Value::from(self.country.clone())
        );
        player_data.insert(
            String::from("rating"), Value::from(self.rating.to_string())
        );
        player_data
    }
}


#[cfg(test)]
mod tests {
    mod test_from_and_to_hashmap {
        use std::collections::HashMap;
        use crate::tournament_manager::Player;

        #[test]
        fn test_from_hashmap_complete() {
            let mut data = HashMap::new();
            data.insert(String::from("joueurs_id"), String::from("132"));
            data.insert(String::from("first_name"), String::from("first_name_1"));
            data.insert(String::from("last_name"), String::from("last_name_1"));
            data.insert(String::from("country"), String::from("SGP"));
            data.insert(String::from("rating"), String::from("1815"));
            let player_result = Player::from_hashmap(data);
            assert_eq!(player_result.is_ok(), true);

            let player = player_result.unwrap();
            assert_eq!(player.joueurs_id, String::from("132"));
            assert_eq!(player.first_name, String::from("first_name_1"));
            assert_eq!(player.last_name, String::from("last_name_1"));
            assert_eq!(player.country, String::from("SGP"));
            assert_eq!(player.rating, 1815);
        }

        #[test]
        fn test_from_hashmap_incomplete_string_field() {
            let mut data = HashMap::new();
            data.insert(String::from("joueurs_id"), String::from("132"));
            data.insert(String::from("first_name"), String::from("first_name_1"));
            data.insert(String::from("country"), String::from("SGP"));
            data.insert(String::from("rating"), String::from("1815"));
            let player_result = Player::from_hashmap(data);
            assert_eq!(player_result.is_err(), true);
        }

        #[test]
        fn test_from_hashmap_incomplete_i32_field() {
            let mut data = HashMap::new();
            data.insert(String::from("joueurs_id"), String::from("1815"));
            data.insert(String::from("first_name"), String::from("first_name_1"));
            data.insert(String::from("last_name"), String::from("last_name_1"));
            data.insert(String::from("country"), String::from("SGP"));
            let player_result = Player::from_hashmap(data);
            assert_eq!(player_result.is_err(), true);
        }

        #[test]
        fn test_from_hashmap_wrong_field() {
            let mut data = HashMap::new();
            data.insert(String::from("joueurs_id"), String::from("this_is_not_i32"));
            data.insert(String::from("first_name"), String::from("first_name_1"));
            data.insert(String::from("last_name"), String::from("last_name_1"));
            data.insert(String::from("country"), String::from("SGP"));
            data.insert(String::from("rating"), String::from("a1815"));
            let player_result = Player::from_hashmap(data);
            assert_eq!(player_result.is_err(), true);
        }

        #[test]
        fn test_to_hashmap() {
            let player = Player {
                joueurs_id: String::from("145"),
                first_name: String::from("first_name_1"),
                last_name: String::from("last_name_1"),
                country: String::from("country_SGP"),
                rating: 1250
            };
            let data = player.to_hashmap();
            assert_eq!(data.get("joueurs_id").unwrap(), &String::from("145"));
            assert_eq!(data.get("first_name").unwrap(), &String::from("first_name_1"));
            assert_eq!(data.get("last_name").unwrap(), &String::from("last_name_1"));
            assert_eq!(data.get("country").unwrap(),  &String::from("country_SGP"));
            assert_eq!(data.get("rating").unwrap(), &String::from("1250"));
        }
    }

    mod test_from_and_to_serdemap {
        use serde_json::{Map, Value};
        use crate::tournament_manager::Player;

        #[test]
        fn test_from_serdemap_complete() {
            let mut data = Map::new();
            data.insert(String::from("joueurs_id"), Value::from("132"));
            data.insert(String::from("first_name"), Value::from("first_name_1"));
            data.insert(String::from("last_name"), Value::from("last_name_1"));
            data.insert(String::from("country"), Value::from("SGP"));
            data.insert(String::from("rating"), Value::from("1815"));
            let player_result = Player::from_serdemap(data);
            assert_eq!(player_result.is_ok(), true);

            let player = player_result.unwrap();
            assert_eq!(player.joueurs_id, String::from("132"));
            assert_eq!(player.first_name, String::from("first_name_1"));
            assert_eq!(player.last_name, String::from("last_name_1"));
            assert_eq!(player.country, String::from("SGP"));
            assert_eq!(player.rating, 1815);
        }

        #[test]
        fn test_from_serdemap_incomplete_string_field() {
            let mut data = Map::new();
            data.insert(String::from("joueurs_id"), Value::from("132"));
            data.insert(String::from("first_name"), Value::from("first_name_1"));
            data.insert(String::from("country"), Value::from("SGP"));
            data.insert(String::from("rating"), Value::from("1815"));
            let player_result = Player::from_serdemap(data);
            assert_eq!(player_result.is_err(), true);
        }

        #[test]
        fn test_from_serdemap_incomplete_i32_field() {
            let mut data = Map::new();
            data.insert(String::from("joueurs_id"), Value::from("1815"));
            data.insert(String::from("first_name"), Value::from("first_name_1"));
            data.insert(String::from("last_name"), Value::from("last_name_1"));
            data.insert(String::from("country"), Value::from("SGP"));
            let player_result = Player::from_serdemap(data);
            assert_eq!(player_result.is_err(), true);
        }

        #[test]
        fn test_from_serdemap_wrong_field() {
            let mut data = Map::new();
            data.insert(String::from("joueurs_id"), Value::from("this_is_not_i32"));
            data.insert(String::from("first_name"), Value::from("first_name_1"));
            data.insert(String::from("last_name"), Value::from("last_name_1"));
            data.insert(String::from("country"), Value::from("SGP"));
            data.insert(String::from("rating"), Value::from("a1815"));
            let player_result = Player::from_serdemap(data);
            assert_eq!(player_result.is_err(), true);
        }

        #[test]
        fn test_to_serdemap() {
            let player = Player {
                joueurs_id: String::from("145"),
                first_name: String::from("first_name_1"),
                last_name: String::from("last_name_1"),
                country: String::from("country_SGP"),
                rating: 1250
            };
            let data = player.to_serdemap();
            assert_eq!(data.get("joueurs_id").unwrap(), &Value::from("145"));
            assert_eq!(data.get("first_name").unwrap(), &Value::from("first_name_1"));
            assert_eq!(data.get("last_name").unwrap(), &Value::from("last_name_1"));
            assert_eq!(data.get("country").unwrap(),  &Value::from("country_SGP"));
            assert_eq!(data.get("rating").unwrap(), &Value::from("1250"));
        }
    }
}
