use crate::game_match::IGameMatch;
use crate::tournament_manager::{IResultKeeper, Player};

pub trait PairingGenerator {
    fn generate_pairings(
        &self,
        players: Vec<Player>,
        past_results: dyn IResultKeeper,
    ) -> Vec<dyn IGameMatch>;
}
