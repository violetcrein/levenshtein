use std::{fs::File, io::{BufReader, Result, BufRead}};

use crate::library::keyboard::qwerty;
use crate::library::distance::distance_lev;

pub fn find_word(a: &str) -> Vec<(String, f64)> {
    let mut possibilities: Vec<(String, f64)> = Vec::new();
    let word_list = load_file("./10k.txt").unwrap();

    for i in word_list {
        let distance = distance_lev(qwerty(), a, &i).unwrap();
        
        if distance == 0.0 {
            possibilities.push((format!("\x1b[32;1m{}\x1b[31;0m", i), distance));
            break;
        }
        else if distance <= 1.5 {
            possibilities.push((format!("\x1b[33;1m{}\x1b[31;0m", i), distance));
        }
        else if distance <= 4.0 {
            possibilities.push((format!("\x1b[31;1m{}\x1b[31;0m", i), distance));
        }
    }

    possibilities.sort_by(|f, j| (f.1).partial_cmp(&j.1).unwrap());
    possibilities
}

fn load_file(filename: &str) -> Result<Vec<String>> {
    let file = File::open(filename).expect("File not found.");
    let reader = BufReader::new(file);

    // let conditional = |l: String, s: usize| {
    //     if l.len() + 3 > s && s + 3 > l.len() { l } else { String::from("") }
    // };
    Ok(
        reader.lines()
            .map(|l| l.unwrap())
            .collect()
    )
}