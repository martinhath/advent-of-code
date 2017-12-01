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
                    1 | 3 => x == 0 || x == 4,
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
    for l in input.lines() {
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
                Dst::Output(n) => outputs.entry(n)
                    .or_insert_with(Default::default)
                    .push(b),
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
            match *self {
                E::Generator(_) => true,
                E::Microchip(_) => false,
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

        fn short_label(&self) -> String {
            match self {
                &E::Generator(s) => format!("G{}", s.chars().next().unwrap()),
                &E::Microchip(s) => format!("M{}", s.chars().next().unwrap()),
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
        fn print(&self) {
            for level in (1..5).rev() {
                if self.elevator == level {
                    print!("E ");
                } else {
                    print!("  ");
                }
                for &(ref e, l) in &self.game {
                    if level == l {
                        print!("{:2} ", e.short_label());
                    } else {
                        print!(" . ");
                    }
                }
                print!("\n");
            }
        }

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

        // fn print_short(&self) {
        //     print!("{}", self.elevator);
        //     for &(_, e) in &self.game {
        //         print!("{}", e);
        //     }
        //     println!("");
        // }
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
    let input = "The first floor contains a thulium generator, a thulium-compatible microchip, a plutonium generator, a strontium generator, a elerium generator, a elerium-compatible microchip, a dilithium generator, and a dilithium-compatible microchip, a emerium generator, a emerium-compatible microchip, a enerium generator, a enerium-compatible microchip, a ejerium generator, a ejerium-compatible microchip, a ekerium generator, a ekerium-compatible microchip.
The second floor contains a plutonium-compatible microchip and a strontium-compatible microchip.
The third floor contains a promethium generator, a promethium-compatible microchip, a ruthenium generator, and a ruthenium-compatible microchip.
The fourth floor contains nothing relevant.";

    // Anders input
//     let input = "The first floor contains a strontium generator, a strontium-compatible microchip, a plutonium generator, and a plutonium-compatible microchip.
// The second floor contains a thulium generator, a ruthenium generator, a ruthenium-compatible microchip, a curium generator, and a curium-compatible microchip.
// The third floor contains a thulium-compatible microchip.
// The fourth floor contains nothing relevant.";


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

    // #[derive(PartialEq, Eq, PartialOrd)]
    struct QueueE {
        id: usize,
        steps: isize,
        score: isize,
    }
    use std::cmp;
    impl PartialEq for QueueE {
        fn eq(&self, other: &Self) -> bool {
            self.score == other.score && self.steps == other.steps && self.id == other.id
        }
    }
    impl Eq for QueueE { }
    impl PartialOrd for QueueE {
        fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
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
        if false
        {
            println!("steps for popped state: {}", base);
            let v = state_queue.iter().map(|qe| qe.steps).collect::<Vec<_>>();
            println!("{:?}", v);
            println!("#{}", iters);
            states.get(&s_id).unwrap().print();
            println!("");
        }
        for neigh in states.get(&s_id).unwrap().neighbours() {
            let neigh_id = neigh.id();
            states.entry(neigh_id).or_insert_with(|| {
                let mut n = neigh.clone();
                n.generate_neighbours();
                if neigh.is_done() {
                    println!("iters: {}", iters);
                    println!("{}", base + 1);
                    panic!();
                }
                n
            });

            let mut prev = table.entry(neigh_id).or_insert(99999);
            if *prev > base + 1 {
                *prev = base + 1;
                let score = *prev;
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
        let input = 1364;
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

extern crate rayon;

pub fn task_14() {
    const SALT: &'static str = "ihaygndm";
    // const SALT: &'static str = "abc";

    fn u8s_to_string(s: &[u8]) -> [u8; 32] {
        fn u8_to_hex(b: u8) -> (u8, u8) {
            fn helper(b: u8) -> u8 {
                if b < 10 {
                    (b + b'0')
                } else {
                    (b - 10 + b'a')
                }
            }
            let x = (b & 0xf0) >> 4;
            let y = b & 0xf;
            (helper(x), helper(y))
        }

        let mut ret = [0; 32];
        for (i, &b) in s.iter().enumerate() {
            let (x, y) = u8_to_hex(b);
            ret[2 * i] = x;
            ret[2 * i + 1] = y;
        }
        ret
    }

    use self::rayon::prelude::*;
    fn extend_hash_to(hashes: &mut Vec<[u8; 32]>, n: usize) {
        if hashes.len() < (n + 1) {
            let mut new_hashes = vec![];
            (hashes.len()..(n + 1)).into_par_iter().map(|i| {
                let mut digest = Md5::new();
                let hash = {
                    let key = format!("{}{}", SALT, i);
                    let mut buff  = [0; 16];
                    digest.input(&key.as_bytes());
                    digest.result(&mut buff);
                    digest.reset();
                    for _ in 0..2016 {
                        let inp = u8s_to_string(&buff);
                        digest.input(&inp);
                        digest.result(&mut buff);
                        digest.reset();
                    }
                    buff
                };
                u8s_to_string(&hash)
            }).collect_into(&mut new_hashes);
            hashes.extend(new_hashes);
        }
    };

    let mut hashes = Vec::new();
    let mut matches_found = 0;
    let mut i = 1;
    while matches_found < 64 {
        extend_hash_to(&mut hashes, i);
        let hash = hashes[i].clone();
        let mut triple = hash.windows(3)
            .filter_map(|s| {
                if s[0] == s[1] && s[1] == s[2] {
                    Some(s[0])
                } else {
                    None
                }
            });
        if let Some(c) = triple.next() {
            extend_hash_to(&mut hashes, i + 1000);
            for j in (i + 1)..(i + 1001) {
                let hash5 = hashes[j];
                if hash5.windows(5).filter(|s| s.iter().all(|&d| d == c)).next().is_some() {
                    matches_found += 1;
                    println!("[{:2}] Match at index {}, matching at {} ({})", matches_found, i, j, c);
                    break;
                }
            }
        }
        i += 1;
    }
}

pub fn task_15() {

    let input = "Disc #1 has 5 positions; at time=0, it is at position 2.
Disc #2 has 13 positions; at time=0, it is at position 7.
Disc #3 has 17 positions; at time=0, it is at position 10.
Disc #4 has 3 positions; at time=0, it is at position 2.
Disc #5 has 19 positions; at time=0, it is at position 9.
Disc #6 has 7 positions; at time=0, it is at position 0.
Disc #7 has 11 positions; at time=0, it is at position 0.";

    struct Disk {
        size: usize,
        pos: usize,
    }
    impl Disk {
        fn next(&mut self) {
            self.pos += 1;
            if self.pos == self.size {
                self.pos = 0;
            }
        }

        fn advance_to(&mut self, n: usize) {
            self.pos += n;
            if self.pos >= self.size {
                self.pos = self.pos % self.size;
            }
        }

        fn from_str(s: &str) -> Self {
            let s = &s[..s.len() - 1];
            let mut words = s.split(" ");
            let size = words.nth(3).unwrap().parse::<usize>().unwrap();
            let pos  = words.nth(7).unwrap().parse::<usize>().unwrap();

            Disk {
                size: size,
                pos: pos,
            }
        }
    }
    let mut disks = input.lines().map(Disk::from_str).collect::<Vec<_>>();
    for (i, ref mut disk) in disks.iter_mut().enumerate() {
        disk.advance_to(i + 1);
    }
    for time in 0.. {
        if disks.iter().all(|d| d.pos == 0) {
            println!("Time {}", time);
            break;
        }
        disks.iter_mut().map(Disk::next).count();
    }
}

pub fn task_16() {
    fn step(a: &mut Vec<bool>) {
        let b: Vec<bool> = a.iter().rev().map(|b| !b).collect();
        a.push(false);
        a.extend(b);
    }
    fn checksum(a: &[bool]) -> Vec<bool> {
        let mut v = Vec::with_capacity(a.len() / 2);
        a.chunks(2).map(|slice| {
            v.push(slice[0] == slice[1])
        }).count();
        if v.len() % 2 == 0 {
            checksum(&v)
        } else {
            v
        }
    }
    let input = "10001001100000001";
    let mut vec = input.chars().map(|c| c == '1').collect::<Vec<bool>>();
    // let target_len = 272;
    let target_len = 35651584;
    while vec.len() < target_len {
        step(&mut vec);
    }
    let checksum = checksum(&vec[..target_len])
        .iter()
        .cloned()
        .map(|b| if b { '1' } else { '0' })
        .collect::<String>();
    println!("{}", checksum);
}

pub fn task_17() {
    #[derive(Copy, Clone)]
    struct Room {
        x: usize,
        y: usize
    };
    impl Room {
        fn start() -> Self {
            Room {
                x: 0,
                y: 0,
            }
        }
        fn is_vault(&self) -> bool {
            self.x == 3 && self.y == 3
        }
        fn up(&self)    -> Self { Room { x: self.x, y: self.y - 1 } }
        fn down(&self)  -> Self { Room { x: self.x, y: self.y + 1 } }
        fn left(&self)  -> Self { Room { x: self.x - 1, y: self.y } }
        fn right(&self) -> Self { Room { x: self.x + 1, y: self.y } }
    }
    fn is_open(b: u8) -> bool {
        b > 10
    }

    struct Queue<T> {
        v: Vec<T>,
        i: usize
    }

    impl<T> Queue<T>
    where T: Clone {
        fn new() -> Self {
            Queue {
                v: Vec::new(),
                i: 0
            }
        }

        fn next(&mut self) -> T {
            self.v.remove(0)
        }

        fn push(&mut self, e: T) {
            self.v.push(e);
        }

        fn len(&self) -> usize {
            self.v.len() - self.i
        }
    }

    fn longest(room: Room, passcode: &mut Vec<u8>, n: &mut usize) {
        let mut digest = Md5::new();
        if room.is_vault() {
            if passcode.len() > *n {
                *n = passcode.len();
            }
            return;
        }
        let mut buffer = [0; 16];
        let (up, down, left, right) = {
            digest.input(&passcode);
            digest.result(&mut buffer);
            digest.reset();
            (buffer[0] >> 4,
             buffer[0] & 0xf,
             buffer[1] >> 4,
             buffer[1] & 0xf)
        };
        if is_open(up) && room.y > 0 {
            passcode.push(b'U');
            longest(room.up(), passcode, n);
            passcode.pop();
        }
        if is_open(down) && room.y < 3 {
            passcode.push(b'D');
            longest(room.down(), passcode, n);
            passcode.pop();
        }
        if is_open(left) && room.x > 0 {
            passcode.push(b'L');
            longest(room.left(), passcode, n);
            passcode.pop();
        }
        if is_open(right) && room.x < 3 {
            passcode.push(b'R');
            longest(room.right(), passcode, n);
            passcode.pop();
        }
    }

    fn shortest(passcode: Vec<u8>) {
        let mut q = Queue::new();
        q.push((Room::start(), passcode.clone()));
        let mut digest = Md5::new();
        while q.len() > 0 {
            let (room, mut passcode) = q.next();
            if room.is_vault() {
                for &c in passcode.iter() {
                    print!("{}", c as char);
                }
                print!("\n");
                return;
            }
            let mut buffer = [0; 16];
            let (up, down, left, right) = {
                digest.input(&passcode);
                digest.result(&mut buffer);
                digest.reset();
                (buffer[0] >> 4,
                 buffer[0] & 0xf,
                 buffer[1] >> 4,
                 buffer[1] & 0xf)
            };
            if is_open(up) && room.y > 0 {
                let mut passcode = passcode.clone();
                passcode.push(b'U');
                q.push((room.up(), passcode));
            }
            if is_open(down) && room.y < 3 {
                let mut passcode = passcode.clone();
                passcode.push(b'D');
                q.push((room.down(), passcode));
            }
            if is_open(left) && room.x > 0 {
                let mut passcode = passcode.clone();
                passcode.push(b'L');
                q.push((room.left(), passcode));
            }
            if is_open(right) && room.x < 3 {
                let mut passcode = passcode.clone();
                passcode.push(b'R');
                q.push((room.right(), passcode));
            }
        }
        println!("No more rooms.");
    }

    if false
    {
        let passcode = "ulqzkmiv".chars().map(|c| c as u8).collect::<Vec<_>>();
        shortest(passcode);

    }
    if true
    {
        let mut passcode = "rrrbmfta".chars().map(|c| c as u8).collect::<Vec<_>>();
        let mut n = 0;
        let prelen = passcode.len();
        longest(Room::start(), &mut passcode, &mut n);
        println!("max len = {}", n - prelen);
    }
}

pub fn task_18() {
    #[derive(PartialEq, Copy, Clone)]
    enum Tile {
        Trap,
        Safe,
    };

    impl Tile {
        fn from_char(c: char) -> Option<Tile> {
            Some(match c {
                '.' => Tile::Safe,
                '^' => Tile::Trap,
                _ => return None,
            })
        }

        fn from_traps(a: Tile, b: Tile, c: Tile) -> Self {
            let a = a == Tile::Trap;
            let b = b == Tile::Trap;
            let c = c == Tile::Trap;
            if a ^ c {
                Tile::Trap
            } else {
                Tile::Safe
            }
        }
    }

    use std::fmt;
    use std::fmt::{Display, Formatter, Write};
    impl Display for Tile {
        fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error>{
            match *self {
                Tile::Trap => fmt.write_char('^'),
                Tile::Safe => fmt.write_char('.'),
            };
            Ok(())
        }
    }

    fn next_row(row: &[Tile]) -> Vec<Tile> {
        let mut v = vec![];
        v.push({
            let fst = row[0];
            let snd = row[1];
            Tile::from_traps(Tile::Safe, fst, snd)
        });
        v.extend(row.windows(3).map(|c| Tile::from_traps(c[0], c[1], c[2])));
        v.push({
            let n = row.len();
            let fst = row[n - 2];
            let snd = row[n - 1];
            Tile::from_traps(fst, snd, Tile::Safe)
        });
        v
    }

    let input = "......^.^^.....^^^^^^^^^...^.^..^^.^^^..^.^..^.^^^.^^^^..^^.^.^.....^^^^^..^..^^^..^^.^.^..^^..^^^..";
    let rows = 400_000;

    let mut line = input.chars().flat_map(Tile::from_char).collect::<Vec<_>>();
    let mut num_safe = 0;
    for _ in 0..rows {
        num_safe += line.iter().filter(|&&t| t == Tile::Safe).count();
        let next = next_row(&line);
        line = next;
    }
    println!("Number of safe tiles: {}", num_safe);
}

pub fn task_19() {
    fn run(input: usize) -> usize {
        let pow = input.next_power_of_two() >> 1;
        let diff = input - pow;
        1 + 2 * diff
    }

    fn make_table(n: usize) -> (Vec<usize>, Vec<usize>) {
        let mut vec = vec![vec![2, 4], vec![2]];
        while *vec[0].last().unwrap() < n {
            // Add a new layer
            let bottom = vec.last().unwrap()[0];
            vec.push(vec![bottom << 1]);
            let height = vec.len();
            for i in 0..height - 1 {
                let y = height - 2 - i;
                let x = i + 1;
                let next = vec[y][x - 1] + vec[y + 1][x - 1];
                vec[y].push(next);
            }
        }
        let a = vec.remove(0);
        let mut b = vec.remove(0);
        b.iter_mut().map(|n| *n = *n + 1).count();
        (a, b)
    }

    fn run2(input: usize) -> usize {
        let input = input;
        let (a, b) = make_table(input);
        let an = a.iter().take_while(|&&n| n <= input).last().unwrap_or(&a[0]);
        let bn = b.iter().take_while(|&&n| n < input).last().unwrap_or(&b[0]);
        let count_ones = an > bn;
        if count_ones {
            input - an + 1
        } else {
            (input - bn + 1) * 2 + bn - an
        }
    }

    assert_eq!(run2(4), 1);
    assert_eq!(run2(5), 2);
    assert_eq!(run2(10), 1);
    assert_eq!(run2(28), 1);
    assert_eq!(run2(82), 1);

    let input = 3014387;
    println!("{}", run2(input));
}

use std;

pub fn task_20() {
    #[derive(PartialEq, Eq, Ord, PartialOrd, Debug, Clone, Copy)]
    struct Interval {
        from: i64,
        to: i64,
    }

    impl Interval {
        fn can_join(&self, other: Self) -> bool {
            self.from <= other.from &&
                self.to + 1 >= other.from
        }

        fn join(&self, other: Self) -> Self {
            use std::cmp::{max, min};
            let from = min(self.from, other.from);
            let to = max(self.to, other.to);
            Interval {
                from: from,
                to: to,
            }
        }

        fn size(&self) -> usize {
            (self.to - self.from + 1) as usize
        }

        fn null() -> Self {
            Interval {
                from: 0,
                to: 0,
            }
        }

        fn max() -> Self {
            Interval {
                from: 4294967296,
                to: 4294967296,
            }
        }

        fn min() -> Self {
            Interval {
                from: -1,
                to: -1,
            }
        }
    }


    impl std::str::FromStr for Interval {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut nums = s.split("-").flat_map(|n| n.parse());
            let a = nums.next().ok_or(())?;
            let b = nums.next().ok_or(())?;
            Ok(Interval {
                from: a,
                to: b,
            })
        }
    }
    let input = read_input("day-20").unwrap();
//     let input = "5-8
// 0-2
// 4-7";

    let mut intervals = input.lines()
        .flat_map(|l| l.parse::<Interval>())
        .collect::<Vec<_>>();
    intervals.insert(0, Interval::min());
    intervals.push(Interval::max());
    intervals.sort();
    let merged_intervals = {
        let (mut v, last) = intervals.iter()
            .skip(1)
            .fold((vec![], intervals[0]), |(mut vec, prev), &curr| {
                print!("{:?} {:?}", prev, curr);
                let next = if prev.can_join(curr) {
                    println!(" yes");
                    prev.join(curr)
                } else {
                    println!(" no");
                    vec.push(prev);
                    curr
                };
                (vec, next)
            });
        v.push(last);
        v
    };
    println!("{:?}", merged_intervals);
    let holes = merged_intervals.as_slice()
        .windows(2)
        .map(|s| {
            Interval {
                from: s[0].to + 1,
                to: s[1].from - 1,
            }
        });
    // if let Some(hole) = holes.next() {
    //     println!("First: {}", hole.from);

    // } else  {
    //     println!("No holes");
    // }
    let num_allowed = holes.map(|h| h.size()).sum::<usize>();
    println!("number of allowed IPs: {}", num_allowed);
}

pub fn task_21() {
    #[derive(Debug, Copy, Clone)]
    enum Action {
        SwapPos(usize, usize),
        SwapLetter(u8, u8),
        RotateLeft(usize),
        RotateRight(usize),
        RotateFromChar(u8),
        Reverse(usize, usize),
        Move(usize, usize),
    }

    impl Action {
        fn exec(self, s: &mut [u8]) -> &mut [u8]{
            match self {
                Action::SwapPos(a, b) => {
                    let tmp = s[a];
                    s[a] = s[b];
                    s[b] = tmp;
                }
                Action::SwapLetter(a, b) => {
                    for c in s.iter_mut() {
                        if *c == a {
                            *c = b;
                        } else if *c == b {
                            *c = a;
                        }
                    }
                }
                Action::RotateLeft(n) => {
                    let mut buffer = Vec::with_capacity(s.len());
                    for &c in s.iter().cycle().skip(n).take(s.len()) {
                        buffer.push(c);
                    }
                    for i in 0..buffer.len() {
                        s[i] = buffer[i];
                    }
                }
                Action::RotateRight(mut n) => {
                    let mut buffer = Vec::with_capacity(s.len());
                    if n > s.len() {
                        n -= s.len();
                    }
                    for &c in s.iter().cycle().skip(s.len() - n).take(s.len()) {
                        buffer.push(c);
                    }
                    for i in 0..buffer.len() {
                        s[i] = buffer[i];
                    }
                }
                Action::RotateFromChar(c) => {
                    let pos = s.iter().position(|&ch| ch == c)
                        .expect(&format!("Failed to find char {}!", c as char));
                    let rots = if pos >= 4 { pos + 2 } else { pos + 1 };
                    Action::RotateRight(rots).exec(s);
                }
                Action::Reverse(a, b) => {
                    let mut buffer = Vec::with_capacity(b - a + 1);
                    for i in a..(b + 1) {
                        buffer.push(s[i]);
                    }
                    for i in a..(b + 1) {
                        s[i] = buffer[buffer.len() - 1 - (i - a)];
                    }
                }
                Action::Move(a, b) => {
                    let mut buffer = Vec::with_capacity(s.len());
                    let c = s[a];
                    for i in 0..a {
                        buffer.push(s[i]);
                    }
                    for i in (a + 1)..s.len() {
                        buffer.push(s[i]);
                    }
                    for i in 0..b {
                        s[i] = buffer[i];
                    }
                    s[b] = c;
                    for i in (b + 1)..s.len() {
                        s[i] = buffer[i - 1];
                    }
                }
            }
            s
        }

        fn reverse(self, s: &[u8]) -> Self {
            match self {
                Action::RotateLeft(a) => Action::RotateRight(a),
                Action::RotateRight(a) => Action::RotateLeft(a),
                Action::RotateFromChar(c) => {
                    // Find the string such that self.exec() returns `s`
                    let mut slice = Vec::new();
                    slice.extend_from_slice(s);
                    for num_rotations in 0..s.len() {
                        let mut copy = slice.iter().cloned().collect::<Vec<_>>();
                        if self.exec(&mut copy) == s {
                            return Action::RotateLeft(num_rotations);
                        }
                        Action::RotateLeft(1).exec(&mut slice);
                    }
                    unreachable!();
                }
                Action::Move(a, b) => Action::Move(b, a),
                _ => self
            }
        }
    }

    use std::str::FromStr;
    impl FromStr for Action {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut split = s.split(" ");
            match split.next().unwrap() {
                "swap" => {
                    let t = split.next().unwrap();
                    let a = split.next().unwrap();
                    let b = split.skip(2).next().unwrap();
                    match t {
                        "position" => {
                            let a = a.parse::<usize>().unwrap();
                            let b = b.parse::<usize>().unwrap();
                            Ok(Action::SwapPos(a, b))
                        },
                        "letter" => {
                            let a = a.chars().next().unwrap();
                            let b = b.chars().next().unwrap();
                            Ok(Action::SwapLetter(a as u8, b as u8))

                        }
                        _ => Err(())
                    }
                },
                "rotate" => {
                    match split.next().unwrap() {
                        "left" => {
                            let n = split.next().unwrap().parse::<usize>().unwrap();
                            Ok(Action::RotateLeft(n))
                        },
                        "right" => {
                            let n = split.next().unwrap().parse::<usize>().unwrap();
                            Ok(Action::RotateRight(n))
                        },
                        "based" => {
                            let c = split.last().unwrap().chars().next().unwrap();
                            Ok(Action::RotateFromChar(c as u8))
                        },
                        _ => Err(())
                    }
                },
                "reverse" => {
                    let _ = split.next();
                    let a = split.next().unwrap().parse::<usize>().unwrap();
                    let b = split.skip(1).next().unwrap().parse::<usize>().unwrap();
                    Ok(Action::Reverse(a, b))
                },
                "move" => {
                    let _ = split.next();
                    let a = split.next().unwrap().parse::<usize>().unwrap();
                    let b = split.skip(2).next().unwrap().parse::<usize>().unwrap();
                    Ok(Action::Move(a, b))
                },
                _ => Err(())
            }
        }
    }
    use std::str::from_utf8;


    let input = read_input("day-21").unwrap();

    let actions = input.lines().flat_map(Action::from_str).collect::<Vec<_>>();
    {
        let mut start_input = b"abcdefgh".iter().cloned().collect::<Vec<_>>();
        let mut result = actions.iter()
            .fold(start_input.as_mut_slice(), |s, a| {
                // println!("{:?}", a);
                // println!("\tfrom {}", from_utf8(s).unwrap());
                a.exec(s);
                // println!("\tto   {}\n", from_utf8(s).unwrap());
                s
            });
        println!("{}", from_utf8(result).unwrap());
    }
    {
        let mut unscramble = b"fbgdceah".iter().cloned().collect::<Vec<_>>();
        let result = actions.iter()
            .rev()
            .fold(unscramble.as_mut_slice(), |s, a| {
                // println!("{:?}", a);
                // println!("\tfrom {}", from_utf8(s).unwrap());
                a.reverse(s).exec(s);
                // println!("\tto   {}\n", from_utf8(s).unwrap());
                s
            });
        println!("{}", from_utf8(result).unwrap());
    }
}

pub fn task_22() {
    #[derive(Debug)]
    struct Node {
        x: usize,
        y: usize,
        size: usize,
        used: usize,
        avail: usize,
        use_percent: usize,
    }
    let re = Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%").unwrap();
    let input = read_input("day-22").unwrap();


//     let input = "Filesystem            Size  Used  Avail  Use%
// /dev/grid/node-x0-y0   10T    8T     2T   80%
// /dev/grid/node-x0-y1   11T    6T     5T   54%
// /dev/grid/node-x0-y2   32T   28T     4T   87%
// /dev/grid/node-x1-y0    9T    7T     2T   77%
// /dev/grid/node-x1-y1    8T    0T     8T    0%
// /dev/grid/node-x1-y2   11T    7T     4T   63%
// /dev/grid/node-x2-y0   10T    6T     4T   60%
// /dev/grid/node-x2-y1    9T    8T     1T   88%
// /dev/grid/node-x2-y2    9T    6T     3T   66%";

    let mut nodes = vec![];
    for line in input.lines() {
        if let Some(caps) = re.captures(line) {
            let mut caps = caps.iter().skip(1).map(|d| d.unwrap().parse::<usize>().unwrap());
            nodes.push(Node {
                x: caps.next().unwrap(),
                y: caps.next().unwrap(),
                size: caps.next().unwrap(),
                used: caps.next().unwrap(),
                avail: caps.next().unwrap(),
                use_percent: caps.next().unwrap(),
            });
        }
    }

    let mut num_pairs = 0;
    let n = nodes.len();
    for i in 0..n {
        if nodes[i].used == 0 {
            continue;
        }
        for j in 0..n {
            if i == j {
                continue;
            }
            if nodes[i].used <= nodes[j].avail {
                num_pairs += 1;
            }
        }
    }
    // println!("{:?}", num_pairs);
    use std::iter::repeat;

    #[derive(Clone, Copy, Hash, PartialEq, Eq)]
    enum Tile {
        Goal,
        Empty,
        Movable,
        Full,
    }

    let lowest_avail = nodes.iter().map(|n| n.avail).min().unwrap();
    let min_data = nodes.iter().map(|n| n.used).filter(|&n| n != 0).min().unwrap();
    let min_cap = nodes.iter().map(|n| n.size).min().unwrap();
    let max_x = nodes.iter().map(|n| n.x).max().unwrap() + 1;
    let max_y = nodes.iter().map(|n| n.y).max().unwrap() + 1;
    let mut board = repeat(repeat(Tile::Empty).take(max_x).collect::<Vec<_>>()).take(max_y).collect::<Vec<_>>();

    for node in &nodes {
        let tile = if node.x == max_x - 1 && node.y == 0 {
            Tile::Goal
        } else if node.used == 0 {
            Tile::Empty
        } else if node.used <= min_cap && node.avail < min_data {
            Tile::Movable
        } else {
            Tile::Full
        };
        board[node.y][node.x] = tile;
    }

    impl Tile {
        fn char(self) -> char {
            match self {
                Tile::Goal => 'G',
                Tile::Empty => '_',
                Tile::Movable => '.',
                Tile::Full => '#',
            }
        }

        fn is_goal(self) -> bool {
            match self {
                Tile::Goal => true,
                _ => false,
            }
        }

        fn is_empty(self) -> bool {
            match self {
                Tile::Empty => true,
                _ => false,
            }
        }
    }
    type Board = Vec<Vec<Tile>>;

    // TODO:
    // let board: Board = vec![
    //     vec![Tile::Movable, Tile::Movable, Tile::Goal],
    //     vec![Tile::Movable, Tile::Empty, Tile::Movable],
    //     vec![Tile::Full, Tile::Movable, Tile::Movable],
    // ];

    struct Queue<T> {
        v: Vec<T>,
        i: usize
    }

    impl<T> Queue<T>
    where T: Clone {
        fn new() -> Self {
            Queue {
                v: Vec::new(),
                i: 0
            }
        }

        fn next(&mut self) -> T {
            self.v.remove(0)
        }

        fn push(&mut self, e: T) {
            self.v.push(e);
        }

        fn len(&self) -> usize {
            self.v.len() - self.i
        }
    }

    let mut queue = Queue::new();
    let mut seen_boards = HashSet::new();
    queue.push((board, 0));

    while queue.len() > 0 {
        let (board, steps) = queue.next();
        // println!("Looking at ");
        // for line in board.iter() {
        //     for t in line.iter() {
        //         print!(" {} ", t.char());
        //     }
        //     print!("\n");
        // }
        // println!("\n\n");
        if board[0][0].is_goal() {
            println!("steps: {}", steps);
            break;
        }
        for y in 0..board.len() {
            for x in 0..board[y].len() {
                if board[y][x].is_empty() {
                    for &(i, j) in &[(x, y.saturating_sub(1)), (x.saturating_sub(1), y), (x + 1, y), (x, y + 1)] {
                        if i >= board[y].len() || j >= board.len() {
                            continue;
                        }
                        match board[j][i] {
                            Tile::Empty | Tile::Movable | Tile::Goal => {
                                let mut c = board.clone();
                                let tmp = c[j][i];
                                c[j][i] = c[y][x];
                                c[y][x] = tmp;
                                if !seen_boards.contains(&c) {
                                    queue.push((c.clone(), steps + 1));
                                    seen_boards.insert(c);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

pub fn task_23() {
    #[derive(Clone, Debug)]
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
    #[derive(Clone, Debug)]
    enum Instruction<'a> {
        Cpy(Op<'a>, Op<'a>),
        Inc(Op<'a>),
        Dec(Op<'a>),
        Jnz(Op<'a>, Op<'a>),
        Tgl(Op<'a>),
    };

    impl<'a> From<&'a str> for Instruction<'a> {
        fn from(s: &'a str) -> Self {
            let mut spl = s.split(" ");
            let op = spl.next().unwrap();
            let a = Op::make(spl.next().unwrap());
            match op {
                "cpy" => {
                    let b = Op::make(spl.next().unwrap());
                    Instruction::Cpy(a, b)
                }
                "inc" => Instruction::Inc(a),
                "dec" => Instruction::Dec(a),
                "jnz" => {
                    let b = Op::make(spl.next().unwrap());
                    Instruction::Jnz(a, b)
                }
                "tgl" => {
                    Instruction::Tgl(a)
                }
                _ => unreachable!()

            }
        }
    }

    let input = "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";
    let input = read_input("day-23").unwrap();

    let mut registers = HashMap::new();
    let mut instructions = input.lines()
        .map(Instruction::from)
        .collect::<Vec<_>>();
    let mut pc = 0i32;
    let n_instr = instructions.len() as i32;
    registers.insert("a", 12);

    while pc < n_instr {
        let instr = instructions[pc as usize].clone();
        match instr {
            Instruction::Inc(reg) => {
                if let Op::Reg(reg) = reg {
                    *registers.entry(reg).or_insert(0) += 1;
                }
            },
            Instruction::Dec(reg) => {
                if let Op::Reg(reg) = reg {
                    *registers.entry(reg).or_insert(0) -= 1;
                }
            },
            Instruction::Cpy(from, to) => {
                if let Op::Reg(to) = to {
                    let v = from.value(&registers);
                    registers.insert(to, v);
                }
            },
            Instruction::Jnz(ref c, ref n) => {
                let should_jump = c.value(&registers);
                if should_jump != 0 {
                    let val = n.value(&registers);
                    // Only jump if valid
                    let target = pc + val - 1;
                    if target < instructions.len() as i32 && target >= 0 {
                        pc = target;
                    }
                }
            },
            Instruction::Tgl(ref c) => {
                let index = (pc + c.value(&registers)) as usize;
                if index < instructions.len() {
                    instructions[index] = match instructions[index].clone() {
                        Instruction::Inc(reg) => Instruction::Dec(reg),
                        Instruction::Dec(reg) => Instruction::Inc(reg),
                        Instruction::Tgl(op) => Instruction::Inc(op),
                        Instruction::Jnz(a, b) => Instruction::Cpy(a, b),
                        Instruction::Cpy(a, b) => Instruction::Jnz(a, b),
                    };
                }
            }
        }
        pc += 1;
    }
    println!("a={}", registers.get("a").unwrap());
}

use std::str::FromStr;
pub fn task_24() {
    #[derive(Clone)]
    enum Tile {
        Wall,
        Space,
        Point(usize),
    }

    impl Tile {
        fn is_space(&self) -> bool {
            match *self {
                Tile::Space => true,
                _ => false,
            }
        }
        fn is_wall(&self) -> bool {
            match *self {
                Tile::Wall => true,
                _ => false,
            }
        }
    }

    fn print(board: &Vec<Vec<Tile>>) {
        for line in board.iter() {
            for tile in line.iter() {
                print!("{}", match tile {
                    &Tile::Wall => '┼',
                    &Tile::Space => ' ',
                    &Tile::Point(n) => (n as u8 + b'0') as char,
                });
            }
            print!("\n");
        }
    }

    impl Tile {
        fn from_char(c: char) -> Result<Self, ()> {
            Ok(match c {
                '.' => Tile::Space,
                '#' => Tile::Wall,
                '0'...'9' => Tile::Point((c as u8 - b'0') as usize),
                _ => return Err(()),
            })

        }
    }

    let input = read_input("day-24").unwrap();
//     let input = "###########
// #0.1.....2#
// #.#######.#
// #4.......3#
// ###########";

    let mut board = input
        .lines()
        .map(|line| line.chars().flat_map(Tile::from_char).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut changed = true;
    let height = board.len();
    let width = board[0].len();
    // fill dead ends in the board
    while changed {
        changed = false;
        print(&board);
        println!("\n");
        for j in 1..height - 1 {
            for i in 1..width - 1 {
                if board[j][i].is_space() {
                    // If three neighbours are walls, this can be a wall as well.
                    let num_walls = [(i, j - 1), (i - 1, j),
                                     (i + 1, j), (i, j + 1)]
                        .iter()
                        .filter(|&&(x, y)| board[y][x].is_wall())
                        .count();
                    if num_walls >= 3 {
                        changed = true;
                        board[j][i] = Tile::Wall;
                    }
                }
            }
        }

    }
    struct Queue<T> {
        v: Vec<T>,
        i: usize
    }

    impl<T> Queue<T>
    where T: Clone {
        fn new() -> Self {
            Queue {
                v: Vec::new(),
                i: 0
            }
        }

        fn next(&mut self) -> T {
            self.v.remove(0)
        }

        fn push(&mut self, e: T) {
            self.v.push(e);
        }

        fn len(&self) -> usize {
            self.v.len() - self.i
        }
    }

    let points = {
        let mut v = Vec::new();
        for j in 1..board.len() - 1 {
            for i in 1..board[0].len() - 1 {
                if let Tile::Point(_) = board[j][i] {
                    v.push((i, j));
                }
            }
        }
        v
    };

    #[derive(Clone, Copy)]
    struct QueueE {
        x: usize,
        y: usize,
        steps: usize,
    }

    impl QueueE {
        fn first(p: (usize, usize)) -> Self {
            QueueE {
                x: p.0,
                y: p.1,
                steps: 0,
            }
        }
        fn neigh(&self) -> [(usize, usize); 4] {
            [
                (self.x, self.y - 1),
                (self.x - 1, self.y),
                (self.x + 1, self.y),
                (self.x, self.y + 1),
            ]
        }
    }


    use std::iter::repeat;
    let mut paths = repeat(repeat(usize::max_value())
                           .take(points.len())
                           .collect::<Vec<_>>())
                           .take(points.len())
                           .collect::<Vec<Vec<usize>>>();
    for i in 0..paths.len() {
        let from = points.iter().filter(|&&(x, y)| {
            if let Tile::Point(n) = board[y][x] {
                i == n
            } else {
                false
            }
        }).next().cloned().unwrap();
        // Run BFS from `from` to `to`.
        let mut queue = Queue {v: Vec::with_capacity(height * width), i: 0 };
        let mut board = board.clone();
        queue.push(QueueE::first(from));
        while queue.len() > 0 {
            let head = queue.next();
            for &(x, y) in head.neigh().iter() {
                if let Tile::Point(n) = board[y][x] {
                    paths[i][n] = head.steps + 1;
                }
                if !board[y][x].is_wall() {
                    queue.push(QueueE {
                        x: x,
                        y: y,
                        steps: head.steps + 1,
                    });
                    board[y][x] = Tile::Wall;
                }

            }
        }
    }

    use permutohedron::Heap;
    use std::iter::once;
    let mut inds = (1..paths.len()).collect::<Vec<_>>();
    let best = Heap::new(&mut inds)
        .map(|perm| {
            once(0)
                .chain(perm.iter().cloned())
                .zip(perm.iter().cloned().chain(once(0)))
                .map(|(from, to)| paths[from][to]).sum::<usize>()
        })
        .min()
        .unwrap_or(0);
    println!("min: {}", best);

    for from in 0..paths.len() {
        // print header
        if from == 0 {
            print!("   |");
            for i in 0..paths.len() {
                print!("{:3} ", i);
            }
            print!("\n");
            print!("----");
            for i in 0..paths.len() {
                print!("----");
            }
            print!("\n");

        }
        print!("{:3}|", from);
        for to in 0..paths.len() {
            print!("{:3} ", paths[from][to]);
        }
        print!("\n");
    }
}

use std::hash::{Hash, Hasher, SipHasher};
pub fn task_25() {
    #[derive(Clone, Debug, Hash)]
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
    #[derive(Clone, Debug, Hash)]
    enum Instruction<'a> {
        Cpy(Op<'a>, Op<'a>),
        Inc(Op<'a>),
        Dec(Op<'a>),
        Jnz(Op<'a>, Op<'a>),
        Tgl(Op<'a>),
        Out(Op<'a>),
    };

    impl<'a> From<&'a str> for Instruction<'a> {
        fn from(s: &'a str) -> Self {
            let mut spl = s.split(" ");
            let op = spl.next().unwrap();
            let a = Op::make(spl.next().unwrap());
            match op {
                "cpy" => {
                    let b = Op::make(spl.next().unwrap());
                    Instruction::Cpy(a, b)
                }
                "inc" => Instruction::Inc(a),
                "dec" => Instruction::Dec(a),
                "jnz" => {
                    let b = Op::make(spl.next().unwrap());
                    Instruction::Jnz(a, b)
                }
                "tgl" => {
                    Instruction::Tgl(a)
                }
                "out" => {
                    Instruction::Out(a)
                }
                _ => unreachable!()

            }
        }
    }

    let input = "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";
    let input = read_input("day-25").unwrap();

    'outer: for a in 1.. {
        let mut registers = HashMap::new();
        let mut instructions = input.lines()
            .map(Instruction::from)
            .collect::<Vec<_>>();
        let mut pc = 0i32;
        let n_instr = instructions.len() as i32;
        let mut expected_out = 0;
        let mut seen_states = HashSet::new();
        registers.insert("a", a);

        while pc < n_instr {
            let mut s = SipHasher::new();
            pc.hash(&mut s);
            registers.iter().map(|e| e.hash(&mut s)).count();
            instructions.iter().map(|e| e.hash(&mut s)).count();
            let hash = s.finish();
            if seen_states.contains(&hash) {
                println!("a={} repeats", a);
                println!("a={}", registers.get("a").unwrap());
                break 'outer;
            }
            seen_states.insert(hash);

            let instr = instructions[pc as usize].clone();
            match instr {
                Instruction::Inc(reg) => {
                    if let Op::Reg(reg) = reg {
                        *registers.entry(reg).or_insert(0) += 1;
                    }
                },
                Instruction::Dec(reg) => {
                    if let Op::Reg(reg) = reg {
                        *registers.entry(reg).or_insert(0) -= 1;
                    }
                },
                Instruction::Cpy(from, to) => {
                    if let Op::Reg(to) = to {
                        let v = from.value(&registers);
                        registers.insert(to, v);
                    }
                },
                Instruction::Jnz(ref c, ref n) => {
                    let should_jump = c.value(&registers);
                    if should_jump != 0 {
                        let val = n.value(&registers);
                        // Only jump if valid
                        let target = pc + val - 1;
                        if target < instructions.len() as i32 && target >= 0 {
                            pc = target;
                        }
                    }
                },
                Instruction::Tgl(ref c) => {
                    let index = (pc + c.value(&registers)) as usize;
                    if index < instructions.len() {
                        instructions[index] = match instructions[index].clone() {
                            Instruction::Inc(reg) => Instruction::Dec(reg),
                            Instruction::Dec(reg) => Instruction::Inc(reg),
                            Instruction::Tgl(op) => Instruction::Inc(op),
                            Instruction::Out(a) => Instruction::Inc(a),
                            Instruction::Jnz(a, b) => Instruction::Cpy(a, b),
                            Instruction::Cpy(a, b) => Instruction::Jnz(a, b),
                        };
                    }
                }
                Instruction::Out(ref c) => {
                    let v = c.value(&registers);
                    if v != expected_out {
                        // if a % 10_000 == 0 {
                            println!("a={} failed to output correct sequence", a);
                        // }
                        expected_out = 0;
                        continue 'outer;
                    }
                    expected_out = match expected_out {
                        0 => 1,
                        _ => 0,
                    };
                }
            }
            pc += 1;
        }
        println!("a={} terminated", a);
    }

}
