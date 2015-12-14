#[allow(dead_code)]
mod days;
use days::*;

use std::fs::File;
use std::io::Read;

fn main() {
    let day = 7;

    let filename = format!("./input/input_day_{}", day);

    let mut file = File::open(filename);

    let input: String = if let Ok(ref mut f) = file {
        let mut inp = String::new();
        let _ = f.read_to_string(&mut inp);
        inp
    } else {
        "".to_string()
    };

    match day {
        1 => day1::day_1(input),
        2 => day2::day_2(input),
        3 => day3::day_3(input),
        4 => day4::day_4(input),
        5 => day5::day_5(input),
        6 => day6::day_6(input),
        7 => day7::day_7(input),
        11 => day11::day_11(input),
        _ => ()
    }
}
