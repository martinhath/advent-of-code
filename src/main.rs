mod days;
use days::*;

use std::fs::File;
use std::io::Read;

fn main() {
    let day = 2;

    let mut filename = "./input_day_".to_string();
    let day_str = day.to_string();
    filename.push_str(&day_str);

    let mut file = File::open(filename).unwrap();
    let mut input = String::new();
    let _ = file.read_to_string(&mut input);

    match day {
        1 => day1::day_1(input),
        2 => day2::day_2(input),
        _ => ()
    }
}
