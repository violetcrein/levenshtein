mod library;
mod ruby;

use crate::ruby::find_word;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut end: usize = args.len();
    let mut start: usize = 1;
    let mut limit: usize = 8;

    if args[1] == "-l" && args[2].parse::<usize>().is_ok() {
        start = 3;
        limit = args[2].parse::<usize>().unwrap();
    }
    else if args[args.len() - 2] == "-l" && args[args.len() - 1].parse::<usize>().is_ok() {
        end = args.len() - 2;
        limit = args[args.len() - 1].parse::<usize>().unwrap();
    }

    if args[1] == "-f" {
        start = 2;
    }
    
    // adding multiple words to the cli
    for j in start..end {
        print!("{}: ", &args[j]);
        let output = find_word(&args[j].to_lowercase());

        // limit size of output
        let limit: usize = if output.len() >= limit { limit } else { output.len() };

        for i in 0..limit {
            let end = if i == limit - 1 || (output[i].1 == 0.0 && args[1] != "-f") { "" } else { ", " };
            print!("{}{}", output[i].0, end);

            if output[i].1 == 0.0 && args[1] != "-f" {
                break;
            }
        }
        println!("");
    }
}