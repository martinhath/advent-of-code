extern crate regex;

use self::regex::Regex;

const N: usize = 1000;

pub fn day_6(input: String) {
    let mut field: Vec<Vec<i32>> = Vec::new();

    for _ in (0..N) {
        let mut v = Vec::new();
        for _ in (0..N) {
            v.push(0);
        }
        field.push(v);
    }

    let regex_nums = Regex::new(r"(\d{1,3}),(\d{1,3}) through (\d{1,3}),(\d{1,3})").unwrap();

    for line in input.lines() {

        let cap = regex_nums.captures(line).unwrap();
        let start = (cap.at(1).unwrap().parse::<usize>().unwrap(),
                     cap.at(2).unwrap().parse::<usize>().unwrap());
        let end   = (cap.at(3).unwrap().parse::<usize>().unwrap(),
                     cap.at(4).unwrap().parse::<usize>().unwrap());

        let (start_x, start_y) = start;
        let (end_x, end_y) = end;
        if line.starts_with("toggle") {
            for j in (start_y..end_y + 1) {
                for i in (start_x..end_x + 1) {
                    field[j][i] += 2;
                }
            }
        } else if line.starts_with("turn off") {
            for j in (start_y..end_y + 1) {
                for i in (start_x..end_x + 1) {
                    if field[j][i] > 0 {
                        field[j][i] -= 1;
                    }
                }
            }
        } else { // turn on
            for j in (start_y..end_y + 1) {
                for i in (start_x..end_x + 1) {
                    field[j][i] += 1;
                }
            }
        }
    }

    let num_true = field.iter().map(|v| v.iter().fold(0, |a, e| a + e))
                        .fold(0, |a, e| a + e);
    println!("solution: {}", num_true);
}
