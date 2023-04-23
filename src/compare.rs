use crate::library::{distance::distance_lev, keyboard::qwerty, levenshtein::lev_distance};

pub fn compare(string_a: &str, string_b: &str) -> (f64, u32, f64) {
    let total_distance = distance_lev(qwerty(), string_a, string_b).unwrap();
    let levenshtein_distance = lev_distance(string_a, string_b).unwrap();

    (total_distance, levenshtein_distance, total_distance - levenshtein_distance as f64)
}