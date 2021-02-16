use serde_json::Value;

use crate::database_models::MatchRowModel;
use crate::properties::SpecialConditionScore;

use super::{ByeGameMatch, IGameMatch, NormalGameMatch, UnfinishedGameMatch};

pub struct GameMatchTransformer {}

impl GameMatchTransformer {
    pub fn transform_to_game_match(match_model: &MatchRowModel) -> Box<dyn IGameMatch> {
        if match_model.black_score == SpecialConditionScore::NotFinished.to_i32() {
            return Box::from(
                UnfinishedGameMatch {
                    round_id: match_model.round_id.clone(),
                    black_player_id: match_model.black_player_id.clone(),
                    white_player_id: match_model.white_player_id.clone(),
                    meta_data: match_model.meta_data.clone(),
                }
            );
        }

        if match_model.black_score == SpecialConditionScore::Bye.to_i32() {
            return Box::from(
                ByeGameMatch {
                    round_id: match_model.round_id.clone(),
                    player_id: match_model.black_player_id.clone(),
                    meta_data: match_model.meta_data.clone(),
                }
            );
        }
        Box::from(
            NormalGameMatch {
                round_id: match_model.round_id.clone(),
                black_player_id: match_model.black_player_id.clone(),
                white_player_id: match_model.white_player_id.clone(),
                black_score: match_model.black_score.clone(),
                white_score: match_model.white_score.clone(),
                meta_data: match_model.meta_data.clone(),
            }
        )
    }

    pub fn transform_to_match_model_data(game_match: &Box<dyn IGameMatch>) -> MatchRowModel {
        game_match.extract_data()
    }
}


pub struct GameMatchCreator {}

impl GameMatchCreator {
    pub fn create_new_match(
        round_id: &i32,
        black_player_id: &i32,
        white_player_id: &i32,
        meta_data: &Value,
    ) -> Box<dyn IGameMatch> {
        return Box::from(
            UnfinishedGameMatch {
                round_id: round_id.clone(),
                black_player_id: black_player_id.clone(),
                white_player_id: white_player_id.clone(),
                meta_data: meta_data.clone(),
            }
        );
    }

    pub fn create_new_bye_match(
        round_id: &i32,
        player_id: &i32,
        meta_data: &Value,
    ) -> Box<dyn IGameMatch> {
        return Box::from(
            ByeGameMatch {
                round_id: round_id.clone(),
                player_id: player_id.clone(),
                meta_data: meta_data.clone(),
            }
        );
    }

    pub fn create_new_finished_match(
        round_id: &i32,
        black_player_id: &i32,
        white_player_id: &i32,
        black_score: &i32,
        white_score: &i32,
        meta_data: &Value,
    ) -> Box<dyn IGameMatch> {
        return Box::from(
            NormalGameMatch {
                round_id: round_id.clone(),
                black_player_id: black_player_id.clone(),
                white_player_id: white_player_id.clone(),
                black_score: black_score.clone(),
                white_score: white_score.clone(),
                meta_data: meta_data.clone(),
            }
        );
    }
}
