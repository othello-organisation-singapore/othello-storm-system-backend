use std::collections::HashSet;
use std::iter::FromIterator;

use diesel::result::Error;
use diesel::{Connection, PgConnection};
use itertools::Itertools;
use rocket_contrib::json::JsonValue;
use serde_json::{Map, Value};

use crate::account::Account;
use crate::database_models::{
    MatchDAO, MatchRowModel, PlayerRowModel, RoundDAO, RoundRowModel, TournamentRowModel,
};
use crate::errors::ErrorType;
use crate::game_match::{GameMatchCreator, GameMatchTransformer, IGameMatch};
use crate::meta_generator::{
    generate_matches_meta, generate_rounds_meta, generate_standings_meta,
    RoundDetailsMetaGenerator, RoundMetaGenerator,
};
use crate::pairings_generator::PairingsGeneratorCreator;
use crate::properties::{RoundType, TournamentType};
use crate::tournament_manager::create_result_keeper;

use super::{is_allowed_to_manage_tournament, ResponseCommand};

pub struct GetTournamentRoundsCommand {
    pub tournament_id: i32,
}

impl ResponseCommand for GetTournamentRoundsCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let rounds = RoundRowModel::get_all_from_tournament(&self.tournament_id, connection)?;
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

        let round_meta_generator = RoundDetailsMetaGenerator {};
        let mut round_meta = round_meta_generator.generate_meta_for(&round);
        let matches_meta = generate_matches_meta(
            matches
                .into_iter()
                .sorted_by_key(|game_match| game_match.id)
                .collect(),
        );

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

pub struct GetStandingsCommand {
    pub round_id_limit: i32,
    pub tournament_id: i32,
}

impl ResponseCommand for GetStandingsCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let tournament_model = TournamentRowModel::get(&self.tournament_id, connection)?;
        let round_ids: HashSet<i32> = HashSet::from_iter(
            RoundRowModel::get_all_from_tournament(&tournament_model.id, connection)?
                .into_iter()
                .filter(|round| {
                    let is_normal_round = round.round_type == RoundType::ManualNormal.to_i32()
                        || round.round_type == RoundType::Automatic.to_i32();
                    let is_round_before_limit = round.id <= self.round_id_limit;
                    is_normal_round && is_round_before_limit
                })
                .map(|round| round.id),
        );

        let previous_matches =
            MatchRowModel::get_all_from_tournament(&tournament_model.id, connection)?;
        let filtered_matches: Vec<Box<dyn IGameMatch>> = previous_matches
            .into_iter()
            .filter(|game_match| round_ids.contains(&game_match.round_id))
            .map(|game_match| GameMatchTransformer::transform_to_game_match(&game_match))
            .collect();
        let result_keeper = create_result_keeper(&filtered_matches);
        let standings = result_keeper.get_detailed_standings();
        let standings_meta = generate_standings_meta(standings);
        Ok(json!({"tournament_id": self.tournament_id, "standings": standings_meta}))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!("GetStandings for {}", &self.tournament_id))
    }
}

pub struct CreateManualNormalRoundCommand {
    pub jwt: String,
    pub tournament_id: i32,
    pub name: String,
    pub match_data: Vec<(i32, i32)>,
    pub bye_match_data: Vec<i32>,
}

impl CreateManualNormalRoundCommand {
    fn is_match_data_valid(&self, connection: &PgConnection) -> bool {
        let mut player_ids = HashSet::new();
        PlayerRowModel::get_all_from_tournament(&self.tournament_id, connection)
            .unwrap_or(Vec::new())
            .iter()
            .for_each(|player_model| {
                player_ids.insert(player_model.id.clone());
            });

        let bye_player_not_in_db = self
            .bye_match_data
            .iter()
            .find(|id| !player_ids.contains(id));

        let player_not_in_db = self.match_data.iter().find(|match_datum| {
            !(player_ids.contains(&match_datum.0) && player_ids.contains(&match_datum.1))
        });

        let no_of_players = player_ids.len();
        if no_of_players % 2 == 1 {
            return player_not_in_db.is_none()
                && self.bye_match_data.len() == 1
                && self.match_data.len() == no_of_players / 2;
        }
        player_not_in_db.is_none()
            && bye_player_not_in_db.is_none()
            && self.bye_match_data.len() == 0
            && self.match_data.len() == no_of_players / 2
    }

    fn create_new_pairings(&self, connection: &PgConnection) -> Result<(), ErrorType> {
        let round = RoundRowModel::create(
            &self.tournament_id,
            &self.name,
            RoundType::ManualNormal,
            Map::new(),
            connection,
        )?;
        let mut pairings: Vec<Box<dyn IGameMatch>> = self
            .match_data
            .iter()
            .map(|match_datum| {
                GameMatchCreator::create_new_match(
                    &round.id,
                    &match_datum.0,
                    &match_datum.1,
                    &Value::from(Map::new()),
                )
            })
            .collect();
        let bye_pairings: Vec<Box<dyn IGameMatch>> = self
            .bye_match_data
            .iter()
            .map(|player_id| {
                GameMatchCreator::create_new_bye_match(
                    &round.id,
                    player_id,
                    &Value::from(Map::new()),
                )
            })
            .collect();

        pairings.extend(bye_pairings);
        MatchRowModel::bulk_create_from(&pairings, connection)?;
        Ok(())
    }
}

impl ResponseCommand for CreateManualNormalRoundCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let account = Account::login_from_jwt(&self.jwt, connection)?;
        let tournament_model = TournamentRowModel::get(&self.tournament_id, connection)?;

        if tournament_model.tournament_type == TournamentType::RoundRobin.to_string() {
            return Err(ErrorType::BadRequestError(String::from(
                "Manual pairing is unavailable for Round Robin tournaments",
            )));
        }

        let is_allowed_to_manage =
            is_allowed_to_manage_tournament(&account, &tournament_model, connection)?;
        if !is_allowed_to_manage {
            return Err(ErrorType::PermissionDenied);
        }

        if !self.is_match_data_valid(connection) {
            return Err(ErrorType::BadRequestError(String::from(
                "Invalid match data, some players are not available",
            )));
        }

        if let Err(_) = connection.transaction::<(), Error, _>(|| {
            match self.create_new_pairings(connection) {
                Ok(()) => Ok(()),
                Err(_) => Err(Error::RollbackTransaction),
            }?;
            Ok(())
        }) {
            return Err(ErrorType::UnknownError(String::from(
                "Error from generating manual pairings",
            )));
        }

        Ok(json!({"message": "New round pairings (Manual Normal) is added to the tournament."}))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!(
            "CreateManualNormalRound for {}",
            &self.tournament_id
        ))
    }
}

pub struct CreateManualSpecialRoundCommand {
    pub jwt: String,
    pub tournament_id: i32,
    pub name: String,
    pub match_data: Vec<(i32, i32)>,
    pub bye_match_data: Vec<i32>,
}

impl CreateManualSpecialRoundCommand {
    fn is_match_data_valid(&self, connection: &PgConnection) -> bool {
        let mut player_ids = HashSet::new();
        PlayerRowModel::get_all_from_tournament(&self.tournament_id, connection)
            .unwrap_or(Vec::new())
            .iter()
            .for_each(|player_model| {
                player_ids.insert(player_model.id.clone());
            });

        let player_not_in_db = self.match_data.iter().find(|match_datum| {
            !(player_ids.contains(&match_datum.0) && player_ids.contains(&match_datum.1))
        });
        let bye_player_not_in_db = self
            .bye_match_data
            .iter()
            .find(|id| !player_ids.contains(id));

        player_not_in_db.is_none() && bye_player_not_in_db.is_none()
    }

    fn create_new_pairings(&self, connection: &PgConnection) -> Result<(), ErrorType> {
        let round = RoundRowModel::create(
            &self.tournament_id,
            &self.name,
            RoundType::ManualSpecial,
            Map::new(),
            connection,
        )?;
        let mut pairings: Vec<Box<dyn IGameMatch>> = self
            .match_data
            .iter()
            .map(|match_datum| {
                GameMatchCreator::create_new_match(
                    &round.id,
                    &match_datum.0,
                    &match_datum.1,
                    &Value::from(Map::new()),
                )
            })
            .collect();
        let bye_pairings: Vec<Box<dyn IGameMatch>> = self
            .bye_match_data
            .iter()
            .map(|player_id| {
                GameMatchCreator::create_new_bye_match(
                    &round.id,
                    player_id,
                    &Value::from(Map::new()),
                )
            })
            .collect();

        pairings.extend(bye_pairings);
        MatchRowModel::bulk_create_from(&pairings, connection)?;
        Ok(())
    }
}

impl ResponseCommand for CreateManualSpecialRoundCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let account = Account::login_from_jwt(&self.jwt, connection)?;
        let tournament_model = TournamentRowModel::get(&self.tournament_id, connection)?;

        let is_allowed_to_manage =
            is_allowed_to_manage_tournament(&account, &tournament_model, connection)?;
        if !is_allowed_to_manage {
            return Err(ErrorType::PermissionDenied);
        }

        if !self.is_match_data_valid(connection) {
            return Err(ErrorType::BadRequestError(String::from(
                "Invalid match data, some players are not available",
            )));
        }

        if let Err(_) = connection.transaction::<(), Error, _>(|| {
            match self.create_new_pairings(connection) {
                Ok(()) => Ok(()),
                Err(_) => Err(Error::RollbackTransaction),
            }?;
            Ok(())
        }) {
            return Err(ErrorType::UnknownError(String::from(
                "Error from generating manual pairings",
            )));
        }

        Ok(json!({"message": "New round pairings (Manual Special) is added to the tournament."}))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!(
            "CreateManualSpecialRound for {}",
            &self.tournament_id
        ))
    }
}

pub struct CreateAutomaticRoundCommand {
    pub jwt: String,
    pub tournament_id: i32,
    pub name: String,
}

impl CreateAutomaticRoundCommand {
    fn create_new_automatic_pairings_round(
        &self,
        tournament_model: &TournamentRowModel,
        connection: &PgConnection,
    ) -> Result<(), ErrorType> {
        let automatic_round_ids: HashSet<i32> = HashSet::from_iter(
            RoundRowModel::get_all_from_tournament(&tournament_model.id, connection)?
                .into_iter()
                .filter(|round| round.round_type == RoundType::Automatic.to_i32())
                .map(|round| round.id),
        );
        let normal_round_ids: HashSet<i32> = HashSet::from_iter(
            RoundRowModel::get_all_from_tournament(&tournament_model.id, connection)?
                .into_iter()
                .filter(|round| round.round_type == RoundType::ManualNormal.to_i32())
                .map(|round| round.id),
        );

        let previous_matches =
            MatchRowModel::get_all_from_tournament(&tournament_model.id, connection)?;
        let previous_normal_matches: Vec<Box<dyn IGameMatch>> = previous_matches
            .into_iter()
            .filter(|game_match| {
                if tournament_model.tournament_type == TournamentType::RoundRobin.to_string() {
                    return automatic_round_ids.contains(&game_match.round_id);
                }
                if tournament_model.tournament_type == TournamentType::SwissPairing.to_string() {
                    return automatic_round_ids.contains(&game_match.round_id)
                        || normal_round_ids.contains(&game_match.round_id);
                }
                false
            })
            .map(|game_match| GameMatchTransformer::transform_to_game_match(&game_match))
            .collect();
        let result_keeper = create_result_keeper(&previous_normal_matches);

        let players = PlayerRowModel::get_all_from_tournament(&tournament_model.id, connection)?;

        let pairing_generator = PairingsGeneratorCreator::create_automatic_pairings_generator(
            TournamentType::from_string(tournament_model.tournament_type.clone()),
            players,
            result_keeper,
        );

        let round = RoundRowModel::create(
            &self.tournament_id,
            &self.name,
            RoundType::Automatic,
            Map::new(),
            connection,
        )?;
        let matches = pairing_generator.generate_pairings(&round.id)?;
        MatchRowModel::bulk_create_from(&matches, connection)?;
        Ok(())
    }
}

impl ResponseCommand for CreateAutomaticRoundCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let account = Account::login_from_jwt(&self.jwt, connection)?;
        let tournament_model = TournamentRowModel::get(&self.tournament_id, connection)?;

        let is_allowed_to_manage =
            is_allowed_to_manage_tournament(&account, &tournament_model, connection)?;
        if !is_allowed_to_manage {
            return Err(ErrorType::PermissionDenied);
        }

        if let Err(_) = connection.transaction::<(), Error, _>(|| {
            match self.create_new_automatic_pairings_round(&tournament_model, connection) {
                Ok(()) => Ok(()),
                Err(_) => Err(Error::RollbackTransaction),
            }?;
            Ok(())
        }) {
            return Err(ErrorType::UnknownError(String::from(
                "Error from generating automatic pairings",
            )));
        }

        Ok(json!({"message": "New round pairings (Automatic) is added to the tournament."}))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!("CreateAutomaticRound for {}", &self.tournament_id))
    }
}

pub struct UpdateRoundCommand {
    pub jwt: String,
    pub tournament_id: i32,
    pub round_id: i32,
    pub updated_name: String,
}

impl ResponseCommand for UpdateRoundCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let account = Account::login_from_jwt(&self.jwt, connection)?;
        let tournament_model = TournamentRowModel::get(&self.tournament_id, connection)?;

        let is_allowed_to_manage =
            is_allowed_to_manage_tournament(&account, &tournament_model, connection)?;
        if !is_allowed_to_manage {
            return Err(ErrorType::PermissionDenied);
        }

        let mut round = RoundRowModel::get(&self.round_id, connection)?;
        round.name = self.updated_name.clone();
        round.update(connection)?;

        Ok(json!({"message": "Round has been updated."}))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!(
            "UpdateRound for {} in tournament {}",
            &self.round_id, &self.tournament_id
        ))
    }
}

pub struct DeleteRoundCommand {
    pub jwt: String,
    pub tournament_id: i32,
    pub round_id: i32,
}

impl ResponseCommand for DeleteRoundCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let account = Account::login_from_jwt(&self.jwt, connection)?;
        let tournament_model = TournamentRowModel::get(&self.tournament_id, connection)?;

        let is_allowed_to_manage =
            is_allowed_to_manage_tournament(&account, &tournament_model, connection)?;
        if !is_allowed_to_manage {
            return Err(ErrorType::PermissionDenied);
        }

        let round = RoundRowModel::get(&self.round_id, connection)?;
        let matches = MatchRowModel::get_all_from_round(&round.id, connection)?;

        if let Err(_) = connection.transaction::<(), Error, _>(|| {
            let matches_delete_result: Vec<bool> = matches
                .into_iter()
                .map(|game_match| game_match.delete(connection).is_err())
                .filter(|is_err| *is_err)
                .collect();

            if matches_delete_result.len() > 0 {
                return Err(Error::RollbackTransaction);
            }

            if let Err(_) = round.delete(connection) {
                return Err(Error::RollbackTransaction);
            }
            Ok(())
        }) {
            return Err(ErrorType::UnknownError(String::from(
                "Error from deleting round",
            )));
        }

        Ok(json!({"message": "Round has been deleted."}))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!(
            "DeleteRound for {} in tournament {}",
            &self.round_id, &self.tournament_id
        ))
    }
}

pub struct GetRoundMatchesCommand {
    pub round_id: i32,
}

impl ResponseCommand for GetRoundMatchesCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let matches = MatchRowModel::get_all_from_round(&self.round_id, connection)?;
        let matches_meta = generate_matches_meta(
            matches
                .into_iter()
                .sorted_by_key(|game_match| game_match.id)
                .collect(),
        );
        Ok(json!({
            "round_id": &self.round_id,
            "matches": matches_meta,
        }))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!("GetRoundMatches for {}", &self.round_id))
    }
}

pub struct UpdateMatchCommand {
    pub jwt: String,
    pub tournament_id: i32,
    pub match_id: i32,
    pub black_score: i32,
    pub white_score: i32,
}

impl ResponseCommand for UpdateMatchCommand {
    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType> {
        let account = Account::login_from_jwt(&self.jwt, connection)?;
        let tournament_model = TournamentRowModel::get(&self.tournament_id, connection)?;

        let is_allowed_to_manage =
            is_allowed_to_manage_tournament(&account, &tournament_model, connection)?;
        if !is_allowed_to_manage {
            return Err(ErrorType::PermissionDenied);
        }

        let mut game_match = MatchRowModel::get(&self.match_id, connection)?;
        game_match.black_score = self.black_score.clone();
        game_match.white_score = self.white_score.clone();
        game_match.update(connection)?;

        Ok(json!({"message": "Match has been updated."}))
    }

    fn get_request_summary(&self) -> String {
        String::from(format!(
            "UpdateMatch for {} in tournament {}",
            &self.match_id, &self.tournament_id
        ))
    }
}
