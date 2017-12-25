use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::collections::{HashMap, HashSet, VecDeque};
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

fn knot_hash(input: &str) -> Vec<usize> {
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
    dense
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

    fn solve_pt2(input: &str) -> Vec<usize> {
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
        dense
    }

    // solve_pt1(256, &[34,88,2,222,254,93,150,0,199,255,39,32,137,136,1,167]);
    // solve_pt2("34,88,2,222,254,93,150,0,199,255,39,32,137,136,1,167");
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
                "n" => a += 1,
                "nw" => b += 1,
                "ne" => c += 1,
                "s" => a -= 1,
                "sw" => c -= 1,
                "se" => b -= 1,
                e => panic!("what about '{}'?", e),
            }
        }
        let x = c - b;
        let y = a + b + c;
        let xa = x.abs();
        let ya = y.abs();
        if xa >= ya { xa } else { xa + (ya - xa) / 2 }
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

pub fn task_12() {
    fn solve_pt1(input: &str) -> usize {
        let mut mappings: HashMap<usize, Vec<usize>> = HashMap::new();
        for line in input.lines() {
            let mut s = line.split("<->");
            let a = s.next().unwrap().trim().parse::<usize>().unwrap();
            let to = s.next()
                .unwrap()
                .split(",")
                .flat_map(|n| n.trim().parse::<usize>())
                .collect::<Vec<_>>();
            mappings.insert(a, to);
        }

        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(0);
        while let Some(item) = queue.pop_front() {
            let v = &mappings[&item];
            for n in v {
                if !seen.contains(n) {
                    seen.insert(n);
                    queue.push_back(*n)
                }
            }
        }
        seen.len()
    }

    fn solve_pt2(input: &str) -> usize {
        let mut mappings: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut programs = Vec::new();
        for line in input.lines() {
            let mut s = line.split("<->");
            let a = s.next().unwrap().trim().parse::<usize>().unwrap();
            let to = s.next()
                .unwrap()
                .split(",")
                .flat_map(|n| n.trim().parse::<usize>())
                .collect::<Vec<_>>();
            mappings.insert(a, to);
            programs.push(a);
        }

        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        let mut num_programs = 0;
        while programs.len() > 0 {
            num_programs += 1;
            queue.push_back(programs.swap_remove(0));
            while let Some(item) = queue.pop_front() {
                programs.remove_item(&item);
                let v = &mappings[&item];
                println!("{} => {:?}", item, v);
                for n in v {
                    if !seen.contains(n) {
                        seen.insert(n);
                        queue.push_back(*n)
                    }
                }
            }
            println!("  {:?}", seen);
            seen.clear();
        }
        num_programs
    }

    let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
    //     solve_pt1(input);
    let real = &file_to_string("input/12").unwrap();
    println!("{:?}", solve_pt2(real));
}

pub fn task_13() {
    #[derive(Debug, Clone)]
    struct Layer {
        capacity: i32,
        current: i32,
        down: bool,
    }

    impl Layer {
        fn next(&mut self) {
            if self.capacity == 0 {
                return;
            }
            if self.down {
                if self.current == self.capacity - 1 {
                    self.current -= 1;
                    self.down = false;
                } else {
                    self.current += 1;
                }
            } else {
                if self.current == 0 {
                    self.current += 1;
                    self.down = true;
                } else {
                    self.current -= 1;
                }
            }
        }
    }

    fn solve_pt1(input: &str) -> i32 {
        let mut layers = vec![];
        let mut next = 0;
        for line in input.lines() {
            let mut s = line.split(": ");
            let i = s.next().unwrap().parse().unwrap();
            let c = s.next().unwrap().parse().unwrap();
            while next < i {
                layers.push(Layer {
                    capacity: 0,
                    current: -1,
                    down: false,
                });
                next += 1;
            }
            layers.push(Layer {
                capacity: c,
                current: 0,
                down: true,
            });
            next = i + 1;
        }
        let mut severity = 0;
        for i in 0..layers.len() {
            {
                let l = &layers[i];
                if l.capacity > 0 && l.current == 0 {
                    severity += i as i32 * l.capacity;
                }
            }
            layers.iter_mut().for_each(Layer::next);
        }
        severity
    }

    fn solve_pt2(input: &str) -> i32 {
        let mut layers = vec![];
        let mut next = 0;
        for line in input.lines() {
            let mut s = line.split(": ");
            let i = s.next().unwrap().parse().unwrap();
            let c = s.next().unwrap().parse().unwrap();
            while next < i {
                layers.push(Layer {
                    capacity: 0,
                    current: -1,
                    down: false,
                });
                next += 1;
            }
            layers.push(Layer {
                capacity: c,
                current: 0,
                down: true,
            });
            next = i + 1;
        }
        let n = layers.len();
        for i in 1..n {
            layers[i..].iter_mut().for_each(Layer::next);
        }

        for delay in 0.. {
            println!("[{}]", delay);
            if !layers.iter().any(|l| l.capacity > 0 && l.current == 0) {
                return delay as i32;
            }
            layers.iter_mut().for_each(Layer::next);
        }
        unreachable!();
    }

    let input = "0: 3
1: 2
4: 4
6: 4";
    let real = &file_to_string("input/13").unwrap();
    println!("{:?}", solve_pt2(real));
}

pub fn task_14() {
    fn solve_pt1(input: &str) -> u32 {
        const N: usize = 128;
        let mut ones = 0;
        for row in 0..N {
            let hash = knot_hash(&format!("{}-{}", input, row));
            ones += hash.iter().map(|u| u.count_ones()).sum::<u32>();
        }
        ones
    }

    fn solve_pt2(input: &str) -> u32 {
        const N: usize = 128;
        let mut grid = vec![vec![]; N];
        for row in 0..N {
            let hash = knot_hash(&format!("{}-{}", input, row));
            for n in &hash {
                fn bit(n: usize, i: usize) -> u32 {
                    if n & (1 << i) > 0 { 1 } else { 0 }
                }
                let a = n >> 4;
                let b = n & 0b1111;
                grid[row].push(bit(a, 3));
                grid[row].push(bit(a, 2));
                grid[row].push(bit(a, 1));
                grid[row].push(bit(a, 0));
                grid[row].push(bit(b, 3));
                grid[row].push(bit(b, 2));
                grid[row].push(bit(b, 1));
                grid[row].push(bit(b, 0));
            }
        }

        fn flood_region(grid: &mut Vec<Vec<u32>>, x: usize, y: usize, n: u32) {
            if grid[y][x] == 1 {
                grid[y][x] = n;
                if y > 0 {
                    flood_region(grid, x, y - 1, n);
                }
                if x > 0 {
                    flood_region(grid, x - 1, y, n);
                }
                if y < grid.len() - 1 {
                    flood_region(grid, x, y + 1, n);
                }
                if x < grid.len() - 1 {
                    flood_region(grid, x + 1, y, n);
                }
            }
        }

        let mut regions = 2;
        for row in 0..N {
            for col in 0..N {
                if grid[row][col] == 1 {
                    flood_region(&mut grid, col, row, regions);
                    regions += 1;
                }
            }
        }
        regions - 2
    }
    let input = "flqrgnkx";
    // println!("{:?}", solve_pt1(input));
    let real = "stpzcrnm";
    println!("{:?}", solve_pt2(real));
}

pub fn task_15() {
    struct Generator {
        factor: u64,
        value: u64,
    }

    impl Generator {
        fn next(&mut self) -> u32 {
            let a = self.value * self.factor % 2147483647;
            self.value = a;
            a as u32
        }

        fn next2(&mut self, multiple: u64) -> u32 {
            let mut a = 1;
            while a % multiple != 0 {
                a = self.value * self.factor % 2147483647;
                self.value = a;
            }
            a as u32
        }
    }

    // part I
    // let n = 40_000_000;
    // let mut a = Generator { factor: 16807, value: 679 };
    // let mut b = Generator { factor: 48271, value: 771 };
    //
    // let score = (0..n).filter(|_| {
    //     (a.next() & 0xffff) == (b.next() & 0xffff)
    // }).count();

    let n = 5_000_000;
    let mut a = Generator {
        factor: 16807,
        value: 679, // value: 65
    };
    let mut b = Generator {
        factor: 48271,
        value: 771, // value: 8921
    };

    let score = (0..n)
        .filter(|_| (a.next2(4) & 0xffff) == (b.next2(8) & 0xffff))
        .count();
    println!("{}", score);
}

pub fn task_16() {
    fn solve_pt1(input: &str) {
        let mut programs = (b'a'..b'q').collect::<Vec<_>>();
        let n = programs.len();
        for mv in input.split(",") {
            let (first, rest) = mv.split_at(1);
            match first {
                "s" => {
                    // move the `mv[1]th` last elements from the back to the front
                    let a = rest.parse::<u32>().unwrap();
                    let mut new: Vec<_> = programs.iter().skip(n - a as usize).cloned().collect();
                    new.extend(programs.iter().take(n - a as usize));
                    programs = new;
                }
                "x" => {
                    // swap on indices
                    let mut s = rest.split("/");
                    let a = s.next().unwrap().parse::<usize>().unwrap();
                    let b = s.next().unwrap().parse::<usize>().unwrap();

                    let tmp = programs[a];
                    programs[a] = programs[b];
                    programs[b] = tmp;
                }
                "p" => {
                    // swap on names
                    let mut s = rest.split("/");
                    let a = s.next().unwrap().as_bytes()[0];
                    let b = s.next().unwrap().as_bytes()[0];
                    let a = programs.iter().position(|&c| c == a).unwrap();
                    let b = programs.iter().position(|&c| c == b).unwrap();

                    let tmp = programs[a];
                    programs[a] = programs[b];
                    programs[b] = tmp;
                }
                _ => unreachable!(),
            }
        }
        println!(
            "{}",
            programs.iter().map(|&b| b as char).collect::<String>()
        );
    }

    fn solve_pt2(input: &str) {
        let char_range = b'a'..b'q';
        const N: usize = 1_000_000_000 - 1;
        // let char_range = b'a'..b'f';
        // const N: usize = 2 - 1;

        let mut programs = char_range.clone().collect::<Vec<_>>();
        let mut label_mapping: HashMap<u8, u8> = programs.iter().map(|&b| (b, b)).collect();
        let n = programs.len();

        for mv in input.split(",") {
            let (first, rest) = mv.split_at(1);
            match first {
                "s" => {
                    // move the `mv[1]th` last elements from the back to the front
                    let a = rest.parse::<u32>().unwrap();
                    let mut new: Vec<_> = programs.iter().skip(n - a as usize).cloned().collect();
                    new.extend(programs.iter().take(n - a as usize));
                    programs = new;
                }
                "x" => {
                    // swap on indices
                    let mut s = rest.split("/");
                    let a = s.next().unwrap().parse::<usize>().unwrap();
                    let b = s.next().unwrap().parse::<usize>().unwrap();

                    let tmp = programs[a];
                    programs[a] = programs[b];
                    programs[b] = tmp;
                }
                "p" => {
                    // swap on names
                    let mut s = rest.split("/");
                    let a = label_mapping[&s.next().unwrap().as_bytes()[0]];
                    let b = label_mapping[&s.next().unwrap().as_bytes()[0]];

                    let tmp = label_mapping[&a];
                    let asd = label_mapping[&b];
                    label_mapping.insert(a, asd);
                    label_mapping.insert(b, tmp);

                }
                _ => unreachable!(),
            }
        }
        // THis doesn't work, since swap on names is different.
        // Accumulate the other perms in between, since they are the same?
        // 0 1 2 3 4 5 6 7
        // 4 6 2 3 5 1 7 0
        let permutation = programs.iter().map(|&b| b - b'a').collect::<Vec<_>>();
        let mut current = programs;
        let mut next = current.clone();
        println!("current = {:?}", current);

        let label_perm = {
            let mut v = Vec::new();
            for c in char_range {
                v.push(label_mapping[&c] - b'a');
            }
            v
        };
        let final_perm = {
            let mut perm = label_perm.clone();
            for i in 0..label_perm.len() {
                println!("label perm #{}/{}", i, label_perm.len());
                let mut current = label_perm[i];
                for _ in 0..N {
                    current = label_perm[current as usize];
                }
                perm[i] = current;
            }
            perm
        };
        println!("final_perm = {:?}", final_perm);

        for i in 0..N {
            if i & 0xffffff == 0 {
                println!("perm #{}/{}", i, N);
            }
            for (old, &new) in permutation.iter().enumerate() {
                next[old] = current[new as usize];
            }
            // println!("permuted: {:?}", next);
            ::std::mem::swap(&mut current, &mut next);
        }

        println!(
            "{}",
            current
                .iter()
                .map(|&b| (final_perm[(b - b'a') as usize] + b'a') as char)
                .collect::<String>()
        );
    }

    fn solve_pt2_2(input: &str) {
        enum Op {
            S(usize),
            X(usize, usize),
            P(u8, u8),
        }

        impl Op {
            fn perform(&self, v: &mut Vec<u8>) {
                match *self {
                    Op::S(a) => {
                        let n = v.len();
                        let mut new: Vec<_> = v.iter().skip(n - a).cloned().collect();
                        new.extend(v.iter().take(n - a));
                        *v = new;
                    }
                    Op::X(a, b) => {
                        let tmp = v[a];
                        v[a] = v[b];
                        v[b] = tmp;
                    }
                    Op::P(a, b) => {
                        let a = v.iter().position(|&c| c == a).unwrap();
                        let b = v.iter().position(|&c| c == b).unwrap();
                        let tmp = v[a];
                        v[a] = v[b];
                        v[b] = tmp;
                    }
                }
            }
        }
        let char_range = b'a'..b'q';
        const N: usize = 1_000_000_000;
        // let char_range = b'a'..b'f';
        // const N: usize = 2;

        let mut operations = Vec::new();

        for mv in input.split(",") {
            let (first, rest) = mv.split_at(1);
            match first {
                "s" => {
                    // move the `mv[1]th` last elements from the back to the front
                    let a = rest.parse::<usize>().unwrap();
                    operations.push(Op::S(a));
                }
                "x" => {
                    // swap on indices
                    let mut s = rest.split("/");
                    let a = s.next().unwrap().parse::<usize>().unwrap();
                    let b = s.next().unwrap().parse::<usize>().unwrap();
                    operations.push(Op::X(a, b));
                }
                "p" => {
                    // swap on names
                    let mut s = rest.split("/");
                    let a = s.next().unwrap().as_bytes()[0];
                    let b = s.next().unwrap().as_bytes()[0];
                    operations.push(Op::P(a, b))
                }
                _ => unreachable!(),
            }
        }

        let mut programs = char_range.clone().collect::<Vec<_>>();
        let mut seen_perms = HashMap::new();
        seen_perms.insert(programs.clone(), 0);

        for i in 1..(N + 1) {
            for op in &operations {
                op.perform(&mut programs);
            }
            if let Some(e) = seen_perms.get(&programs) {
                let ind = (N - e) % (i - e);
                programs = seen_perms
                    .iter()
                    .filter(|&(_k, &v)| v == ind)
                    .next()
                    .expect("Couldn't find ind")
                    .0
                    .clone();
                break;
            }
            seen_perms.insert(programs.clone(), i);
        }
        let s = programs.iter().map(|&b| b as char).collect::<String>();
        println!("{}", s);
    }

    let test = "s1,x3/4,pe/b";
    let real = file_to_string("input/16").unwrap();
    solve_pt2_2(&real);
}

pub fn task_17() {
    fn solve_pt1(input: usize) {
        let mut buffer = vec![0];
        let mut pos = 0;
        let N = 2017;
        for i in 1..(N + 1) {
            pos = (pos + input) % buffer.len();
            if pos == buffer.len() - 1 {
                buffer.push(i);
            } else {
                buffer.insert(pos + 1, i);
            }
            pos += 1;
        }
        let l = (pos + 1) % buffer.len();
        println!("{}", buffer[l]);
    }

    fn solve_pt2(input: usize) {
        const N: usize = 50_000_000;

        let mut pos = 0;
        let mut buffer_len = 1;
        let mut first = 0;
        for i in 1..(N + 1) {
            if i % 500_000 == 0 {
                println!("{}", i);
            }
            pos = (pos + input) % buffer_len;
            if pos == 0 {
                first = i;
            }
            buffer_len += 1;
            pos += 1;
        }
        println!("{}", first);
    }

    solve_pt2(376);
}

pub fn task_18() {
    #[derive(Debug)]
    enum Val<'a> {
        Int(i64),
        Reg(&'a str),
    }

    impl<'a> Val<'a> {
        fn from_str(s: &'a str) -> Self {
            if let Ok(n) = s.parse::<i64>() {
                Val::Int(n)
            } else {
                Val::Reg(s)
            }
        }
    }

    #[derive(Debug)]
    enum Instr<'a> {
        Snd(Val<'a>),
        Set(Val<'a>, Val<'a>),
        Add(Val<'a>, Val<'a>),
        Mul(Val<'a>, Val<'a>),
        Mod(Val<'a>, Val<'a>),
        Rcv(Val<'a>),
        Jgz(Val<'a>, Val<'a>),
    }

    impl<'a> Instr<'a> {
        fn from_str(s: &'a str) -> Self {
            let mut s = s.split(" ");
            let op = s.next();
            let a = s.next();
            let b = s.next();
            match op.unwrap() {
                "snd" => Instr::Snd(Val::from_str(a.unwrap())),
                "set" => Instr::Set(Val::from_str(a.unwrap()), Val::from_str(b.unwrap())),
                "add" => Instr::Add(Val::from_str(a.unwrap()), Val::from_str(b.unwrap())),
                "mul" => Instr::Mul(Val::from_str(a.unwrap()), Val::from_str(b.unwrap())),
                "mod" => Instr::Mod(Val::from_str(a.unwrap()), Val::from_str(b.unwrap())),
                "rcv" => Instr::Rcv(Val::from_str(a.unwrap())),
                "jgz" => Instr::Jgz(Val::from_str(a.unwrap()), Val::from_str(b.unwrap())),
                e => unreachable!(),
            }
        }
    }

    fn value<'a>(val: &Val<'a>, reg: &HashMap<&str, i64>) -> i64 {
        match *val {
            Val::Reg(r) => *reg.get(r).unwrap_or(&0),
            Val::Int(i) => i,
        }
    }

    fn solve_pt1(input: &str) -> i64 {
        let instructions = input.lines().map(Instr::from_str).collect::<Vec<_>>();
        let mut registers = HashMap::new();
        let mut pc = 0i64;
        let mut played_freq = 0;
        while pc >= 0 && (pc as usize) < instructions.len() {
            let ins = &instructions[pc as usize];
            match ins {
                &Instr::Snd(ref a) => {
                    played_freq = value(a, &registers);
                }
                &Instr::Set(Val::Reg(a), ref b) => {
                    let v = value(b, &registers);
                    registers.insert(a, v);
                }
                &Instr::Add(Val::Reg(a), ref b) => {
                    let x = *registers.get(a).unwrap_or(&0);
                    let y = value(b, &registers);
                    registers.insert(a, x + y);
                }
                &Instr::Mul(Val::Reg(a), ref b) => {
                    let x = *registers.get(a).unwrap_or(&0);
                    let y = value(b, &registers);
                    registers.insert(a, x * y);
                }
                &Instr::Mod(Val::Reg(a), ref b) => {
                    let x = *registers.get(a).unwrap_or(&0);
                    let y = value(b, &registers);
                    registers.insert(a, x % y);
                }
                &Instr::Rcv(ref a) => {
                    if value(a, &registers) > 0 {
                        return played_freq;
                    }
                }
                &Instr::Jgz(ref a, ref b) => {
                    if value(a, &registers) > 0 {
                        pc += value(b, &registers) - 1;
                    }
                }
                _ => unreachable!(),
            }
            pc += 1;
        }
        unreachable!()
    }

    fn solve_pt2(input: &str) -> i64 {
        enum Ret {
            Block,
            Send,
            Jump(i64),
            Ok,
        }

        fn execute_instruction<'a>(
            instr: &Instr<'a>,
            registers: &mut HashMap<&'a str, i64>,
            send: &mut VecDeque<i64>,
            recv: &mut VecDeque<i64>,
        ) -> Ret {
            match instr {
                &Instr::Snd(ref a) => {
                    send.push_back(value(a, &registers));
                    return Ret::Send;
                }
                &Instr::Set(Val::Reg(a), ref b) => {
                    let v = value(b, &registers);
                    registers.insert(a, v);
                }
                &Instr::Add(Val::Reg(a), ref b) => {
                    let x = *registers.get(a).unwrap_or(&0);
                    let y = value(b, &registers);
                    registers.insert(a, x + y);
                }
                &Instr::Mul(Val::Reg(a), ref b) => {
                    let x = *registers.get(a).unwrap_or(&0);
                    let y = value(b, &registers);
                    registers.insert(a, x * y);
                }
                &Instr::Mod(Val::Reg(a), ref b) => {
                    let x = *registers.get(a).unwrap_or(&0);
                    let y = value(b, &registers);
                    registers.insert(a, x % y);
                }
                &Instr::Rcv(Val::Reg(a)) => {
                    if let Some(x) = recv.pop_front() {
                        registers.insert(a, x);
                    } else {
                        return Ret::Block;
                    }
                }
                &Instr::Jgz(ref a, ref b) => {
                    if value(a, &registers) > 0 {
                        return Ret::Jump(value(b, &registers));
                    }
                }
                _ => unreachable!(),
            }
            Ret::Ok
        }
        let instructions = input.lines().map(Instr::from_str).collect::<Vec<_>>();
        let mut a_reg = HashMap::new();
        let mut b_reg = HashMap::new();
        a_reg.insert("p", 0);
        b_reg.insert("p", 1);
        let mut a_queue = VecDeque::new();
        let mut b_queue = VecDeque::new();

        let mut pc_a = 0i64;
        let mut pc_b = 0i64;

        let mut sends = 0;
        loop {
            let mut a_block = false;
            if pc_a >= 0 && (pc_a as usize) < instructions.len() {
                let ins = &instructions[pc_a as usize];
                match execute_instruction(ins, &mut a_reg, &mut b_queue, &mut a_queue) {
                    Ret::Block => {
                        a_block = true;
                    }
                    Ret::Send | Ret::Ok => {
                        pc_a += 1;
                    }
                    Ret::Jump(n) => pc_a += n,
                }
            }
            let mut b_block = false;
            if pc_b >= 0 && (pc_b as usize) < instructions.len() {
                let ins = &instructions[pc_b as usize];
                match execute_instruction(ins, &mut b_reg, &mut a_queue, &mut b_queue) {
                    Ret::Block => {
                        b_block = true;
                    }
                    Ret::Send => {
                        sends += 1;
                        pc_b += 1;
                    }
                    Ret::Ok => {
                        pc_b += 1;
                    }
                    Ret::Jump(n) => pc_b += n,
                }
            }
            if a_block && b_block {
                break;
            }
        }
        sends
    }

    let input = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";
    let real = &file_to_string("input/18").unwrap();

    println!("{}", solve_pt2(real));
}

pub fn task_19() {
    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Dir {
        U,
        D,
        L,
        R,
    }

    fn solve_pt1(input: &str) -> String {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let w = grid.len();
        let h = grid[0].len();
        let mut letters = String::new();
        let (mut x, mut y) = (grid[0].iter().position(|&b| b != ' ').unwrap(), 0);
        let mut came_from = Dir::U;
        let mut nexts = Vec::new();
        let mut steps = 0;
        loop {
            steps += 1;
            if came_from != Dir::U && y > 0 && grid[y - 1][x] != ' ' {
                nexts.push(Dir::U);
            }
            if came_from != Dir::D && y < w - 1 && grid[y + 1][x] != ' ' {
                nexts.push(Dir::D);
            }
            if came_from != Dir::L && x > 0 && grid[y][x - 1] != ' ' {
                nexts.push(Dir::L);
            }
            if came_from != Dir::R && x < h - 1 && grid[y][x + 1] != ' ' {
                nexts.push(Dir::R);
            }
            if nexts.len() == 0 {
                println!("{}", steps);
                return letters;
            } else if nexts.len() == 1 {
                match nexts[0] {
                    Dir::U => {
                        came_from = Dir::D;
                        y -= 1
                    }
                    Dir::D => {
                        came_from = Dir::U;
                        y += 1
                    }
                    Dir::L => {
                        came_from = Dir::R;
                        x -= 1
                    }
                    Dir::R => {
                        came_from = Dir::L;
                        x += 1
                    }
                }
            } else {
                match came_from {
                    Dir::U => y += 1,
                    Dir::D => y -= 1,
                    Dir::L => x += 1,
                    Dir::R => x -= 1,
                }
            }
            if grid[y][x].is_alphabetic() {
                letters.push(grid[y][x]);
            }
            nexts.clear();
        }
    }

    let mut test = String::new();
    test.push_str("     |          \n");
    test.push_str("     |  +--+    \n");
    test.push_str("     A  |  C    \n");
    test.push_str(" F---|----E|--+ \n");
    test.push_str("     |  |  |  D \n");
    test.push_str("     +B-+  +--+ ");

    let real = file_to_string("input/19").unwrap();

    println!("'{}'", solve_pt1(&real));
}

pub fn task_20() {
    #[derive(Debug, Clone)]
    struct Particle {
        pos: [i64; 3],
        vel: [i64; 3],
        acc: [i64; 3],
    }

    fn parse_input(input: &str) -> Vec<Particle> {
        let mut v = Vec::new();
        use std::str::{self, FromStr};
        use nom::IResult;

        named!(factor<i64>,
               map_res!( do_parse!(
                        minus: opt!(char!('-')) >>
                        num: take_while!(|b| b >= b'0' && b <= b'9') >>
                        (minus, num)),
                    (|(minus, num): (Option<char>, &[u8])|
                     str::from_utf8(num)
                        .map_err(|_| ())
                        .and_then(|s| FromStr::from_str(s).map_err(|_| ()))
                        .map(|n: i64| n * if minus.is_some() { -1 } else { 1  })
                    )));

        named!(numbers<Vec<i64>>,
               ws!(delimited!(
                   char!('<'),
                   separated_nonempty_list!(
                           char!(','),
                           preceded!(opt!(char!(' ')), factor)),
                   char!('>'))));

        named!(field<(&[u8], Vec<i64>)>,
               ws!(pair!(take!(1),
                    preceded!(char!('='), numbers)
               )));

        named!(fields<Vec<(&[u8], Vec<i64>)>>, ws!(separated_list_complete!(tag!(", "), field)));

        for line in input.lines() {
            let bytes: &[u8] = line.as_bytes();
            let asd = fields(bytes);
            if let IResult::Done(_, nums) = asd {
                v.push(Particle {
                    pos: [nums[0].1[0], nums[0].1[1], nums[0].1[2]],
                    vel: [nums[1].1[0], nums[1].1[1], nums[1].1[2]],
                    acc: [nums[2].1[0], nums[2].1[1], nums[2].1[2]],
                });
            }
        }
        v
    }

    fn solve_pt1(mut ps: Vec<Particle>) -> usize {
        const N: usize = 10_000;
        for i in 0..N {
            for p in ps.iter_mut() {
                p.vel[0] += p.acc[0];
                p.vel[1] += p.acc[1];
                p.vel[2] += p.acc[2];
                p.pos[0] += p.vel[0];
                p.pos[1] += p.vel[1];
                p.pos[2] += p.vel[2];
            }
        }
        ps.iter()
            .enumerate()
            .min_by_key(|&(_i, p)| p.pos[0].abs() + p.pos[1].abs() + p.pos[2].abs())
            .map(|(i, _p)| i)
            .unwrap()
    }

    fn solve_pt2(ps: Vec<Particle>) -> usize {
        const N: usize = 100_000;
        let mut ps = ps.into_iter().map(Some).collect::<Vec<Option<Particle>>>();
        let mut place = HashMap::new();
        for _ in 0..N {
            for (i, ref mut p) in ps.iter_mut().enumerate().flat_map(
                |(i, o)| o.as_mut().map(|p| (i, p)),
            )
            {
                p.vel[0] += p.acc[0];
                p.vel[1] += p.acc[1];
                p.vel[2] += p.acc[2];
                p.pos[0] += p.vel[0];
                p.pos[1] += p.vel[1];
                p.pos[2] += p.vel[2];
                place.entry(p.pos).or_insert_with(|| Vec::new()).push(i);
            }
            for (_place, inds) in &place {
                if inds.len() > 1 {
                    for &i in inds {
                        ps[i] = None;
                    }
                }
            }
            place.clear();
        }
        ps.iter().flat_map(|i| i).count()
    }


    let test = "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>";

    let real = &file_to_string("input/20").unwrap();
    // let res = solve_pt1(parse_input(real));
    let res = solve_pt2(parse_input(real));
    println!("{}", res);
}

pub fn task_21() {
    #[derive(Hash, PartialEq, Eq, Clone, Debug)]
    struct Pattern {
        buffer: Vec<char>,
    }

    impl Pattern {
        fn size(&self) -> usize {
            let n = self.buffer.len();
            (n as f64).sqrt() as usize
        }

        fn from_str(s: &str) -> Self {
            let buffer = s.chars().filter(|&c| c == '.' || c == '#').collect();
            Self { buffer }
        }

        fn rotate(&self) -> Self {
            let ind = |n, x, y| y * n + x;
            let mut new_buffer = vec!['.'; self.buffer.len()];
            let s = self.size();
            for j in 0..s {
                for i in 0..s {
                    let a = ind(s, i, s - j - 1);
                    let b = ind(s, j, i);
                    new_buffer[a] = self.buffer[b];
                }
            }
            Pattern { buffer: new_buffer }
        }

        fn get(&self, x: usize, y: usize) -> char {
            let s = self.size();
            self.buffer[y * s + x]
        }

        fn mirror(&self) -> Self {
            let ind = |n, x, y| y * n + x;
            let mut new_buffer = vec!['.'; self.buffer.len()];
            let s = self.size();
            for j in 0..s {
                for i in 0..s {
                    let a = ind(s, j, i);
                    let b = ind(s, s - 1 - j, i);
                    new_buffer[a] = self.buffer[b];
                }
            }
            Pattern { buffer: new_buffer }
        }

        fn print(&self) {
            let s = self.size();
            for i in 0..self.buffer.len() {
                print!("{}", self.buffer[i]);
                if i % s == (s - 1) {
                    println!();
                }
            }
            println!();
        }
    }

    fn solve_pt1(input: &str) {
        let mut pattern = Pattern::from_str(
            ".#.
..#
###",
        );

        let mut rules = HashMap::new();
        for line in input.lines() {
            let mut s = line.split(" => ");
            let mut p = Pattern::from_str(s.next().unwrap());
            let target = Pattern::from_str(s.next().unwrap());

            for _ in 0..4 {
                rules.insert(p.clone(), target.clone());
                p = p.rotate();
            }
            p = p.mirror();
            for _ in 0..4 {
                rules.insert(p.clone(), target.clone());
                p = p.rotate();
            }
        }
        for _ in 0..18 {
            let s = pattern.size();
            if s % 2 == 0 {
                let n = s / 2;
                let mut final_buffer = vec!['X'; 3 * 3 * n * n];
                for j in 0..s / 2 {
                    for i in 0..s / 2 {
                        let block = Pattern {
                            buffer: vec![
                            pattern.get(i * 2 + 0, j * 2 + 0),
                            pattern.get(i * 2 + 1, j * 2 + 0),
                            pattern.get(i * 2 + 0, j * 2 + 1),
                            pattern.get(i * 2 + 1, j * 2 + 1),
                        ],
                        };
                        let new_block = match rules.get(&block) {
                            Some(b) => b,
                            None => {
                                block.print();
                                unreachable!();
                            }
                        };

                        let start = 3 * n * j * 3 + i * 3;
                        for y in 0..3 {
                            for x in 0..3 {
                                let ind = start + x + y * 3 * n;
                                assert_eq!(final_buffer[ind], 'X');
                                final_buffer[ind] = new_block.buffer[3 * y + x];
                            }
                        }
                    }
                }
                pattern = Pattern { buffer: final_buffer };
            } else {
                assert!(s % 3 == 0);
                let n = s / 3;
                let mut final_buffer = vec!['X'; 4 * 4 * n * n];
                for j in 0..n {
                    for i in 0..n {
                        let block = Pattern {
                            buffer: vec![
                            pattern.get(i * 3 + 0, j * 3 + 0),
                            pattern.get(i * 3 + 1, j * 3 + 0),
                            pattern.get(i * 3 + 2, j * 3 + 0),
                            pattern.get(i * 3 + 0, j * 3 + 1),
                            pattern.get(i * 3 + 1, j * 3 + 1),
                            pattern.get(i * 3 + 2, j * 3 + 1),
                            pattern.get(i * 3 + 0, j * 3 + 2),
                            pattern.get(i * 3 + 1, j * 3 + 2),
                            pattern.get(i * 3 + 2, j * 3 + 2),
                        ],
                        };
                        let new_block = match rules.get(&block) {
                            Some(b) => b,
                            None => {
                                println!("didn't find match for block!");
                                block.print();
                                unreachable!();
                            }
                        };
                        let start = 4 * n * j * 4 + i * 4;
                        for y in 0..4 {
                            for x in 0..4 {
                                let ind = start + x + y * 4 * n;
                                assert_eq!(final_buffer[ind], 'X');
                                final_buffer[ind] = new_block.buffer[4 * y + x];
                            }
                        }
                    }

                }
                pattern = Pattern { buffer: final_buffer };
            }
        }
        println!("{} pixels are on", pattern.buffer.iter().filter(|&&c| c == '#').count());
    }

    let input = &file_to_string("input/21").unwrap();

    solve_pt1(input);
}


