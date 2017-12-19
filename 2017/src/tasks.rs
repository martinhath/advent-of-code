use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;

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
        R,
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
                        return pos.0.abs() + pos.1.abs();
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
                    let value = values.get(&(pos.0 - 1, pos.1 - 1)).unwrap_or(&0) +
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

pub fn task_4() {
    let input = file_to_string("input/4").unwrap();
    // Part I
    // let num_valid = input.lines()
    //     .filter(|line| {
    //         let words = line.split(" ").count();
    //         let unique = line.split(" ").collect::<HashSet<_>>().len();
    //         words == unique
    //     }).count();
    let num_valid = input
        .lines()
        .filter(|line| {
            let words = line.split(" ").count();
            let unique = line.split(" ")
                .map(|w| {
                    let mut ws: Vec<char> = w.chars().collect();
                    ws.sort();
                    ws
                })
                .collect::<HashSet<_>>()
                .len();
            words == unique
        })
        .count();
    println!("num_valid = {}", num_valid);
}

pub fn task_5() {
    fn solve(input: &str) -> usize {
        let mut nums = input
            .lines()
            .flat_map(|n| n.parse())
            .collect::<Vec<isize>>();
        let n = nums.len() as isize;
        let mut pc = 0isize;
        let mut count = 0;
        while pc >= 0 && pc < n {
            let pcu = pc as usize;
            let new_pc = pc + nums[pcu];
            if nums[pcu] >= 3 {
                nums[pcu] -= 1;
            } else {
                nums[pcu] += 1;
            }
            pc = new_pc;
            count += 1;
        }
        count
    }

    let test = "
0
3
0
1
-3";
    // println!("test: {} ", solve(test));
    println!("part 1: {} ", solve(&file_to_string("input/5").unwrap()));
}

pub fn task_6() {
    fn solve_pt1(mut banks: Vec<usize>) -> usize {
        let mut seen = HashSet::new();
        seen.insert(banks.clone());
        let n = banks.len();
        loop {
            let mut next_banks = banks.clone();
            let i = next_banks
                .iter()
                .enumerate()
                .max_by_key(|&(i, n)| (n, -(i as i32)))
                .map(|(i, _n)| i)
                .unwrap();
            let a = next_banks[i];
            println!("{:?}  (i={} a={})", banks, i, a);
            next_banks[i] = 0;
            for i in (0..n).cycle().skip(i + 1).take(a) {
                next_banks[i] += 1;
            }
            if seen.contains(&next_banks) {
                break;
            }
            seen.insert(next_banks.clone());
            banks = next_banks;
        }
        seen.len()
    }

    fn solve_pt2(mut banks: Vec<usize>) -> usize {
        let mut seen = HashMap::new();
        seen.insert(banks.clone(), 0);
        let n = banks.len();
        let mut c = 0;
        loop {
            c += 1;
            let mut next_banks = banks.clone();
            let i = next_banks
                .iter()
                .enumerate()
                .max_by_key(|&(i, n)| (n, -(i as i32)))
                .map(|(i, _n)| i)
                .unwrap();
            let a = next_banks[i];
            println!("{:?}  (i={} a={})", banks, i, a);
            next_banks[i] = 0;
            for i in (0..n).cycle().skip(i + 1).take(a) {
                next_banks[i] += 1;
            }
            match seen.entry(next_banks.clone()) {
                Entry::Vacant(p) => {
                    p.insert(c);
                }
                Entry::Occupied(v) => {
                    return c - *v.get();
                }
            }
            banks = next_banks;
        }
    }

    // test:
    // let banks = vec![0, 2, 7, 0];

    let banks = vec![5, 1, 10, 0, 1, 7, 13, 14, 3, 12, 8, 10, 7, 12, 0, 6];
    println!("{}", solve_pt1(banks.clone()));
    println!("{}", solve_pt2(banks.clone()));
}

pub fn task_7() {
    fn solve_pt1(input: &str) {
        let mut parents = HashMap::new();
        let mut last_name = "";
        for line in input.lines() {
            let mut split = line.split("->");
            let name = split
                .next()
                .and_then(|s| s.split(" ").next())
                .unwrap()
                .trim();
            if let Some(s) = split.next() {
                let children = s.split(", ");
                for child in children {
                    parents.insert(child.trim(), name);
                }
            }
            last_name = name;
        }
        while parents.contains_key(&last_name) {
            last_name = parents.get(&last_name).unwrap();
        }
        println!("top parent = {:?}", last_name);
    }

    fn solve_pt2(input: &str) {
        let mut parents = HashMap::new();
        let mut children = HashMap::new();
        let mut weights = HashMap::new();
        let mut last_name = "";
        for line in input.lines() {
            let mut split = line.split("->");
            let (name, weight) = {
                let mut s = split.next().unwrap().split(" ");
                let name = s.next().unwrap();
                let weight = s.next().unwrap();
                (name, weight[1..weight.len() - 1].parse::<i32>().unwrap())
            };
            weights.insert(name, weight);
            if let Some(s) = split.next() {
                let mut cs = Vec::new();
                for child in s.split(", ") {
                    parents.insert(child.trim(), name);
                    cs.push(child.trim());
                }
                children.insert(name, cs);
            }
            last_name = name;
        }
        while parents.contains_key(&last_name) {
            last_name = parents.get(&last_name).unwrap();
        }

        let mut accumulated_weights = HashMap::new();

        fn accumulate_weight<'a>(
            root: &'a str,
            weights: &HashMap<&'a str, i32>,
            children: &HashMap<&'a str, Vec<&'a str>>,
            acc_wegihts: &mut HashMap<&'a str, i32>,
        ) {
            if !children.contains_key(&root) {
                acc_wegihts.insert(root, weights[root]);
                return;
            }
            let mut sum = weights[root];
            for child in &children[root] {
                accumulate_weight(child, weights, children, acc_wegihts);
                sum += acc_wegihts[child];
            }
            acc_wegihts.insert(root, sum);
        }

        fn print_weights(
            root: &str,
            children: &HashMap<&str, Vec<&str>>,
            weights: &HashMap<&str, i32>,
            level: usize,
        ) {
            for i in 0..level {
                print!("{}", if i % 4 == 3 { '.' } else { ' ' });
            }
            println!("{} ({})", root, weights[root]);
            if let Some(cs) = children.get(root) {
                for child in cs {
                    print_weights(child, children, weights, level + 4);
                }
            }
            for i in 0..level {
                print!("{}", if i % 4 == 3 { '.' } else { ' ' });
            }
            println!("{}={}", root, weights[root]);
        }

        accumulate_weight(last_name, &weights, &children, &mut accumulated_weights);
        print_weights(last_name, &children, &accumulated_weights, 0);
    }

    let input = "
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
    solve_pt2(&file_to_string("input/7").unwrap());
}

pub fn task_8() {
    fn solve_pt1(input: &str) -> i32 {
        let mut registers = HashMap::new();
        let mut max = 0;
        for line in input.lines() {
            if line.trim() == "" {
                continue;
            }
            let mut s = line.split(" ");
            let reg = s.next().unwrap();
            let inc = s.next().unwrap();
            let n = s.next().unwrap().parse::<i32>().unwrap();
            let _if = s.next().unwrap();
            let cond_reg = s.next().unwrap();
            let cond_op = s.next().unwrap();
            let cond_r = s.next().unwrap().parse::<i32>().unwrap();

            let mul = if inc == "inc" { 1 } else { -1 };
            let cond_l = *registers.entry(cond_reg).or_insert(0);
            let should_execute = match cond_op {
                ">" => cond_l > cond_r,
                "<" => cond_l < cond_r,
                ">=" => cond_l >= cond_r,
                "<=" => cond_l <= cond_r,
                "==" => cond_l == cond_r,
                "!=" => cond_l != cond_r,
                _ => panic!("what is {}", cond_op),
            };
            if should_execute {
                *registers.entry(reg).or_insert(0) += mul * n;
                max = max.max(*registers.get(reg).unwrap());
            }
        }
        max
    }

    let input = "
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
    println!("{}", solve_pt1(&file_to_string("input/8").unwrap()));
}
