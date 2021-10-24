use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::database_models::{
    MatchRowModel, PlayerRowModel, RoundRowModel, TournamentRowModel, UserRowModel,
};
use crate::meta_generator::RoundMetaGenerator;
use crate::tournament_manager::PlayerStanding;

use super::{
    DefaultMatchMetaGenerator, DefaultStandingMetaGenerator, MatchMetaGenerator, MetaGenerator,
    PlayerMetaGenerator, RoundPreviewMetaGenerator, StandingMetaGenerator, TournamentMetaGenerator,
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
    user_models: Vec<UserRowModel>,
) -> Vec<Map<String, Value>> {
    let mut users_by_username: HashMap<&str, &UserRowModel> = HashMap::new();
    user_models.iter().for_each(|user| {
        users_by_username.insert(user.username.as_str(), user.clone());
    });
    let meta_generator = TournamentPreviewMetaGenerator { users_by_username };
    tournament_models
        .into_iter()
        .map(|tournament| meta_generator.generate_meta_for(&tournament))
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
    let meta_generator = RoundPreviewMetaGenerator {};
    round_models
        .into_iter()
        .map(|round| meta_generator.generate_meta_for(&round))
        .collect()
}

pub fn generate_matches_meta(match_models: Vec<MatchRowModel>) -> Vec<Map<String, Value>> {
    let meta_generator = DefaultMatchMetaGenerator {};
    match_models
        .into_iter()
        .map(|game_match| meta_generator.generate_meta_for(&game_match))
        .collect()
}

pub fn generate_standings_meta(standings: Vec<PlayerStanding>) -> Vec<Map<String, Value>> {
    let meta_generator = DefaultStandingMetaGenerator {};
    standings
        .into_iter()
        .map(|player_standing| meta_generator.generate_meta_for(&player_standing))
        .collect()
}
