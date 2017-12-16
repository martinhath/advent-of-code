use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

fn file_to_string(s: &str) -> Result<String, io::Error> {
    let mut string = String::new();
    File::open(Path::new(s))?.read_to_string(&mut string)?;
    Ok(string)
}

pub fn task_1() {
    fn solve_task_1(input: String) -> u32 {
        let mut nums = input
            .chars()
            .flat_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>();
        // Part I
        //  let first = nums[0];
        //  nums.push(first);
        //  nums.windows(2).flat_map(|win| {
        //      if win[0] == win[1] { Some(win[0]) } else {None}
        //  }).sum::<u32>()
        let mut sum = 0;
        let n = nums.len();
        for i in 0..n {
            let next_i = (i + n / 2) % n;
            if nums[i] == nums[next_i] {
                sum += nums[i];
            }
        }
        sum
    }
    // Part I
    // assert_eq!(solve_task_1("1122".to_string()), 3);
    // assert_eq!(solve_task_1("1111".to_string()), 4);
    // assert_eq!(solve_task_1("1234".to_string()), 0);
    // assert_eq!(solve_task_1("91212129".to_string()), 9);

    assert_eq!(solve_task_1("1212".to_string()), 6);
    assert_eq!(solve_task_1("1221".to_string()), 0);
    assert_eq!(solve_task_1("123425".to_string()), 4);
    assert_eq!(solve_task_1("123123".to_string()), 12);
    assert_eq!(solve_task_1("12131415".to_string()), 4);

    let input = file_to_string("input/1").unwrap();
    let res = solve_task_1(input);
    println!("Sum is {}", res);
}

pub fn task_2() {
    fn solve_task_2(input: &str) -> u32 {
        let nums: Vec<Vec<u32>> = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .flat_map(|n| n.parse::<u32>())
                    .collect::<Vec<_>>()
            })
            .collect();

        // Part I
        // nums.iter()
        //     .map(|line| {
        //         line.iter().max().unwrap_or(&0) - line.iter().min().unwrap_or(&0)
        //     })
        //     .sum::<u32>()

        nums.iter()
            .flat_map(|line| {
                for i in 0..line.len() {
                    for j in (i + 1)..line.len() {
                        let a = line[i];
                        let b = line[j];
                        if a % b == 0 {
                            return Some(a / b);
                        }
                        if b % a == 0 {
                            return Some(b / a);
                        }
                    }
                }
                None
            })
            .sum::<u32>()
    }

    // Part I
    //     let test = solve_task_2(
    //         "5 1 9 5
    // 7 5 3
    // 2 4 6 8",
    //     );
    //     assert_eq!(test, 18);
    assert_eq!(
        solve_task_2(
            "5 9 2 8
9 4 7 3
3 8 6 5",
        ),
        9
    );
    println!(
        "Result is {}",
        solve_task_2(&file_to_string("input/2").expect("file not found"))
    );
}

pub fn task_3() {
    enum Dir {
        U,
        L,
        D,
        R
    }
    fn iterative_pt1(input: i64) -> i64 {
        let mut pos = (1i64, -1i64);
        let mut i = 1;
        for n in (1..).map(|n| n * 2) {
            for d in [Dir::U, Dir::L, Dir::D, Dir::R].iter() {
                for _ in 0..n {
                    i += 1;
                    match *d {
                        Dir::U => pos.1 += 1,
                        Dir::L => pos.0 -= 1,
                        Dir::D => pos.1 -= 1,
                        Dir::R => pos.0 += 1,
                    }
                    if i == input {
                        return pos.0.abs() + pos.1.abs()
                    }
                }
            }
            pos.0 += 1;
            pos.1 -= 1;
        }
        unreachable!();
    }

    fn iterative_pt2(input: i64) -> i64 {
        let mut pos = (1i64, -1i64);
        let mut i = 1;
        let mut values = HashMap::new();
        values.insert((0, 0), 1);
        for n in (1..).map(|n| n * 2) {
            for d in [Dir::U, Dir::L, Dir::D, Dir::R].iter() {
                for _ in 0..n {
                    i += 1;
                    match *d {
                        Dir::U => pos.1 += 1,
                        Dir::L => pos.0 -= 1,
                        Dir::D => pos.1 -= 1,
                        Dir::R => pos.0 += 1,
                    }
                    let value =
                        values.get(&(pos.0 - 1, pos.1 - 1)).unwrap_or(&0) +
                        values.get(&(pos.0 - 1, pos.1 + 0)).unwrap_or(&0) +
                        values.get(&(pos.0 - 1, pos.1 + 1)).unwrap_or(&0) +
                        values.get(&(pos.0 + 0, pos.1 - 1)).unwrap_or(&0) +
                        values.get(&(pos.0 + 0, pos.1 + 0)).unwrap_or(&0) +
                        values.get(&(pos.0 + 0, pos.1 + 1)).unwrap_or(&0) +
                        values.get(&(pos.0 + 1, pos.1 - 1)).unwrap_or(&0) +
                        values.get(&(pos.0 + 1, pos.1 + 0)).unwrap_or(&0) +
                        values.get(&(pos.0 + 1, pos.1 + 1)).unwrap_or(&0);
                    values.insert(pos, value);
                    println!("{:?} = {}", pos, value);
                    if value > input {
                        return value;
                    }
                }
            }
            pos.0 += 1;
            pos.1 -= 1;
        }
        unreachable!();
    }
    assert_eq!(iterative_pt1(2), 1);
    assert_eq!(iterative_pt1(3), 2);
    assert_eq!(iterative_pt1(19), 2);
    assert_eq!(iterative_pt1(21), 4);

    assert_eq!(iterative_pt2(6), 10);

    let input = 361527;
    // println!("{:?}", iterative_pt1(input));
    println!("{:?}", iterative_pt2(input));
}
