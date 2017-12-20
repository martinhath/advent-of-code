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

pub fn task_9() {
    fn solve_pt1(input: &str) -> (usize, usize) {
        let mut chars = input.chars();

        let mut remove_bangs = String::new();
        while let Some(c) = chars.next() {
            if c == '!' {
                chars.next();
            } else {
                remove_bangs.push(c);
            }

        }

        let mut chars = remove_bangs.chars();

        let mut remove_garbage = String::new();
        let mut garbage_count = 0;
        let mut is_skipping = false;
        while let Some(c) = chars.next() {
            if c == '<' {
                if is_skipping {
                    garbage_count += 1;
                }
                is_skipping = true;
            } else if c == '>' {
                is_skipping = false;
            } else if !is_skipping {
                remove_garbage.push(c);
            } else {
                garbage_count += 1;
            }
        }

        let mut chars = remove_garbage.chars();

        let mut remove_rest = String::new();
        while let Some(c) = chars.next() {
            if c == '{' || c == '}' {
                remove_rest.push(c);
            }
        }

        let mut sum = 0;
        let mut level = 1;
        let mut chars = remove_rest.chars();
        while let Some(c) = chars.next() {
            match c {
                '{' => {
                    sum += level;
                    level += 1;
                }
                '}' => {
                    level -= 1;
                }
                c => panic!("should not get '{}' here!", c),
            }
        }
        (sum, garbage_count)
    }

    //     let test = "{}
    // {{{}}}
    // {{},{}}
    // {{{},{},{{}}}}
    // {<a>,<a>,<a>,<a>}
    // {{<ab>},{<ab>},{<ab>},{<ab>}}
    // {{<!!>},{<!!>},{<!!>},{<!!>}}
    // {{<a!>},{<a!>},{<a!>},{<ab>}}";

    let test = "<>
<random characters>
<<<<>
<{!>}>
<!!>
<!!!>>
<{o\"i!a,<{i<a>";

    for line in test.lines() {
        println!("{:?}", solve_pt1(line));
    }

    println!("{:?}", solve_pt1(&file_to_string("input/9").unwrap()));
}

pub fn task_10() {
    fn solve_pt1(n: usize, steps: &[usize]) {
        let mut numbers = Vec::with_capacity(n as usize);
        for i in 0..n {
            numbers.push(i);
        }
        let mut pos = 0;
        let mut skip = 0;
        for &step in steps {
            let mut copy = numbers
                .iter()
                .cycle()
                .skip(pos)
                .take(step)
                .cloned()
                .collect::<Vec<_>>();
            copy.reverse();

            if pos + step >= n {
                // wrap around case
                let copy = {
                    let last = copy.iter().take(n - pos);
                    let first = copy.iter().skip(n - pos);
                    let rest = numbers.iter().skip((step + pos) - n).take(n - step);
                    first
                        .cloned()
                        .chain(rest.cloned())
                        .chain(last.cloned())
                        .collect()
                };
                numbers = copy;
            } else {
                numbers.iter_mut().skip(pos).zip(copy.iter()).for_each(
                    |(p, n)| {
                        *p = *n
                    },
                );
            }
            pos = (pos + step + skip) % n;
            skip += 1;
        }
        println!("{:?}", numbers);
    }

    fn solve_pt2(input: &str) {
        let mut lengths = input.as_bytes().iter().cloned().collect::<Vec<u8>>();
        lengths.extend(&[17, 31, 73, 47, 23]);

        let n = 256;
        let mut numbers = Vec::with_capacity(n);
        for i in 0..n {
            numbers.push(i);
        }

        let mut pos = 0;
        let mut skip = 0;
        for _ in 0..64 {
            for &length in lengths.iter() {
                let l = length as usize;
                let mut copy = numbers
                    .iter()
                    .cycle()
                    .skip(pos)
                    .take(l)
                    .cloned()
                    .collect::<Vec<_>>();
                copy.reverse();

                if pos + l >= n {
                    // wrap around case
                    let copy = {
                        let last = copy.iter().take(n - pos);
                        let first = copy.iter().skip(n - pos);
                        let rest = numbers.iter().skip((l + pos) - n).take(n - l);
                        first
                            .cloned()
                            .chain(rest.cloned())
                            .chain(last.cloned())
                            .collect()
                    };
                    numbers = copy;
                } else {
                    numbers.iter_mut().skip(pos).zip(copy.iter()).for_each(
                        |(p, n)| {
                            *p = *n
                        },
                    );
                }
                pos = (pos + l + skip) % n;
                skip += 1;
            }
        }
        let dense = numbers
            .chunks(16)
            .map(|chunk| chunk.iter().fold(0, ::std::ops::BitXor::bitxor))
            .collect::<Vec<_>>();
        for byte in &dense {
            print!("{:02x}", byte);
        }
        println!();
    }

    // solve_pt1(256, &[34,88,2,222,254,93,150,0,199,255,39,32,137,136,1,167]);
    solve_pt2("34,88,2,222,254,93,150,0,199,255,39,32,137,136,1,167");
}

pub fn task_11() {
    fn solve_pt1(input: &str) -> i32 {
        let dirs = input.trim().split(",");
        // Dirs: a = n-s, b = nw-se, c = ne - sw
        let mut a = 0i32;
        let mut b = 0i32;
        let mut c = 0i32;
        for dir in dirs {
            match dir {
                "n" =>  a += 1,
                "nw" => b += 1,
                "ne" => c += 1,
                "s" =>  a -= 1,
                "sw" => c -= 1,
                "se" => b -= 1,
                e => panic!("what about '{}'?", e),
            }
        }
        let x = c - b;
        let y = a + b + c;
        let xa = x.abs();
        let ya = y.abs();
        if xa >= ya { 
            xa
        } else {
            xa + (ya - xa) / 2
        }
    }

    fn solve_pt2(input: &str) -> i32 {
        let dirs = input.trim().split(",");
        // Dirs: a = n-s, b = nw-se, c = ne - sw
        let mut a = 0i32;
        let mut b = 0i32;
        let mut c = 0i32;
        let mut max_dist = 0;
        for dir in dirs {
            match dir {
                "n" => a += 1,
                "nw" => b += 1,
                "ne" => c += 1,
                "s" => a -= 1,
                "sw" => c -= 1,
                "se" => b -= 1,
                e => panic!("what about '{}'?", e),
            }
            let dist = {
                let x = c - b;
                let y = a + b + c;
                let xa = x.abs();
                let ya = y.abs();
                if xa >= ya { xa } else { xa + (ya - xa) / 2 }

            };
            max_dist = max_dist.max(dist);
        }
        max_dist
    }
    assert_eq!(solve_pt1("ne,ne,ne"), 3);
    assert_eq!(solve_pt1("ne,ne,sw,sw"), 0);
    assert_eq!(solve_pt1("ne,ne,s,s"), 2);
    assert_eq!(solve_pt1("se,sw,se,sw,sw"), 3);

    println!("{}", solve_pt2(&file_to_string("input/11").unwrap()));
}
