use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn generate_deterministic_id(input: &str) -> String {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let hash = hasher.finish();

    // Convert hash to a 10-character alphanumeric string
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .collect();

    let mut result = String::with_capacity(10);
    let mut hash_value = hash;

    for _ in 0..10 {
        let index = (hash_value % chars.len() as u64) as usize;
        result.push(chars[index]);
        hash_value = hash_value.wrapping_mul(31).wrapping_add(1);
    }

    result
}
