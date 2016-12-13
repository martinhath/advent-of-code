use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

fn read_input(s: &str) -> Result<String, io::Error> {
    let mut string = String::new();
    File::open(Path::new(s))?.read_to_string(&mut string)?;
    Ok(string)
}

pub fn task_1() {
    const INPUT: &'static str =
        "L3, R2, L5, R1, L1, L2, L2, R1, R5, R1, L1, L2, R2, R4, L4, L3, L3, R5, L1, R3, L5, L2, \
         R4, L5, R4, R2, L2, L1, R1, L3, L3, R2, R1, L4, L1, L1, R4, R5, R1, L2, L1, R188, R4, \
         L3, R54, L4, R4, R74, R2, L4, R185, R1, R3, R5, L2, L3, R1, L1, L3, R3, R2, L3, L4, R1, \
         L3, L5, L2, R2, L1, R2, R1, L4, R5, R4, L5, L5, L4, R5, R4, L5, L3, R4, R1, L5, L4, L3, \
         R5, L5, L2, L4, R4, R4, R2, L1, L3, L2, R5, R4, L5, R1, R2, R5, L2, R4, R5, L2, L3, R3, \
         L4, R3, L2, R1, R4, L5, R1, L5, L3, R4, L2, L2, L5, L5, R5, R2, L5, R1, L3, L2, L2, R3, \
         L3, L4, R2, R3, L1, R2, L5, L3, R4, L4, R4, R3, L3, R1, L3, R5, L5, R1, R5, R3, L1";
    #[derive(Clone)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    };
    impl Direction {
        fn turn(&self, other: char) -> Direction {
            match *self {
                Direction::Up => {
                    if other == 'R' {
                        Direction::Right
                    } else {
                        Direction::Left
                    }
                }
                Direction::Down => {
                    if other == 'R' {
                        Direction::Left
                    } else {
                        Direction::Right
                    }
                }
                Direction::Left => {
                    if other == 'R' {
                        Direction::Up
                    } else {
                        Direction::Down
                    }
                }
                Direction::Right => {
                    if other == 'R' {
                        Direction::Down
                    } else {
                        Direction::Up
                    }
                }
            }
        }
    }

    let mut coords = HashSet::new();
    let mut dir = Direction::Up;
    let mut x = 0i32;
    let mut y = 0i32;
    let moves = INPUT.split(", ");
    for mv in moves {
        let n = mv[1..].parse::<i32>().unwrap();
        dir = dir.turn(mv.chars().next().unwrap());
        for _ in 0..n {
            if coords.contains(&(x, y)) {
                println!("{:?} {:?}", x, y);
                println!("{:?}", x.abs() + y.abs());
                return;
            }
            coords.insert((x, y));
            match dir {
                Direction::Up => y += 1,
                Direction::Down => y -= 1,
                Direction::Left => x -= 1,
                Direction::Right => x += 1,
            }
        }
    }
    println!("{:?} {:?}", x, y);
    println!("{:?}", x.abs() + y.abs());
}

pub fn task_2() {
    let input = "LLULLLRLDLLLRLUURDDLRDLDURULRLUULUDDUDDLLLURRLDRRLDRRRLDUDLRDLRRDLLDUDUDUDRLUDUUDLLLRDURUDUULUDLRDUUUDUUDURLDUULLRDLULDUURUDRDDLDRLURLRURRDUURLRLUURURUUULLRLLULRUURLULURDLLRRUDLUDULDRDRLRULUURRDRULLRUUUDLRLDLUURRRURDLUDDRRUDRLUDRDLLLLLRULLDUDRLRRDDULDLRUURRRRRLDLDLRDURDRUUURDLRDDDDULURRRRDUURLULLLDLRULRDULRUDLRRLRDLLRLLLUDDLRDRURDDLLLLDUDRDLRURRDLRDDDLDULDRLRULUUDRRRUUULLLURRDDUULURULDURRLLULLDRURUUULRLRDRRUDRDRRDURRUUUULDRDDDUDLDDURLLRR
\
                 LDLRRRUURDLDDRLRRDLLULRULLLUDUUDUDLRULLDRUDRULLDULURDRDDLRURDDULLLLDLRDRDRDDURLURLURLUDRDDRDULULUDDRURRDLLDUURDRDDLRLLURRDLRDDULDLULURDRDLUDRRUUDULLULURRDUDRUUUDRULDLDURLRRUDURLDLRRUURRRURDLUDRLDUDRRUDUURURUDDUUDRDULRDLUDRRRLDRURLLRDDDLUDRDUDURDDDRRDDRRRLLRRDDLDDLRUURRURDLLDRLRRDLLUDRRRURURLRDRLLRLRLRULLRURLDLRRULLRRRDULUUULDRDLLURDDLDLRDRLUUDLLUDDLDRRLDLRUDRUDLLUURLLULURUDUDRLULLUDRURDDLDLDDUDLRDDRRURLRLLUDDUDRUURRURRULDRLDDRLLRRLDDURRDLDULLLURULLLRUURLRRRRUUULRLLLURRLRLRUDRDUUUDUUUDDLULLDLLLLDLDRULDRUUULDDDLURLDLRLULRUDDDDURDDLU
\
                 RURLURRDLDULLULDDDLRUULLUURLRUDRUDRRUDDLDDDDRRDLRURLRURLDDDUDDUURRDRULDRRRULRDRDDLRUDULRLURDUUDRRLDLRDRURDLDRRRRDRURUUDDDLLRDRDUDUDUDLLULURULRRLRURUULUULDDDDURULRULLRUDUURLURDUDLUDLUDRLLDUUDUULRLRLUUDRDULDULRURDRRRULRUDLRURDDULUDULLRLRURURUULLULDRURLLRRUUDDUUURRDLURUURULRDRRDDUDULRDDLUDLURURUURDRULLRDDLLRDDLDRDUDRRDLUURRLRLUURRULUDURLDDRLLURRDDDLDDRURULLDDRLUDDLRLURDUDULLRDULLLDLLUDDRUDRUDDUUDRDRULRL
\
                 RLRDRDULULUDLUDRDRLUDLDLLUDURULDDDUDLRURLLRLRLDLDRLDURDLRRURLULLULURLLDRRDRLUDRLRDLLULRULURRURURUULRDUDLLRDLRRRRRLUURDRRRDLRUDLLDLLDLRUUUDLLLDDDLRDULLRUUDDRLDDURRRDLRLRLDDDDLRDRULLUURUUDRRLLRLLRDDLLRURRRRDRULRRLLRLLLRLDRRLDDDURRURLDURUURRLRLRLDRURULLRLRUDLDUURDLLRLDLURUUUDLLRDRDDDDDDRLDRRRLRRRRURUDLDDRDLLURUDLRRLDDDLUDUDUULRDULULUDDULUUDLLLLRLDDUUULRLRDULURDURRRURRULURRRDRDLDDURDLURUDURRRDDRLRLUDLUDDLUULLDURLURDDUDDLRUUUDRLLDRURL
\
                 ULUDLLUDDULRUURDRURDUDUDLUURDDDRRLUDURURDRURRLDRDURLRLLRRDDRRDRRRUULURUDURUDULRRRRDDLDURRLRRDUDDDRLLLULDRLRLURRDUURDURRRURRDLUDUDDRLDLURRRDDRLLRDRDDRDURRRRLURRLUDDURRULRUDUDULDRUDDRULLUUULDURRRLDRULLURULLRUDLDUDDLDULDLUUDRULULDLLDRULLRUULDUDUUDRLRRLDLUULUDLLDDRLRRDDLLURURDULRRDDRURDRLRLULDLDURULLUUUDURURDLDUDDDDUUULUDLUURRULLDLRLURDLURLRLDDURRLDDRRRDUUULLUULDLLDLLDDRLRRUDLULDRLULDULULRRLRULUUURURUUURDUUDDURLLUDDRLRDDLUURRUULRDLDDRLULUULRDRURLUURDRDUURUDLRR";
    //     let input = "ULL
    // RRDDD
    // LURDL
    // UUUUD";
    if false {
        // part 1
        let mut x = 1usize;
        let mut y = 1usize;
        for line in input.lines() {
            for c in line.chars() {
                use std::cmp::min;
                match c {
                    'U' => {
                        y = y.saturating_sub(1);
                    }
                    'L' => {
                        x = x.saturating_sub(1);
                    }
                    'R' => {
                        x = min(x + 1, 2);
                    }
                    'D' => {
                        y = min(y + 1, 2);
                    }
                    _ => unreachable!(),
                }
            }
            print!("{}", x + y * 3 + 1);
        }
    }
    if true {
        // part 2
        let mut x = 0usize;
        let mut y = 2usize;
        for line in input.lines() {
            fn is_outside(x: usize, y: usize) -> bool {
                match y {
                    0 => x != 2,
                    1 => x == 0 || x == 4,
                    3 => x == 0 || x == 4,
                    4 => x != 2,
                    _ => false,
                }
            }
            fn coord(x: usize, y: usize) -> char {
                // assume is_inside
                match y {
                    0 => '1',
                    1 => {
                        match x {
                            1 => '2',
                            2 => '3',
                            3 => '4',
                            _ => unreachable!(),
                        }
                    }
                    2 => {
                        match x {
                            0 => '5',
                            1 => '6',
                            2 => '7',
                            3 => '8',
                            4 => '9',
                            _ => unreachable!(),
                        }
                    }
                    3 => {
                        match x {
                            1 => 'A',
                            2 => 'B',
                            3 => 'C',
                            _ => unreachable!(),
                        }
                    }
                    _ => 'D',
                }
            };
            for c in line.chars() {
                use std::cmp::min;
                let mut x1 = x;
                let mut y1 = y;
                match c {
                    'U' => {
                        y1 = y.saturating_sub(1);
                    }
                    'L' => {
                        x1 = x.saturating_sub(1);
                    }
                    'R' => {
                        x1 = min(x + 1, 4);
                    }
                    'D' => {
                        y1 = min(y + 1, 4);
                    }
                    _ => unreachable!(),
                }
                if !is_outside(x1, y1) {
                    x = x1;
                    y = y1;
                }
            }
            print!("{}", coord(x, y));
        }
    }
}

pub fn task_3() {
    let is_valid = |ns: &Vec<u32>| {
        let sum: u32 = ns.iter().sum();
        let max: u32 = ns.iter().max().cloned().unwrap();
        max < sum - max
    };
    let input = read_input("day-3").expect("Failed to read day 3 input");
    // Part 1
    if false {
        let n = input.lines()
            .filter(|line| {
                is_valid(&line.split(" ")
                    .filter(|n| n.len() > 0)
                    .flat_map(str::parse::<u32>)
                    .collect::<Vec<_>>())
            })
            .count();
        println!("n: {}", n);
    }
    // Part 2
    if true {
        let lines = input.lines()
            .map(|s| {
                s.split(" ")
                    .filter(|n| n.len() > 0)
                    .flat_map(str::parse::<u32>)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let n = lines.chunks(3)
            .flat_map(|lns| {
                let a = vec![lns[0][0], lns[1][0], lns[2][0]];
                let b = vec![lns[0][1], lns[1][1], lns[2][1]];
                let c = vec![lns[0][2], lns[1][2], lns[2][2]];
                vec![a, b, c]
            })
            .filter(is_valid)
            .count();
        println!("{}", n);
    }
}

use regex::*;
use std::collections::HashMap;

pub fn task_4() {
    let input = read_input("day-4").expect("Failed to read day 4 input");
    let re = Regex::new(r"(?P<chars>([a-z]+-)+)(?P<n>\d+)\[(?P<checksum>[a-z]+)\]").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        for cap in re.captures_iter(line) {
            let chars = cap.name("chars").unwrap_or("");
            let map = chars.chars()
                .filter(|&c| c.is_alphanumeric())
                .fold(HashMap::new(), |mut map, e| {
                    *map.entry(e).or_insert(0) += 1;
                    map
                });
            let mut letters = map.iter()
                .collect::<Vec<_>>();
            letters.sort_by_key(|&(k, v)| - 255 * v + *k as i32);
            let is_valid = cap.name("checksum")
                .map(|i| i.chars()
                    .zip(letters)
                    .all(|(a, (&b, _))| a == b))
                .unwrap_or(false);
            let n = cap.name("n").and_then(|s| s.parse::<i32>().ok()).unwrap();
            if is_valid {
                sum += n
            }
            let transform = |c: char| {
                if c.is_alphanumeric() {
                    let mut u = c as i32;
                    let a = b'a' as i32;
                    u += n - a;
                    u %= 26;
                    u += a;
                    (u as u8) as char
                } else {
                    c
                }
            };
            let shifted = chars.clone()
                .chars()
                .map(transform)
                .collect::<String>();
            if shifted.contains("north") {
                println!("{:?} {}", shifted, n);
            }
        }
    }
    println!("{}", sum);
}

use crypto::md5::Md5;
use crypto::digest::Digest;

pub fn task_5() {
    let input = "reyedfim";
    // let input = "abc";
    let mut digest = Md5::new();
    let mut buff = [0; 16];
    let mut prev = 0;
    let mut password = [None; 8];
    for _ in 0..8 {
        for i in prev.. {
            let pass = format!("{}{}", input, i);
            digest.input(pass.as_bytes());
            digest.result(&mut buff);
            digest.reset();
            if buff[0] == 0 && buff[1] == 0 && buff[2] < 16 {
                let pos = (buff[2] & 0xf) as usize;
                if pos > 7 {
                    continue;
                }
                let n = (buff[3] & 0xf0) >> 4;
                if password[pos].is_none() {
                    password[pos] = Some(n);
                    prev = i + 1;
                    break;
                }
            }
        }
    }
    for c in password.iter() {
        print!("{:x}", c.unwrap());
    }
    print!("\n");
}

pub fn task_6() {
    let input = read_input("day-6").unwrap();
    let mut freqs = [[0; 26]; 8];
    for line in input.lines() {
        line.chars()
            .enumerate()
            .map(|(i, c)| freqs[i][(c as u8 - b'a') as usize] += 1)
            .count();
    }
    for lst in freqs.iter() {
        let c = lst.iter()
            .enumerate()
            // .max_by_key(|&(_, n)| n)
            .min_by_key(|&(_, n)| n)
            .map(|(i, _)| i)
            .unwrap();
        print!("{}", (c as u8 + b'a') as char);
    }
}

#[allow(dead_code)]
fn contains_abba(s: &[u8]) -> bool {
    if s.len() < 4 { return false }
    s.windows(4).any(|chunk| {
        chunk[0] == chunk[3] && chunk[1] == chunk[2] && chunk[0] != chunk[1]
    })
}

pub fn task_7() {
    let input = read_input("day-7").unwrap();
    let mut n = 0;
    'f: for l in input.lines() {
        let line = l.as_bytes();
        let mut outside = Vec::new();
        let mut inside = Vec::new();
        let mut i = 0;
        while let Some(a) = line[i..].iter().position(|&c| c == b'[') {
            let new_i;
            let ind = i + a + 1;
            if let Some(b) = line[i + a..].iter().position(|&c| c == b']') {
                inside.push((ind, ind + b - 1));
                new_i = ind + b;
            } else {
                unreachable!("no matching ]");
            }
            outside.push((i, i + a));
            i = new_i;
        }
        outside.push((i, line.len()));

        let is_valid =
            outside.iter()
                .flat_map(|&(a, b)|
                          line[a..b].windows(3)
                               .filter(|s| s[0] == s[2] && s[0] != s[1]))
                .filter(|aba| {
                    let rev = [aba[1], aba[0], aba[1]];
                    inside.iter().any(|&(a, b)| line[a..b].windows(3).any(|s| s == rev))
                })
                .next()
                .is_some();

        if is_valid {
            n += 1;
        }
    }
    println!("{}", n);
}

pub fn task_8() {
    #[derive(Debug)]
    enum Command {
        Rect(usize, usize),
        RotateRow(usize, usize),
        RotateCol(usize, usize),
    };
    impl Command {
        fn from_str(s: &str) -> Command {
            let words = s.split(" ").collect::<Vec<_>>();
            if words[0] == "rect" {
                let mut i = words[1].split("x");
                let x = i.next().unwrap().parse::<usize>().unwrap();
                let y = i.next().unwrap().parse::<usize>().unwrap();
                Command::Rect(x, y)
            } else {
                let n = words[2][2..].parse::<usize>().unwrap();
                let m = words[4].parse::<usize>().unwrap();
                if words[1].as_bytes()[0] == b'c' {
                    Command::RotateCol(n, m)
                } else {
                    Command::RotateRow(n, m)
                }
            }
        }
    }
    let mut grid = [[false; 50]; 6];
    for cmd in read_input("day-8")
            .expect("failed to read day-8")
            .lines()
            .map(Command::from_str) {
        match cmd {
            Command::Rect(x, y) => {
                for j in 0..y {
                    for i in 0..x {
                        grid[j][i] = true;
                    }
                }
            },
            Command::RotateRow(y, n) => {
                let row = grid[y];
                let mut buf = [false; 50];
                for i in 0..50 {
                    buf[(i + n) % 50] = row[i];
                }
                grid[y] = buf;
            },
            Command::RotateCol(x, n) => {
                let mut buf = [false; 6];
                for i in 0..6 {
                    buf[(i + n) % 6] = grid[i][x];
                }
                for i in 0..6 {
                    grid[i][x] = buf[i];
                }
            }
        }
    }
    println!("{}", grid.iter().map(|line| line.iter().filter(|&&e| e).count()).sum::<usize>());
    for j in 0..6 {
        for i in 0..50 {
            print!("{}", if grid[j][i] { '#' } else { '.' });
        }
        print!("\n");
    }
}

pub fn task_9() {
    fn get_nums(s: &str) -> (usize, usize) {
        let mut iter = s.split(|b| b == 'x')
            .flat_map(|n| n.parse::<usize>());
        (iter.next().unwrap(),
         iter.next().unwrap())
    };
    fn solve(input: &str) -> usize {
        let mut len = 0;
        let mut start = 0;
        while let Some(a) = input[start..].find(|c| c == '(') {
            len += a;
            let first_paren = start + a + 1;
            let b = input[first_paren..].find(|c| c == ')').expect("No matching )");
            let last_paren = first_paren + b;
            let (m, n) = get_nums(&input[first_paren..last_paren]);
            len += solve(&input[last_paren + 1..last_paren + 1 + m]) * n;

            start = last_paren + 1 + m;
        }
        len + (input.len() - start)
    }
    assert_eq!(solve("(3x3)XYZ"), 9);
    assert_eq!(solve("X(8x2)(3x3)ABCY"), 20);
    assert_eq!(solve("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
    assert_eq!(solve("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"), 445);
    let input = read_input("day-9").unwrap().chars().filter(|c| !c.is_whitespace()).collect::<String>();
    println!("len: {}", solve(&input));
}

pub fn task_10() {
    let input = read_input("day-10").expect("failed to read input");
    enum Dst {
        Bot(usize),
        Output(usize),
    };
    fn parse_dst(s: &str) -> Dst {
        let mut iter = s.split(" ");
        let t = iter.next();
        let n = iter.next().unwrap().parse::<usize>().unwrap();
        match t {
            Some("bot") => { Dst::Bot(n) },
            Some("output") => { Dst::Output(n) },
            _ => unreachable!(),
        }
    }

    let init_reg = Regex::new("value (?P<val>\\d+) goes to bot (?P<bot>\\d+)").unwrap();
    let give_reg = Regex::new("bot (?P<val>\\d+) gives low to (?P<low>(output|bot) \\d+) and high to (?P<high>(output|bot) \\d+)").unwrap();
    let mut bots = HashMap::new();
    let mut outputs = HashMap::new();
    let mut moves = Vec::new();
    for line in input.lines() {
        if let Some(caps) = init_reg.captures(line) {
            // value 3 goes to bot 1
            let val = caps.name("val").unwrap().parse::<usize>().unwrap();
            let dst = caps.name("bot").unwrap().parse::<usize>().unwrap();
            bots.entry(dst).or_insert(Vec::new()).push(val);
        } else if let Some(caps) = give_reg.captures(line) {
            // bot 0 gives low to output 2 and high to output 0
            let val = caps.name("val").unwrap().parse::<usize>().unwrap();
            let low = parse_dst(caps.name("low").unwrap());
            let high = parse_dst(caps.name("high").unwrap());
            moves.push((val, low, high));
        } else {
            panic!(format!("No matching regex for \"{}\"", line));
        }
    }

    fn remove_minmax(v: &mut Vec<usize>) -> Option<(usize, usize)> {
        if v.len() < 2 { return None; }
        let min = v.iter().enumerate().min_by_key(|&(_, n)| n).unwrap().0;
        let max = v.iter().enumerate().max_by_key(|&(_, n)| n).unwrap().0;
        // Remove the largest index first, so the indices doesnt mess up each other
        if min > max {
            let a = v.swap_remove(min);
            let b = v.swap_remove(max);
            Some((a, b))
        } else {
            let b = v.swap_remove(max);
            let a = v.swap_remove(min);
            Some((a, b))
        }
    }
    if let Some((b, _)) = bots.iter()
        .filter(|&(_, vals)|
                vals.iter()
                    .filter(|&&n| n == 61 || n == 17)
                    .count() == 2)
        .next() {
        println!(">>{}<<", b);
    }

    while moves.len() > 0 {
        moves.retain(|&(ref bot, ref low, ref high)| {
            let (a, b) = {
                let vals = bots.get_mut(&bot);
                if vals.is_none() { return true; }
                let mm = remove_minmax(&mut vals.unwrap());
                if mm.is_none() { return true; }
                mm.unwrap()
            };
            match *low {
                Dst::Bot(n) => bots.entry(n).or_insert(Vec::new()).push(a),
                Dst::Output(n) => outputs.entry(n).or_insert(Vec::new()).push(a),
            }
            match *high {
                Dst::Bot(n) => bots.entry(n).or_insert(Vec::new()).push(b),
                Dst::Output(n) => outputs.entry(n).or_insert(Vec::new()).push(b),
            }
            if let Some((b, _)) = bots.iter()
                .filter(|&(_, vals)|
                        vals.iter()
                            .filter(|&&n| n == 61 || n == 17)
                            .count() == 2)
                .next() {
                println!("{}", b);
            }
            false
        });
    }
    let prod: usize = (0..3).flat_map(|i| outputs.get(&i).unwrap()).product();
    println!("{}", prod);
}

pub fn task_11() {
    use std::hash::{Hash, Hasher};


    #[derive(Debug, Hash, Eq, PartialEq, Clone, Ord, PartialOrd)]
    enum E<'a> {
        Generator(&'a str),
        Microchip(&'a str),
    }

    impl<'a> E<'a> {
        fn is_generator(&self) -> bool {
            match self {
                &E::Generator(_) => true,
                &E::Microchip(_) => false,
            }
        }

        fn is_microchip(&self) -> bool {
            !self.is_generator()
        }

        fn label(&self) -> &'a str {
            match self {
                &E::Generator(s) => s,
                &E::Microchip(s) => s,
            }
        }
    }

    #[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd)]
    struct State<'a> {
        game: Vec<(E<'a>, usize)>,
        elevator: usize,
        levels: usize,
        neighbours: Vec<State<'a>>,
    }

    impl<'a> State<'a> {
        fn id(&self) -> usize {
            let mut id = 0;
            id += self.elevator;
            let mut f = 1;
            for &(_, n) in self.game.iter() {
                f *= 4;
                id += f * n;
            }
            id
        }
        fn neighbours(&self) -> Vec<State<'a>> {
            self.neighbours.clone()
        }

        fn generate_neighbours(&mut self) {
            let mut v = Vec::with_capacity(16);
            let inds = self.game.iter()
                .enumerate()
                .filter(|&(_, &(_, n))| n == self.elevator)
                .map(|(i, _)| i)
                .collect::<Vec<usize>>();
            if self.elevator > 1 {
                // Move stuff down
                let n = inds.len();
                for i in 0..n {
                    for j in (i + 1)..n {
                        let mut s = self.clone();
                        s.game[inds[i]].1 -= 1;
                        s.game[inds[j]].1 -= 1;
                        s.elevator -= 1;
                        v.push(s);
                    }
                    let mut s = self.clone();
                    s.game[inds[i]].1 -= 1;
                    s.elevator -= 1;
                    v.push(s);
                }
            }
            if self.elevator < self.levels {
                // Move stuff up
                let n = inds.len();
                for i in 0..n {
                    for j in (i + 1)..n {
                        let mut s = self.clone();
                        s.game[inds[i]].1 += 1;
                        s.game[inds[j]].1 += 1;
                        s.elevator += 1;
                        v.push(s);
                    }
                    let mut s = self.clone();
                    s.game[inds[i]].1 += 1;
                    s.elevator += 1;
                    v.push(s);
                }
            }
            v.retain(State::no_fries);
            self.neighbours = v;
        }

        fn no_fries(&self) -> bool {
            (0..self.levels).all(|level| {
                let gens = || self.game.iter()
                    .filter(|&&(ref e, l)| level == l && e.is_generator());
                gens().next().is_none() || self.game.iter()
                    .filter(|&&(ref e, l)| level == l && e.is_microchip())
                    .all(|&(ref e, _)| {
                        // For each chip, check that its generator is in `gens`.
                        // If not, there is a gen on the same floor that isn't
                        // the correct gen for this chip, and it will be fried.
                        let label = e.label();
                        gens().filter(|&&(ref g, _)| {
                           g.is_generator() && g.label() == label
                        }).next().is_some()
                    })
            })
        }

        fn is_done(&self) -> bool {
            self.game.iter().all(|&(_, n)| n == self.levels)
        }

        fn print_short(&self) {
            print!("{}", self.elevator);
            for &(_, e) in &self.game {
                print!("{}", e);
            }
            println!("");
        }
    }

    impl<'a> Hash for State<'a> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.elevator.hash(state);
            for &(_, n) in &self.game {
                n.hash(state);
            }
        }
    }

//     let input = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
// The second floor contains a hydrogen generator.
// The third floor contains a lithium generator.
// The fourth floor contains nothing relevant.";

    let input = "The first floor contains a thulium generator, a thulium-compatible microchip, a plutonium generator, a strontium generator, a elerium generator, a elerium-compatible microchip, a dilithium generator, and a dilithium-compatible microchip.
The second floor contains a plutonium-compatible microchip and a strontium-compatible microchip.
The third floor contains a promethium generator, a promethium-compatible microchip, a ruthenium generator, and a ruthenium-compatible microchip.
The fourth floor contains nothing relevant.";
    let reg = Regex::new(r"([a-zA-Z-]+) (microchip|generator)").unwrap();
    let mut state = State {
        game: vec![],
        elevator: 1,
        levels: 4,
        neighbours: vec![],
    };
    for (lineno, line) in input.lines().enumerate() {
        for cap in reg.captures_iter(line) {
            let m = cap.at(1).unwrap();
            let t = cap.at(2).unwrap();
            let e = match t {
                "generator" => { E::Generator(m) }
                "microchip" => { E::Microchip(m.split("-").next().unwrap()) }
                _ => unreachable!(&format!("What is {}", t)),
            };
            state.game.push((e, lineno + 1));
        }
    }

    #[derive(PartialEq, Eq, PartialOrd)]
    struct QueueE {
        id: usize,
        steps: isize,
        score: isize,
    }
    use std::cmp;
    impl Ord for QueueE {
        fn cmp(&self, other: &QueueE) -> cmp::Ordering {
            (self.score, self.steps, self.id).cmp(&(other.score, other.steps, other.id))
        }
    }

    use std::collections::BinaryHeap;

    state.generate_neighbours();
    // Mapping id -> State
    let mut states: HashMap<usize, State> = HashMap::new();
    let init_state_id = state.id();
    states.insert(init_state_id, state);
    // Mapping id -> score
    let mut table: HashMap<usize, isize> = HashMap::new();
    table.insert(init_state_id, 0);
    // Queue (score, id)
    let mut state_queue: BinaryHeap<QueueE> = BinaryHeap::new();
    state_queue.push( QueueE { steps: 0, id: init_state_id, score: 0 });

    let mut iters = 0;
    'outer: while let Some(e) = state_queue.pop() {
        let base = e.steps;
        let s_id = e.id;
        for neigh in states.get(&s_id).unwrap().neighbours() {
            let neigh_id = neigh.id();
            states.entry(neigh_id).or_insert_with(|| {
                let mut n = neigh.clone();
                n.generate_neighbours();
                if neigh.is_done() {
                    println!("iters: {}", iters);
                    println!("{}", -base + 1);
                    panic!();
                }
                n
            });

            let mut prev = table.entry(neigh_id).or_insert(-99999);
            if *prev < base - 1 {
                *prev = base - 1;
                let score = *prev;// + neigh.game.iter().map(|&(_, n)| n as isize).sum::<isize>();
                let e = QueueE {
                    id: neigh_id,
                    steps: *prev,
                    score: score,
                };
                state_queue.push(e);
            }
        }
        iters += 1;
    }
}

pub fn task_12() {
    enum Op<'a> {
        Reg(&'a str),
        Val(i32)
    };
    impl<'a> Op<'a> {
        fn make(s: &'a str) -> Self {
            s.parse::<i32>().map(Op::Val).unwrap_or(Op::Reg(s))
        }

        fn value(&self, env: &HashMap<&str, i32>) -> i32 {
            match self {
                &Op::Reg(r) => env.get(r).cloned().unwrap_or(0),
                &Op::Val(v) => v
            }
        }
    }
    enum Instruction<'a> {
        Cpy(Op<'a>, &'a str),
        Inc(&'a str),
        Dec(&'a str),
        Jnz(Op<'a>, Op<'a>),
    };
    impl<'a> From<&'a str> for Instruction<'a> {
        fn from(s: &'a str) -> Self {
            let mut spl = s.split(" ");
            let op = spl.next().unwrap();
            let a = spl.next().unwrap();
            match op {
                "cpy" => {
                    let b = spl.next().unwrap();
                    let from = Op::make(a);
                    Instruction::Cpy(from, b)
                }
                "inc" => Instruction::Inc(a),
                "dec" => Instruction::Dec(a),
                "jnz" => {
                    let a = Op::make(a);
                    let b = Op::make(spl.next().unwrap());
                    Instruction::Jnz(a, b)
                }
                _ => unreachable!()

            }
        }
    }

    let input = "cpy 1 a
cpy 1 b
cpy 26 d
jnz c 2
jnz 1 5
cpy 7 c
inc d
dec c
jnz c -2
cpy a c
inc a
dec b
jnz b -2
cpy c b
dec d
jnz d -6
cpy 19 c
cpy 14 d
inc a
dec d
jnz d -2
dec c
jnz c -5";
//     let input = "cpy 41 a
// inc a
// inc a
// dec a
// jnz a 2
// dec a";

    let mut registers = HashMap::new();
    let instructions = input.lines()
        .map(Instruction::from)
        .collect::<Vec<_>>();
    registers.insert("c", 1);
    let mut pc = 0i32;
    let n_instr = instructions.len() as i32;
    while pc < n_instr {
        match &instructions[pc as usize] {
            &Instruction::Inc(reg) => {
                *registers.entry(reg).or_insert(0) += 1;
            },
            &Instruction::Dec(reg) => {
                *registers.entry(reg).or_insert(0) -= 1;
            },
            &Instruction::Cpy(ref from, to) => {
                let v =from.value(&registers);
                registers.insert(to, v);
            },
            &Instruction::Jnz(ref c, ref n) => {
                if c.value(&registers) != 0 {
                    let val = n.value(&registers);
                    pc += val - 1;
                }
            },
        }
        pc += 1;
    }
    println!("a={}", registers.get("a").unwrap());
}

pub fn task_13() {
    #[derive(Clone, Copy)]
    struct Node {
        coord: (usize, usize),
        steps: usize,
    }
    fn is_space(t: (usize, usize)) -> bool {
        let (x, y) = t;
        let input = 1362;
        let f = x * x + 3 * x + 2 * x * y + y + y * y + input;
        f.count_ones() % 2 == 0
    }
    let mut visited = HashSet::<(usize, usize)>::new();
    let mut queue = vec![Node { coord: (1, 1), steps: 0 }];
    let mut queue_i = 0;
    let target = (31, 39);
    while queue_i < queue.len() {
        let e = queue[queue_i];
        if e.steps > 50 {
            println!("{} targets below 50 steps", visited.len());
            break;
        }
        if e.coord == target {
            println!("{}", e.steps);
            break;
        }
        let (x, y) = e.coord;
        if x > 0 {
            let left = (x - 1, y);
            if is_space(left) && !visited.contains(&left) {
                queue.push(Node { coord: left, steps: e.steps + 1});
            }
        }
        if y > 0 {
            let up = (x, y - 1);
            if is_space(up) && !visited.contains(&up) {
                queue.push(Node { coord: up, steps: e.steps + 1});
            }
        }
        let down = (x, y + 1);
        if is_space(down) && !visited.contains(&down) {
            queue.push(Node { coord: down, steps: e.steps + 1});
        }

        let right = (x + 1, y);
        if is_space(right) && !visited.contains(&right) {
            queue.push(Node { coord: right, steps: e.steps + 1});
        }
        visited.insert(e.coord);

        queue_i += 1;
    }
}
