use serde_json::{Map, Value};

use crate::database_models::{
    MatchRowModel, PlayerRowModel, RoundRowModel, TournamentRowModel, UserRowModel,
};
use crate::meta_generator::StandingMetaGenerator;
use crate::tournament_manager::PlayerStanding;

use super::{
    MatchMetaGenerator, MetaGenerator, PlayerMetaGenerator, RoundPreviewMetaGenerator,
    TournamentPreviewMetaGenerator, UserMetaGenerator,
};

pub fn generate_players_meta(player_models: Vec<PlayerRowModel>) -> Vec<Map<String, Value>> {
    player_models
        .into_iter()
        .map(|player| {
            let meta_generator = PlayerMetaGenerator::from_player_model(player);
            meta_generator.generate_meta()
        })
        .collect()
}

pub fn generate_tournaments_meta(
    tournament_models: Vec<TournamentRowModel>,
) -> Vec<Map<String, Value>> {
    tournament_models
        .into_iter()
        .map(|tournament| {
            let meta_generator = TournamentPreviewMetaGenerator::from_tournament(tournament);
            meta_generator.generate_meta()
        })
        .collect()
}

pub fn generate_users_meta(user_models: Vec<UserRowModel>) -> Vec<Map<String, Value>> {
    user_models
        .into_iter()
        .map(|user| {
            let meta_generator = UserMetaGenerator::from_user(user);
            meta_generator.generate_meta()
        })
        .collect()
}

pub fn generate_rounds_meta(round_models: Vec<RoundRowModel>) -> Vec<Map<String, Value>> {
    round_models
        .into_iter()
        .map(|round| {
            let meta_generator = RoundPreviewMetaGenerator::from_round_model(round);
            meta_generator.generate_meta()
        })
        .collect()
}

pub fn generate_matches_meta(match_models: Vec<MatchRowModel>) -> Vec<Map<String, Value>> {
    match_models
        .into_iter()
        .map(|game_match| {
            let meta_generator = MatchMetaGenerator::from_match_model(game_match);
            meta_generator.generate_meta()
        })
        .collect()
}

pub fn generate_standings_meta(standings: Vec<PlayerStanding>) -> Vec<Map<String, Value>> {
    standings
        .into_iter()
        .map(|player_standing| {
            let meta_generator = StandingMetaGenerator::from_standing(player_standing);
            meta_generator.generate_meta()
        })
        .collect()
}
