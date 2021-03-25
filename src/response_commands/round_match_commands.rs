use std::collections::HashSet;

use diesel::PgConnection;
use rocket::http::Cookies;
use rocket_contrib::json::JsonValue;
use serde_json::{Map, Value};

use crate::account::Account;
use crate::database_models::{MatchDAO, MatchRowModel, PlayerRowModel, RoundDAO, RoundRowModel, TournamentRowModel};
use crate::errors::ErrorType;
use crate::game_match::GameMatchCreator;
use crate::meta_generator::{MetaGenerator, RoundDetailsMetaGenerator};
use crate::properties::RoundType;

use super::{generate_matches_meta, generate_rounds_meta, is_allowed_to_manage_tournament};
use super::ResponseCommand;

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

pub struct CreateManualNormalRoundCommand<'a> {
    pub cookies: Cookies<'a>,
    pub tournament_id: i32,
    pub name: String,
    pub meta_data: Map<String, Value>,
    pub match_data: Vec<(i32, i32)>,
}

impl CreateManualNormalRoundCommand<'_> {
    fn is_match_data_invalid(&self, connection: &PgConnection) -> bool {
        let mut player_ids = HashSet::new();
        PlayerRowModel::get_all_from_tournament(&self.tournament_id, connection)
            .unwrap_or(Vec::new())
            .iter()
            .for_each(|player_model| {
                player_ids.insert(player_model.id.clone());
            });
        return self.match_data
            .iter()
            .find(|match_datum| {
                !(player_ids.contains(&match_datum.0)
                    && player_ids.contains(&match_datum.1))
            })
            .is_some();
    }
}

impl ResponseCommand for CreateManualNormalRoundCommand<'_> {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let account = Account::login_from_cookies(&self.cookies, connection)?;
        let tournament_model = TournamentRowModel::get(&self.tournament_id, connection)?;

        let is_allowed_to_manage = is_allowed_to_manage_tournament(
            &account,
            &tournament_model,
            connection
        )?;
        if !is_allowed_to_manage {
            return Err(ErrorType::PermissionDenied);
        }

        if self.is_match_data_invalid(connection) {
            return Err(ErrorType::BadRequestError(
                String::from("Invalid match data, some players are not available")
            ));
        }
        let round = RoundRowModel::create(
            &self.tournament_id,
            &self.name,
            RoundType::ManualNormal,
            Map::new(),
            connection,
        )?;
        let pairings = self.match_data
            .iter()
            .map(|match_datum| GameMatchCreator::create_new_match(
                &round.id,
                &match_datum.0,
                &match_datum.1,
                &Value::from(Map::new()),
            ))
            .collect();
        MatchRowModel::bulk_create_from(&pairings, connection)?;

        Ok(json!({"message": "Manual Normal Round Pairing is added to tournament"}))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!("CreateManualRound for {}", &self.tournament_id))
    }
}
