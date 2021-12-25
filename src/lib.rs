use std::io::{self, BufRead};

pub mod core;
pub mod re;
pub mod stack;
pub mod token;
pub mod compile;

pub fn read_line() -> Option<String> {
    let stdin: io::Stdin = io::stdin();
    let mut handle: io::Lines<io::StdinLock> = stdin.lock().lines();

    match handle.next() {
        Some(result) => match result {
            Ok(str) => Some(str),
            Err(_) => None,
        },
        None => None,
    }
}