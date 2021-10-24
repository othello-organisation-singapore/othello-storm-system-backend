use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::database_models::{MatchRowModel, PlayerRowModel, RoundRowModel, TournamentRowModel};
use crate::tournament_manager::PlayerStanding;
use crate::utils::date_to_string;

use super::{
    MatchMetaGenerator, RoundMetaGenerator, StandingMetaGenerator, TournamentMetaGenerator,
};

pub struct TournamentSummaryMetaGenerator {}

impl TournamentMetaGenerator for TournamentSummaryMetaGenerator {
    fn generate_meta_for(&self, tournament: &TournamentRowModel) -> Map<String, Value> {
        let mut meta = Map::new();
        meta.insert(String::from("id"), Value::from(tournament.id.clone()));
        meta.insert(String::from("name"), Value::from(tournament.name.clone()));
        meta.insert(
            String::from("country"),
            Value::from(tournament.country.clone()),
        );
        meta.insert(
            String::from("start_date"),
            Value::from(date_to_string(tournament.start_date.clone())),
        );
        meta.insert(
            String::from("end_date"),
            Value::from(date_to_string(tournament.end_date.clone())),
        );
        meta
    }
}

pub struct StandingSummaryMetaGenerator<'a> {
    pub players_by_id: HashMap<&'a i32, &'a PlayerRowModel>,
}

impl StandingMetaGenerator for StandingSummaryMetaGenerator<'_> {
    fn generate_meta_for(&self, standing: &PlayerStanding) -> Map<String, Value> {
        let mut meta = Map::new();

        let &player = self.players_by_id.get(&standing.player_id).unwrap();
        meta.insert(
            String::from("player_id"),
            Value::from(standing.player_id.clone()),
        );
        meta.insert(
            String::from("joueurs_id"),
            Value::from(player.joueurs_id.clone()),
        );
        meta.insert(
            String::from("first_name"),
            Value::from(player.first_name.clone()),
        );
        meta.insert(
            String::from("last_name"),
            Value::from(player.last_name.clone()),
        );
        meta.insert(String::from("country"), Value::from(player.country.clone()));
        meta.insert(
            String::from("major_score"),
            Value::from(standing.major_score.clone()),
        );
        meta.insert(
            String::from("minor_score"),
            Value::from(standing.minor_score.clone()),
        );
        meta
    }
}

pub struct RoundSummaryMetaGenerator {}

impl RoundMetaGenerator for RoundSummaryMetaGenerator {
    fn generate_meta_for(&self, round: &RoundRowModel) -> Map<String, Value> {
        let mut meta = Map::new();
        meta.insert(String::from("id"), Value::from(round.id.clone()));
        meta.insert(String::from("name"), Value::from(round.name.clone()));
        meta
    }
}

pub struct MatchSummaryMetaGenerator<'a> {
    pub players_by_id: HashMap<&'a i32, &'a PlayerRowModel>,
}

impl MatchMetaGenerator for MatchSummaryMetaGenerator<'_> {
    fn generate_meta_for(&self, game_match: &MatchRowModel) -> Map<String, Value> {
        let mut meta = Map::new();
        meta.insert(String::from("id"), Value::from(game_match.id.clone()));

        let black_player_joueurs_id = self
            .players_by_id
            .get(&game_match.black_player_id)
            .unwrap()
            .joueurs_id
            .clone();
        meta.insert(
            String::from("black_player_id"),
            Value::from(game_match.black_player_id.clone()),
        );
        meta.insert(
            String::from("black_player_joueurs_id"),
            Value::from(black_player_joueurs_id),
        );

        let white_player_joueurs_id = match self.players_by_id.get(&game_match.white_player_id) {
            Some(white_player) => white_player.joueurs_id.clone(),
            None => String::from("BYE"),
        };
        meta.insert(
            String::from("white_player_id"),
            Value::from(game_match.white_player_id.clone()),
        );
        meta.insert(
            String::from("white_player_joueurs_id"),
            Value::from(white_player_joueurs_id),
        );

        meta.insert(
            String::from("black_score"),
            Value::from(game_match.black_score.clone()),
        );
        meta.insert(
            String::from("white_score"),
            Value::from(game_match.white_score.clone()),
        );
        meta
    }
}
