use crate::library::levenshtein::{lev_distance};
use crate::library::keyboard::{Keyboard, euc_distance};
use std::io::Result;

pub fn distance_lev(keyboard: Keyboard, a: &str, b: &str) -> Result<f64> {
    let levenshtein: u32 = lev_distance(a, b).expect("Error in the Levenshtein");
    let mut key_distance: f64 = 0.0;
    let mut b_chars = b.chars();
    
    let greater: usize = if a.len() < b.len() { a.len() } else { b.len() };
    
    for i in 0..greater {
        if i > a.len() || i > b.len() {
            key_distance += 0.5;
        }
        else if i >= 2 && b_chars.nth(i-2) == b_chars.nth(i-1) && b_chars.nth(i-1) == b_chars.nth(i) {
            key_distance -= 0.25;
        }
        else if a.chars().nth(i) != b.chars().nth(i) {
            key_distance += euc_distance(&keyboard, a.chars().nth(i).unwrap(), b.chars().nth(i).unwrap()).unwrap();
        }
    }

    Ok(key_distance + levenshtein as f64)
}