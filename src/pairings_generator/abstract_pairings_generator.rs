use crate::errors::ErrorType;
use crate::game_match::IGameMatch;

pub type Pairings = Vec<Box<dyn IGameMatch>>;

pub trait PairingGenerator {
    fn generate_pairings(&self, round_id: &i32) -> Result<Pairings, ErrorType>;
}
