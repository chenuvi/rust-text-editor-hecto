use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Read};

fn main() {
    enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        println!("char: {}", c);
        if c == 'q' {
            disable_raw_mode().unwrap();
            break;
        }
    }
}