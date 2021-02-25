use crate::properties::TournamentType;

use super::{PairingGenerator, SwissPairingsGenerator};

pub struct PairingsGeneratorCreator {}

impl PairingsGeneratorCreator {
    pub fn create_automatic_pairings_generator(
        tournament_type: TournamentType,
    ) -> Box<dyn PairingGenerator> {
        match tournament_type {
            TournamentType::SwissPairing => Box::from(SwissPairingsGenerator {}),
            _ => unimplemented!()
        }
    }
}
