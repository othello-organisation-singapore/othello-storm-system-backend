use std::cmp::Ordering::Equal;
use std::collections::{HashMap, HashSet};
use std::env;
use std::iter::FromIterator;
use std::str::FromStr;

use crate::game_match::IGameMatch;
use crate::properties::PlayerColor;

#[derive(Clone)]
pub struct PlayerStanding {
    player_id: i32,
    major_score: f64,
    minor_score: f64,
    match_history: Vec<Box<dyn IGameMatch>>,
}

pub trait IResultKeeper {
    fn has_players_met(&self, player_1_id: &i32, player_2_id: &i32) -> bool;
    fn get_standings(&self) -> Vec<i32>;
    fn get_detailed_standings(&self) -> Vec<PlayerStanding>;
    fn is_empty(&self) -> bool;
    fn get_color_count(&self, player_id: &i32, color: PlayerColor) -> i32;
}

pub struct ResultKeeper {
    sorted_player_standings: Vec<PlayerStanding>,
    opponents_ids_by_player_id: HashMap<i32, HashSet<i32>>,
    black_color_count_by_player_id: HashMap<i32, i32>,
    white_color_count_by_player_id: HashMap<i32, i32>,
}

impl ResultKeeper {}

impl IResultKeeper for ResultKeeper {
    fn has_players_met(&self, player_1_id: &i32, player_2_id: &i32) -> bool {
        self.opponents_ids_by_player_id
            .get(player_1_id)
            .unwrap_or(&HashSet::new())
            .contains(player_2_id)
    }

    fn get_standings(&self) -> Vec<i32> {
        self.sorted_player_standings[..]
            .iter()
            .map(|standing| standing.player_id)
            .collect()
    }

    fn get_detailed_standings(&self) -> Vec<PlayerStanding> {
        self.sorted_player_standings.clone()
    }

    fn is_empty(&self) -> bool {
        self.sorted_player_standings.is_empty()
    }

    fn get_color_count(&self, player_id: &i32, color: PlayerColor) -> i32 {
        match color {
            PlayerColor::Black => self.black_color_count_by_player_id.get(player_id).unwrap_or(&0),
            PlayerColor::White => self.white_color_count_by_player_id.get(player_id).unwrap_or(&0),
        }.clone()
    }
}


pub fn create_result_keeper(matches: &Vec<Box<dyn IGameMatch>>) -> Box<dyn IResultKeeper> {
    let mut player_ids: HashSet<i32> = HashSet::new();

    matches[..]
        .iter()
        .for_each(|game_match| {
            let players_id = game_match.get_players_id();
            if let Some(id) = players_id.0 {
                player_ids.insert(id);
            }
            if let Some(id) = players_id.1 {
                player_ids.insert(id);
            }
        });

    let sorted_player_standings = get_sorted_player_standings(&player_ids, &matches);
    let opponents_ids_by_player_id = get_opponent_ids_by_player_id(&player_ids, &matches);

    let black_color_count_by_player_id = get_color_count_by_player_id(&player_ids, &matches, PlayerColor::Black);
    let white_color_count_by_player_id = get_color_count_by_player_id(&player_ids, &matches, PlayerColor::White);

    Box::from(
        ResultKeeper {
            sorted_player_standings,
            opponents_ids_by_player_id,
            black_color_count_by_player_id,
            white_color_count_by_player_id
        }
    )
}

fn get_opponent_ids_by_player_id(
    player_ids: &HashSet<i32>,
    matches: &Vec<Box<dyn IGameMatch>>,
) -> HashMap<i32, HashSet<i32>> {
    HashMap::from_iter(
        player_ids
            .iter()
            .map(|id| {
                let opponent_ids = HashSet::from_iter(
                    matches[..]
                        .iter()
                        .map(|game_match| game_match.get_opponent_id(id))
                        .filter(|result| result.is_some())
                        .map(|result| result.unwrap())
                );
                (id.clone(), opponent_ids)
            })
    )
}

fn get_sorted_player_standings<'a>(
    player_ids: &HashSet<i32>,
    matches: &Vec<Box<dyn IGameMatch>>,
) -> Vec<PlayerStanding> {
    let major_scores_by_id = get_major_scores_by_player_id(player_ids, matches);
    let mut standings: Vec<PlayerStanding> = player_ids
        .iter()
        .map(|id| {
            let filtered_matches: Vec<Box<dyn IGameMatch>> = matches
                .into_iter()
                .filter(|game_match| game_match.is_player_playing(id))
                .map(|game_match| game_match.clone())
                .collect();
            let major_score = major_scores_by_id.get(id).unwrap_or(&0.0);
            let minor_score = calculate_minor_score(
                id,
                &filtered_matches,
                &major_scores_by_id,
            );
            PlayerStanding {
                player_id: id.clone(),
                major_score: major_score.clone(),
                minor_score,
                match_history: filtered_matches,
            }
        })
        .collect();
    standings.sort_by(|a, b| {
        if a.major_score == b.major_score {
            return b.minor_score.partial_cmp(&a.minor_score).unwrap_or(Equal);
        }
        b.major_score.partial_cmp(&a.major_score).unwrap_or(Equal)
    });
    standings
}


fn get_major_scores_by_player_id(
    player_ids: &HashSet<i32>,
    matches: &Vec<Box<dyn IGameMatch>>,
) -> HashMap<i32, f64> {
    HashMap::from_iter(
        player_ids
            .iter()
            .map(|id| (id.clone(), calculate_major_score(id, matches)))
    )
}


fn calculate_major_score(player_id: &i32, matches: &Vec<Box<dyn IGameMatch>>) -> f64 {
    matches
        .iter()
        .map(|game_match| game_match.calculate_major_score(player_id))
        .sum()
}

fn calculate_minor_score(
    player_id: &i32,
    matches: &Vec<Box<dyn IGameMatch>>,
    major_scores_by_player_ids: &HashMap<i32, f64>,
) -> f64 {
    // Following https://www.worldothello.org/about/world-othello-championship/woc-rules
    let brightwell_constant = f64::from_str(
        &env::var("BRIGHTWELL_CONSTANT").unwrap()[..]
    ).unwrap();
    matches
        .iter()
        .map(|game_match| game_match.calculate_minor_score(
            player_id,
            major_scores_by_player_ids,
            &brightwell_constant,
        ))
        .sum()
}

fn get_color_count_by_player_id(
    player_ids: &HashSet<i32>,
    matches: &Vec<Box<dyn IGameMatch>>,
    color: PlayerColor,
) -> HashMap<i32, i32> {
    HashMap::from_iter(
        player_ids
            .iter()
            .map(|id| (
                id.clone(),
                matches
                    .iter()
                    .map(|game_match| game_match.get_player_color(id))
                    .filter(|result| result.is_some() && result.as_ref().unwrap() == &color)
                    .count() as i32
            ))
    )
}

#[cfg(test)]
mod tests {
    mod test_get_standings {
        use std::env;
        use std::str::FromStr;

        use serde_json::{Map, Value};

        use crate::game_match::GameMatchCreator;
        use crate::tournament_manager::create_result_keeper;

        #[test]
        fn test_standard() {
            let game_matches = vec![
                GameMatchCreator::create_new_finished_match(
                    &1,
                    &1,
                    &2,
                    &40,
                    &24,
                    &Value::from(Map::new()),
                ),
                GameMatchCreator::create_new_bye_match(
                    &2,
                    &3,
                    &Value::from(Map::new()),
                ),
                GameMatchCreator::create_new_finished_match(
                    &2,
                    &1,
                    &3,
                    &30,
                    &34,
                    &Value::from(Map::new()),
                ),
            ];
            let result_keeper = create_result_keeper(&game_matches);
            let standings = result_keeper.get_detailed_standings();
            let brightwell_constant = f64::from_str(
                &env::var("BRIGHTWELL_CONSTANT").unwrap()[..]
            ).unwrap();

            assert_eq!(standings[0].player_id, 3);
            assert_eq!(standings[0].major_score, 2.0);
            assert_eq!(standings[0].minor_score, 66.0 + brightwell_constant * 3.0);
            assert_eq!(standings[0].match_history.len(), 2);
            assert_eq!(standings[0].match_history[0].extract_data(), game_matches[1].extract_data());
            assert_eq!(standings[0].match_history[1].extract_data(), game_matches[2].extract_data());

            assert_eq!(standings[1].player_id, 1);
            assert_eq!(standings[1].major_score, 1.0);
            assert_eq!(standings[1].minor_score, 70.0 + brightwell_constant * 2.0);
            assert_eq!(standings[1].match_history.len(), 2);
            assert_eq!(standings[1].match_history[0].extract_data(), game_matches[0].extract_data());
            assert_eq!(standings[1].match_history[1].extract_data(), game_matches[2].extract_data());

            assert_eq!(standings[2].player_id, 2);
            assert_eq!(standings[2].major_score, 0.0);
            assert_eq!(standings[2].minor_score, 24.0 + brightwell_constant * 1.0);
            assert_eq!(standings[2].match_history.len(), 1);
            assert_eq!(standings[2].match_history[0].extract_data(), game_matches[0].extract_data());
        }
    }

    mod test_has_player_met {
        use serde_json::{Map, Value};

        use crate::game_match::GameMatchCreator;
        use crate::tournament_manager::create_result_keeper;

        #[test]
        fn test_standard() {
            let game_matches = vec![
                GameMatchCreator::create_new_finished_match(
                    &1,
                    &1,
                    &2,
                    &40,
                    &24,
                    &Value::from(Map::new()),
                ),
                GameMatchCreator::create_new_bye_match(
                    &2,
                    &3,
                    &Value::from(Map::new()),
                ),
                GameMatchCreator::create_new_finished_match(
                    &2,
                    &1,
                    &3,
                    &30,
                    &34,
                    &Value::from(Map::new()),
                ),
            ];
            let result_keeper = create_result_keeper(&game_matches);
            assert_eq!(result_keeper.has_players_met(&1, &2), true);
            assert_eq!(result_keeper.has_players_met(&1, &3), true);
            assert_eq!(result_keeper.has_players_met(&2, &3), false);
            assert_eq!(result_keeper.has_players_met(&2, &1), true);
            assert_eq!(result_keeper.has_players_met(&3, &1), true);
            assert_eq!(result_keeper.has_players_met(&3, &2), false);
        }
    }

    mod test_get_color_count {
        use serde_json::{Map, Value};

        use crate::game_match::GameMatchCreator;
        use crate::properties::PlayerColor;
        use crate::tournament_manager::create_result_keeper;

        #[test]
        fn test_standard() {
            let game_matches = vec![
                GameMatchCreator::create_new_finished_match(
                    &1,
                    &1,
                    &2,
                    &40,
                    &24,
                    &Value::from(Map::new()),
                ),
                GameMatchCreator::create_new_bye_match(
                    &2,
                    &3,
                    &Value::from(Map::new()),
                ),
                GameMatchCreator::create_new_finished_match(
                    &2,
                    &1,
                    &3,
                    &30,
                    &34,
                    &Value::from(Map::new()),
                ),
            ];
            let result_keeper = create_result_keeper(&game_matches);

            assert_eq!(result_keeper.get_color_count(&1, PlayerColor::Black), 2);
            assert_eq!(result_keeper.get_color_count(&1, PlayerColor::White), 0);
            assert_eq!(result_keeper.get_color_count(&2, PlayerColor::Black), 0);
            assert_eq!(result_keeper.get_color_count(&2, PlayerColor::White), 1);
            assert_eq!(result_keeper.get_color_count(&3, PlayerColor::Black), 0);
            assert_eq!(result_keeper.get_color_count(&3, PlayerColor::White), 1);
        }
    }
}
