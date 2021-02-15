use std::cmp::Ordering::Equal;
use std::collections::{HashMap, HashSet};
use std::env;
use std::iter::FromIterator;
use std::str::FromStr;

use super::{GameMatch, IGameMatch};

#[derive(Clone)]
pub struct PlayerStanding {
    player_id: i32,
    major_score: f64,
    minor_score: f64,
    match_history: Vec<GameMatch>,
}

pub trait IResultKeeper where Self: Sized {
    fn from_matches(matches: Vec<GameMatch>) -> Self;
    fn has_players_met(&self, player_1_id: &i32, player_2_id: &i32) -> bool;
    fn get_standings(&self) -> Vec<i32>;
    fn get_detailed_standings(&self) -> Vec<PlayerStanding>;
}

pub struct ResultKeeper {
    sorted_player_standings: Vec<PlayerStanding>,
    opponents_ids_by_player_id: HashMap<i32, HashSet<i32>>,
}

impl ResultKeeper {}

impl IResultKeeper for ResultKeeper {
    fn from_matches(matches: Vec<GameMatch>) -> Self {
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

        ResultKeeper { sorted_player_standings, opponents_ids_by_player_id }
    }

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
}

fn get_opponent_ids_by_player_id(
    player_ids: &HashSet<i32>,
    matches: &Vec<GameMatch>,
) -> HashMap<i32, HashSet<i32>> {
    HashMap::from_iter(
        player_ids
            .iter()
            .map(|id| {
                let opponent_ids = HashSet::from_iter(
                    matches[..]
                        .iter()
                        .filter(|game_match| {
                            &game_match.black_player_id == id || &game_match.white_player_id == id
                        })
                        .map(|game_match| {
                            if &game_match.black_player_id == id {
                                return game_match.white_player_id;
                            }
                            game_match.black_player_id
                        })
                );
                (id.clone(), opponent_ids)
            })
    )
}

fn get_sorted_player_standings(player_ids: &HashSet<i32>, matches: &Vec<GameMatch>) -> Vec<PlayerStanding> {
    let major_scores_by_id = get_major_scores_by_player_id(player_ids, matches);
    let mut standings: Vec<PlayerStanding> = player_ids
        .iter()
        .map(|id| {
            let filtered_matches: Vec<GameMatch> = matches[..]
                .iter()
                .filter(|game_match| {
                    &game_match.black_player_id == id || &game_match.white_player_id == id
                })
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


fn get_major_scores_by_player_id(player_ids: &HashSet<i32>, matches: &Vec<GameMatch>) -> HashMap<i32, f64> {
    HashMap::from_iter(
        player_ids
            .iter()
            .map(|id| (id.clone(), calculate_major_score(id, matches)))
    )
}


fn calculate_major_score(player_id: &i32, matches: &Vec<GameMatch>) -> f64 {
    matches
        .iter()
        .map(|game_match| {
            if !(
                &game_match.black_player_id == player_id
                    || &game_match.white_player_id == player_id
            ) {
                return 0.0;
            }

            if !game_match.is_finished() {
                return 0.0;
            }

            if game_match.is_bye() {
                return 1.0;
            }

            if game_match.black_score == game_match.white_score {
                return 0.5;
            }

            if &game_match.black_player_id == player_id
                && game_match.black_score > game_match.white_score {
                return 1.0;
            }

            if &game_match.white_player_id == player_id
                && game_match.white_score > game_match.black_score {
                return 1.0;
            }

            0.0
        })
        .sum()
}

fn calculate_minor_score(
    player_id: &i32,
    matches: &Vec<GameMatch>,
    major_scores_by_player_ids: &HashMap<i32, f64>,
) -> f64 {
    // Following https://www.worldothello.org/about/world-othello-championship/woc-rules
    let brightwell_constant = f64::from_str(
        &env::var("BRIGHTWELL_CONSTANT").unwrap()[..]
    ).unwrap();
    matches
        .iter()
        .map(|game_match| {
            if !(&game_match.black_player_id == player_id || &game_match.white_player_id == player_id) {
                return 0.0;
            }

            if game_match.is_bye() || !game_match.is_finished() {
                let self_major_score = major_scores_by_player_ids
                    .get(player_id)
                    .unwrap_or(&0.0);
                return 32.0 + brightwell_constant * self_major_score;
            }

            let opponent_player_id = match &game_match.black_player_id == player_id {
                true => game_match.white_player_id,
                false => game_match.black_player_id
            };
            let opponent_major_score = major_scores_by_player_ids
                .get(&opponent_player_id)
                .unwrap_or(&0.0);

            let disc_count = match &game_match.black_player_id == player_id {
                true => f64::from(game_match.black_score),
                false => f64::from(game_match.white_score)
            };

            disc_count + brightwell_constant * opponent_major_score
        })
        .sum()
}

#[cfg(test)]
mod tests {
    mod test_get_standings {
        use std::env;
        use std::str::FromStr;

        use serde_json::{Map, Value};

        use crate::tournament_manager::{
            GameMatch,
            IGameMatch,
            IResultKeeper,
            ResultKeeper,
        };

        #[test]
        fn test_standard() {
            let game_matches = vec![
                GameMatch {
                    round_id: 1,
                    black_player_id: 1,
                    white_player_id: 2,
                    black_score: 40,
                    white_score: 24,
                    meta_data: Value::from(Map::new()),
                },
                GameMatch::create_new_bye(1, 3, Map::new()),
                GameMatch {
                    round_id: 2,
                    black_player_id: 1,
                    white_player_id: 3,
                    black_score: 30,
                    white_score: 34,
                    meta_data: Value::from(Map::new()),
                },
            ];
            let result_keeper = ResultKeeper::from_matches(game_matches.clone());
            let standings = result_keeper.get_detailed_standings();
            let brightwell_constant = f64::from_str(
                &env::var("BRIGHTWELL_CONSTANT").unwrap()[..]
            ).unwrap();

            assert_eq!(standings[0].player_id, 3);
            assert_eq!(standings[0].major_score, 2.0);
            assert_eq!(standings[0].minor_score, 66.0 + brightwell_constant * 3.0);
            assert_eq!(standings[0].match_history.len(), 2);
            assert_eq!(standings[0].match_history[0], game_matches[1]);
            assert_eq!(standings[0].match_history[1], game_matches[2]);

            assert_eq!(standings[1].player_id, 1);
            assert_eq!(standings[1].major_score, 1.0);
            assert_eq!(standings[1].minor_score, 70.0 + brightwell_constant * 2.0);
            assert_eq!(standings[1].match_history.len(), 2);
            assert_eq!(standings[1].match_history[0], game_matches[0]);
            assert_eq!(standings[1].match_history[1], game_matches[2]);

            assert_eq!(standings[2].player_id, 2);
            assert_eq!(standings[2].major_score, 0.0);
            assert_eq!(standings[2].minor_score, 24.0 + brightwell_constant * 1.0);
            assert_eq!(standings[2].match_history.len(), 1);
            assert_eq!(standings[2].match_history[0], game_matches[0]);
        }


    }
    mod test_has_player_met {
        use serde_json::{Map, Value};

        use crate::tournament_manager::{
            GameMatch,
            IGameMatch,
            IResultKeeper,
            ResultKeeper,
        };

        #[test]
        fn test_standard() {
            let game_matches = vec![
                GameMatch {
                    round_id: 1,
                    black_player_id: 1,
                    white_player_id: 2,
                    black_score: 40,
                    white_score: 24,
                    meta_data: Value::from(Map::new()),
                },
                GameMatch::create_new_bye(1, 3, Map::new()),
                GameMatch {
                    round_id: 2,
                    black_player_id: 1,
                    white_player_id: 3,
                    black_score: 30,
                    white_score: 34,
                    meta_data: Value::from(Map::new()),
                },
            ];
            let result_keeper = ResultKeeper::from_matches(game_matches.clone());
            assert_eq!(result_keeper.has_players_met(&1, &2), true);
            assert_eq!(result_keeper.has_players_met(&1, &3), true);
            assert_eq!(result_keeper.has_players_met(&2, &3), false);
            assert_eq!(result_keeper.has_players_met(&2, &1), true);
            assert_eq!(result_keeper.has_players_met(&3, &1), true);
            assert_eq!(result_keeper.has_players_met(&3, &2), false);
        }

    }
}
