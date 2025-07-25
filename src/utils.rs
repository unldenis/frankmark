use rand::{Rng, distr::Alphanumeric}; // 0.8

pub fn generate_id(chars: usize) -> String {
    let s: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(chars)
        .map(char::from)
        .collect();

    s
}
