extern crate crypto;

use self::crypto::md5::Md5;
use self::crypto::digest::Digest;

use std::i32;

const MAX: i32 = i32::MAX;

fn solve(word: &str) {
    println!("Solving for \"{}\"", word);

    for n in 1..MAX {
        let string = format!("{}{}", word, n);
        let mut md5 = Md5::new();
        md5.input(string.as_bytes());
        let res = md5.result_str();

        if res.starts_with("000000") {
            println!("Solution: {}", n);
            return;
        }
    }
    println!("Found no solution.");
}

pub fn day_4(input: String) {
    println!("hehe");
    for line in input.lines() {
        solve(line);
    }
}
