use crate::database_models::PlayerRowModel;
use crate::properties::TournamentType;
use crate::tournament_manager::IResultKeeper;

use super::{PairingGenerator, SwissPairingsGenerator, RRPairingsGenerator};

pub struct PairingsGeneratorCreator {}

impl PairingsGeneratorCreator {
    pub fn create_automatic_pairings_generator(
        tournament_type: TournamentType,
        players: Vec<PlayerRowModel>,
        past_results: Box<dyn IResultKeeper>
    ) -> Box<dyn PairingGenerator> {
        match tournament_type {
            TournamentType::SwissPairing => Box::from(SwissPairingsGenerator::new(players, past_results)),
            TournamentType::RoundRobin => Box::from(RRPairingsGenerator::new(players, past_results)),
            _ => unimplemented!()
        }
    }
}
