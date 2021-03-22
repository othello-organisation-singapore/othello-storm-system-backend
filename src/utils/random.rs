extern crate rand;

use mocktopus::macros::mockable;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn generate_random_string(length: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric).take(length).collect()
}

pub fn generate_random_number() -> i32 {
    let mut rng = thread_rng();
    rng.gen()
}

#[mockable]
pub fn generate_random_number_ranged(low_ex: i32, hi_in: i32) -> i32 {
    let mut rng = thread_rng();
    rng.gen_range(low_ex, hi_in)
}
