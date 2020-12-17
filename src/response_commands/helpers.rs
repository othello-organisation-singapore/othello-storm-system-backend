use serde_json::{Map, Value};

use crate::database_models::{TournamentRowModel, UserRowModel};
use crate::meta_generator::{
    MetaGenerator,
    TournamentPreviewMetaGenerator,
    UserMetaGenerator,
};

pub fn generate_tournaments_meta(
    tournament_models: Vec<TournamentRowModel>
) -> Vec<Map<String, Value>> {
    tournament_models
        .into_iter()
        .map(|tournament| {
            let meta_generator = TournamentPreviewMetaGenerator::from_tournament(
                tournament
            );
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
