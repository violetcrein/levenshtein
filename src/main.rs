mod compare;
mod library;
mod find_word;
mod util;

use crate::find_word::find_word;
use clap::Parser;
use compare::compare;
use util::color;

#[derive(Parser, Debug)]
#[command(author = "viowo", version, about = "A simple program to understand WHAT THE FUCK RUBY IS SAYING~", long_about = None)]
struct Args {
    /// Text to check and cmatch
    #[arg(short, long)]
    text: String,

    /// Compare text to string (instead of finding matching word)
    #[arg(short, long, )]
    compare: Option<String>,

    /// Number of options to send
    #[arg(short, default_value_t = 8)]
    length: usize,

    /// Full even if matched
    #[arg(short, default_value_t = false)]
    full: bool,
}

fn main() {
    let args = Args::parse();
    let text = args.text.split(" ").collect::<Vec<&str>>();

    // if you have the compare option on
    if args.compare != None {
        let c_output = compare(&args.text, &args.compare.unwrap());
        println!("Distance   : {}", color(c_output.0, (3.0, 6.0, 10.0)));
        println!("Levenshtein: {}", color(c_output.1 as f64, (3.0, 6.0, 10.0)));
        println!("Keyboard   : {}", color(c_output.2, (3.0, 6.0, 10.0)));
    }
    else {
        // multiple words
        for j in 0..text.len() {
            print!("{}: ", &text[j]);
            let output = find_word(&text[j].to_lowercase());

            // limit size of output
            let limit: usize = if output.len() >= args.length { args.length } else { output.len() };

            for i in 0..limit {
                let end = if i == limit - 1 || (output[i].1 == 0.0 && !args.full) { "" } else { ", " };
                print!("{}{}", output[i].0, end);

                if output[i].1 == 0.0 && !args.full {
                    break;
                }
            }
            println!("");
        }
    }
}