use crate::database_models::PlayerRowModel;
use crate::game_match::IGameMatch;
use crate::tournament_manager::IResultKeeper;

pub trait PairingGenerator {
    fn generate_pairings(
        &self,
        round_id: &i32,
        players: &Vec<PlayerRowModel>,
        past_results: &Box<dyn IResultKeeper>,
    ) -> Vec<Box<dyn IGameMatch>>;
}
