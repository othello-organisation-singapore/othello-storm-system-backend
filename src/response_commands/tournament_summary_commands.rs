use std::collections::HashMap;

use diesel::PgConnection;
use rocket_contrib::json::JsonValue;
use serde_json::{Map, Value};

use crate::database_models::{
    MatchDAO, MatchRowModel, PlayerRowModel, RoundDAO, RoundRowModel, TournamentRowModel,
};
use crate::errors::ErrorType;
use crate::game_match::GameMatchTransformer;
use crate::meta_generator::{
    MatchMetaGenerator, MatchSummaryMetaGenerator, RoundMetaGenerator, RoundSummaryMetaGenerator,
    StandingMetaGenerator, StandingSummaryMetaGenerator, TournamentMetaGenerator,
    TournamentSummaryMetaGenerator,
};
use crate::tournament_manager::create_result_keeper;

use super::ResponseCommand;

pub struct GetTournamentSummaryCommand {
    pub id: i32,
}

impl GetTournamentSummaryCommand {
    fn get_tournament_info_summary_meta(
        &self,
        tournament: &TournamentRowModel,
    ) -> Map<String, Value> {
        let tournament_meta_generator = TournamentSummaryMetaGenerator {};
        tournament_meta_generator.generate_meta_for(&tournament)
    }

    fn get_standings_summary_meta(
        &self,
        players_by_id: &HashMap<&i32, &PlayerRowModel>,
        game_matches: &Vec<MatchRowModel>,
    ) -> Vec<Map<String, Value>> {
        let standing_meta_generator = StandingSummaryMetaGenerator {
            players_by_id: players_by_id.clone(),
        };

        let transformed_matches = game_matches
            .iter()
            .map(|game_match| GameMatchTransformer::transform_to_game_match(game_match))
            .collect();
        let result_keeper = create_result_keeper(&transformed_matches);
        let standings = result_keeper.get_detailed_standings();
        standings
            .iter()
            .map(|player_standing| standing_meta_generator.generate_meta_for(player_standing))
            .collect()
    }

    fn get_rounds_summary_meta(
        &self,
        players_by_id: &HashMap<&i32, &PlayerRowModel>,
        rounds: &Vec<RoundRowModel>,
        game_matches: &Vec<MatchRowModel>,
    ) -> Vec<Map<String, Value>> {
        let round_meta_generator = RoundSummaryMetaGenerator {};
        let match_meta_generator = MatchSummaryMetaGenerator {
            players_by_id: players_by_id.clone(),
        };

        rounds
            .iter()
            .map(|round| {
                let mut round_summary_meta = round_meta_generator.generate_meta_for(round);
                let round_matches_meta: Vec<Map<String, Value>> = game_matches
                    .iter()
                    .filter(|game_match| &game_match.round_id == &round.id)
                    .map(|game_match| match_meta_generator.generate_meta_for(game_match))
                    .collect();
                round_summary_meta.insert(String::from("matches"), Value::from(round_matches_meta));
                round_summary_meta
            })
            .collect()
    }
}

impl ResponseCommand for GetTournamentSummaryCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let tournament = TournamentRowModel::get(&self.id, connection)?;
        let players = PlayerRowModel::get_all_from_tournament(&self.id, connection)?;

        let mut players_by_id: HashMap<&i32, &PlayerRowModel> = HashMap::new();
        players.iter().for_each(|player| {
            players_by_id.insert(&player.id, player.clone());
        });

        let rounds = RoundRowModel::get_all_from_tournament(&self.id, connection)?;
        let game_matches = MatchRowModel::get_all_from_tournament(&self.id, connection)?;

        Ok(json!({
            "tournament_info": self.get_tournament_info_summary_meta(&tournament),
            "standings": self.get_standings_summary_meta(&players_by_id, &game_matches),
            "rounds": self.get_rounds_summary_meta(&players_by_id, &rounds, &game_matches),
        }))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!("GetTournamentSummary for {}", &self.id))
    }
}
