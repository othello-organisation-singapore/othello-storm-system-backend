use std::cmp::Ordering::Equal;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use super::{IGameMatch, GameMatch};

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
                player_ids.insert(game_match.black_player_id);
                player_ids.insert(game_match.white_player_id);
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
                &6.0,
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
            if &game_match.black_player_id == player_id {
                if game_match.black_score > game_match.white_score {
                    return 1.0;
                } else if game_match.black_score == game_match.white_score {
                    return 0.5;
                }
            }

            if &game_match.white_player_id == player_id {
                if game_match.black_score < game_match.white_score {
                    return 1.0;
                } else if game_match.black_score == game_match.white_score {
                    return 0.5;
                }
            }
            0.0
        })
        .sum()
}

fn calculate_minor_score(
    player_id: &i32,
    matches: &Vec<GameMatch>,
    major_scores_by_player_ids: &HashMap<i32, f64>,
    brightwell_constant: &f64,
) -> f64 {
    // Following https://www.worldothello.org/about/world-othello-championship/woc-rules
    matches
        .iter()
        .map(|game_match| {
            if &game_match.black_player_id == player_id {
                if game_match.is_bye() || game_match.is_finished() {
                    let self_major_score = major_scores_by_player_ids
                        .get(player_id)
                        .unwrap_or(&0.0);
                    return brightwell_constant * self_major_score;
                }
                let opponent_major_score = major_scores_by_player_ids
                    .get(&game_match.white_player_id)
                    .unwrap_or(&0.0);
                return brightwell_constant * opponent_major_score;
            }

            if &game_match.white_player_id == player_id {
                if game_match.is_bye() || game_match.is_finished() {
                    let self_major_score = major_scores_by_player_ids
                        .get(player_id)
                        .unwrap_or(&0.0);
                    return brightwell_constant * self_major_score;
                }
                let opponent_major_score = major_scores_by_player_ids
                    .get(&game_match.black_player_id)
                    .unwrap_or(&0.0);
                return brightwell_constant * opponent_major_score;
            }
            0.0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::tournament_manager::{IGameMatch, GameMatch, ResultKeeper};

    mod test_get_standings {}

    mod test_has_player_met {
    }
}
