use std::io::Result;

pub struct Keyboard {
    pub keyboard: Vec<Vec<char>>
}

// Euclidean Distance between characters
pub fn euc_distance(keyboard: &Keyboard, a: char, b: char) -> Result<f64> {
    let a_pos = get_pos(&keyboard.keyboard, a);
    let b_pos = get_pos(&keyboard.keyboard, b);
    return Ok(((a_pos.0 - b_pos.0).pow(2) as f64 + (a_pos.1 - b_pos.1).pow(2) as f64).sqrt())
}

fn get_pos(kb: &Vec<Vec<char>>, a: char) -> (i32, i32) {
    let vector_position = kb.iter().position(|r| r.contains(&a)).unwrap();
    let element_position = kb[vector_position].iter().position(|r| r == &a).unwrap();

    (element_position as i32, vector_position as i32)
}

// predefined layouts
pub fn qwerty() -> Keyboard {
    Keyboard{keyboard: vec![
        vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '='],
        vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']'],
        vec!['a', 's', 'd', 'e', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '\\'],
        vec!['`', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/']
    ]}
}