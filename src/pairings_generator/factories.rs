use crate::database_models::TournamentRowModel;
use crate::properties::TournamentType;

use super::{PairingGenerator, SwissPairingsGenerator};

pub struct PairingsGeneratorCreator {}

impl PairingsGeneratorCreator {
    pub fn create_automatic_pairings_generator(
        tournament_type: TournamentType,
    ) -> Box<dyn PairingsGenerator> {
        match tournament_type {
            TournamentType::SwissPairing => Box::from(SwissPairingsGenerator {}),
            _ => unimplemented!()
        }
    }
}
