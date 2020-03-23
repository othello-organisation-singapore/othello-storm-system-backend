extern crate rand;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn generate_random_string(length: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric).take(length).collect()
}

pub fn generate_random_number() -> i32 {
    let mut rng = thread_rng();
    rng.gen::<i32>()
}
