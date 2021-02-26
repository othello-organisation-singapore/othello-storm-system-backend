pub use abstract_pairings_generator::{PairingGenerator, Pairings};
pub use helpers::get_player_1_color;
pub use rr_pairings_generator::RRPairingsGenerator;
pub use swiss_pairings_generator::SwissPairingsGenerator;

mod abstract_pairings_generator;
mod factories;
mod helpers;
mod rr_pairings_generator;
mod swiss_pairings_generator;
