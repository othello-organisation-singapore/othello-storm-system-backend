use diesel::PgConnection;
use rocket_contrib::json::JsonValue;
use serde_json::{Map, Value};

use crate::account::Account;
use crate::database_models::{MatchDAO, MatchRowModel, RoundDAO, RoundRowModel};
use crate::errors::ErrorType;
use crate::tournament_manager::GameMatch;

use super::{generate_rounds_meta, generate_matches_meta, is_allowed_to_manage_tournament};
use super::ResponseCommand;
use crate::meta_generator::{RoundDetailsMetaGenerator, MetaGenerator};

pub struct GetTournamentRoundsCommand {
    pub tournament_id: i32,
}

impl ResponseCommand for GetTournamentRoundsCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let rounds = RoundRowModel::get_all_from_tournament(
            &self.tournament_id,
            connection,
        )?;
        let rounds_meta = generate_rounds_meta(rounds);
        Ok(json!({
            "tournament_id": &self.tournament_id,
            "rounds": rounds_meta,
        }))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!("GetTournamentRounds for {}", &self.tournament_id))
    }
}

pub struct GetRoundCommand {
    pub round_id: i32,
}

impl ResponseCommand for GetRoundCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let round = RoundRowModel::get(&self.round_id, connection)?;
        let tournament_id = round.tournament_id.clone();
        let matches = MatchRowModel::get_all_from_round(&self.round_id, connection)?;

        let round_meta_generator = RoundDetailsMetaGenerator::from_round_model(round);
        let mut round_meta = round_meta_generator.generate_meta();
        let matches_meta = generate_matches_meta(matches);

        round_meta.insert(String::from("matches"), Value::from(matches_meta));
        Ok(json!({
            "tournament_id": tournament_id,
            "round": round_meta,
        }))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!("GetRound for {}", &self.round_id))
    }
}
