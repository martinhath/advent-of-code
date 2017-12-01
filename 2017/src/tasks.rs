use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

fn file_to_string(s: &str) -> Result<String, io::Error> {
    let mut string = String::new();
    File::open(Path::new(s))?.read_to_string(&mut string)?;
    Ok(string)
}

fn solve_task_1(input: String) -> u32 {
    let mut nums = input.chars().flat_map(|c| c.to_digit(10)).collect::<Vec<u32>>();
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

pub fn task_1() {
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
