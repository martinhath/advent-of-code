use std::collections::HashSet;

pub fn task_1() {
    const INPUT: &'static str = "L3, R2, L5, R1, L1, L2, L2, R1, R5, R1, L1, L2, R2, R4, L4, L3, L3, R5, L1, R3, L5, L2, R4, L5, R4, R2, L2, L1, R1, L3, L3, R2, R1, L4, L1, L1, R4, R5, R1, L2, L1, R188, R4, L3, R54, L4, R4, R74, R2, L4, R185, R1, R3, R5, L2, L3, R1, L1, L3, R3, R2, L3, L4, R1, L3, L5, L2, R2, L1, R2, R1, L4, R5, R4, L5, L5, L4, R5, R4, L5, L3, R4, R1, L5, L4, L3, R5, L5, L2, L4, R4, R4, R2, L1, L3, L2, R5, R4, L5, R1, R2, R5, L2, R4, R5, L2, L3, R3, L4, R3, L2, R1, R4, L5, R1, L5, L3, R4, L2, L2, L5, L5, R5, R2, L5, R1, L3, L2, L2, R3, L3, L4, R2, R3, L1, R2, L5, L3, R4, L4, R4, R3, L3, R1, L3, R5, L5, R1, R5, R3, L1";
    #[derive(Clone)]
    enum Direction {
        Up,
        Down,
        Left,
        Right
    };
    impl Direction {
        fn turn(&self, other: char) -> Direction {
            match *self {
                Direction::Up => if other == 'R' { Direction::Right } else { Direction::Left },
                Direction::Down => if other == 'R' { Direction::Left } else { Direction::Right },
                Direction::Left => if other == 'R' { Direction::Up } else { Direction::Down },
                Direction::Right => if other == 'R' { Direction::Down } else { Direction::Up }
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
                Direction::Up => { y += 1},
                Direction::Down => { y -= 1},
                Direction::Left => { x -= 1},
                Direction::Right => { x += 1},
            }
        }
    }
    println!("{:?} {:?}", x, y);
    println!("{:?}", x.abs() + y.abs());
}

pub fn task_2() {
    let input = "LLULLLRLDLLLRLUURDDLRDLDURULRLUULUDDUDDLLLURRLDRRLDRRRLDUDLRDLRRDLLDUDUDUDRLUDUUDLLLRDURUDUULUDLRDUUUDUUDURLDUULLRDLULDUURUDRDDLDRLURLRURRDUURLRLUURURUUULLRLLULRUURLULURDLLRRUDLUDULDRDRLRULUURRDRULLRUUUDLRLDLUURRRURDLUDDRRUDRLUDRDLLLLLRULLDUDRLRRDDULDLRUURRRRRLDLDLRDURDRUUURDLRDDDDULURRRRDUURLULLLDLRULRDULRUDLRRLRDLLRLLLUDDLRDRURDDLLLLDUDRDLRURRDLRDDDLDULDRLRULUUDRRRUUULLLURRDDUULURULDURRLLULLDRURUUULRLRDRRUDRDRRDURRUUUULDRDDDUDLDDURLLRR
LDLRRRUURDLDDRLRRDLLULRULLLUDUUDUDLRULLDRUDRULLDULURDRDDLRURDDULLLLDLRDRDRDDURLURLURLUDRDDRDULULUDDRURRDLLDUURDRDDLRLLURRDLRDDULDLULURDRDLUDRRUUDULLULURRDUDRUUUDRULDLDURLRRUDURLDLRRUURRRURDLUDRLDUDRRUDUURURUDDUUDRDULRDLUDRRRLDRURLLRDDDLUDRDUDURDDDRRDDRRRLLRRDDLDDLRUURRURDLLDRLRRDLLUDRRRURURLRDRLLRLRLRULLRURLDLRRULLRRRDULUUULDRDLLURDDLDLRDRLUUDLLUDDLDRRLDLRUDRUDLLUURLLULURUDUDRLULLUDRURDDLDLDDUDLRDDRRURLRLLUDDUDRUURRURRULDRLDDRLLRRLDDURRDLDULLLURULLLRUURLRRRRUUULRLLLURRLRLRUDRDUUUDUUUDDLULLDLLLLDLDRULDRUUULDDDLURLDLRLULRUDDDDURDDLU
RURLURRDLDULLULDDDLRUULLUURLRUDRUDRRUDDLDDDDRRDLRURLRURLDDDUDDUURRDRULDRRRULRDRDDLRUDULRLURDUUDRRLDLRDRURDLDRRRRDRURUUDDDLLRDRDUDUDUDLLULURULRRLRURUULUULDDDDURULRULLRUDUURLURDUDLUDLUDRLLDUUDUULRLRLUUDRDULDULRURDRRRULRUDLRURDDULUDULLRLRURURUULLULDRURLLRRUUDDUUURRDLURUURULRDRRDDUDULRDDLUDLURURUURDRULLRDDLLRDDLDRDUDRRDLUURRLRLUURRULUDURLDDRLLURRDDDLDDRURULLDDRLUDDLRLURDUDULLRDULLLDLLUDDRUDRUDDUUDRDRULRL
RLRDRDULULUDLUDRDRLUDLDLLUDURULDDDUDLRURLLRLRLDLDRLDURDLRRURLULLULURLLDRRDRLUDRLRDLLULRULURRURURUULRDUDLLRDLRRRRRLUURDRRRDLRUDLLDLLDLRUUUDLLLDDDLRDULLRUUDDRLDDURRRDLRLRLDDDDLRDRULLUURUUDRRLLRLLRDDLLRURRRRDRULRRLLRLLLRLDRRLDDDURRURLDURUURRLRLRLDRURULLRLRUDLDUURDLLRLDLURUUUDLLRDRDDDDDDRLDRRRLRRRRURUDLDDRDLLURUDLRRLDDDLUDUDUULRDULULUDDULUUDLLLLRLDDUUULRLRDULURDURRRURRULURRRDRDLDDURDLURUDURRRDDRLRLUDLUDDLUULLDURLURDDUDDLRUUUDRLLDRURL
ULUDLLUDDULRUURDRURDUDUDLUURDDDRRLUDURURDRURRLDRDURLRLLRRDDRRDRRRUULURUDURUDULRRRRDDLDURRLRRDUDDDRLLLULDRLRLURRDUURDURRRURRDLUDUDDRLDLURRRDDRLLRDRDDRDURRRRLURRLUDDURRULRUDUDULDRUDDRULLUUULDURRRLDRULLURULLRUDLDUDDLDULDLUUDRULULDLLDRULLRUULDUDUUDRLRRLDLUULUDLLDDRLRRDDLLURURDULRRDDRURDRLRLULDLDURULLUUUDURURDLDUDDDDUUULUDLUURRULLDLRLURDLURLRLDDURRLDDRRRDUUULLUULDLLDLLDDRLRRUDLULDRLULDULULRRLRULUUURURUUURDUUDDURLLUDDRLRDDLUURRUULRDLDDRLULUULRDRURLUURDRDUURUDLRR";
//     let input = "ULL
// RRDDD
// LURDL
// UUUUD";
    if false { // part 1
        let mut x = 1usize;
        let mut y = 1usize;
        for line in input.lines() {
            for c in line.chars() {
                use std::cmp::min;
                match c {
                    'U' => { y = y.saturating_sub(1); }
                    'L' => { x = x.saturating_sub(1); }
                    'R' => { x = min(x + 1, 2); }
                    'D' => { y = min(y + 1, 2); }
                    _ => unreachable!(),
                }
            }
            print!("{}", x + y * 3 + 1);
        }
    }
    if true { // part 2
        let mut x = 0usize;
        let mut y = 2usize;
        for line in input.lines() {
            fn is_outside(x: usize, y: usize) -> bool {
                match y {
                    0 => { x != 2 }
                    1 => { x == 0 || x == 4 }
                    3 => { x == 0 || x == 4 }
                    4 => { x != 2 }
                    _ => false
                }
            }
            fn coord(x: usize, y: usize) -> char {
                // assume is_inside
                match y {
                    0 => '1',
                    1 => match x {
                        1 => '2',
                        2 => '3',
                        3 => '4',
                        _ => unreachable!()
                    },
                    2 => match x {
                        0 => '5',
                        1 => '6',
                        2 => '7',
                        3 => '8',
                        4 => '9',
                        _ => unreachable!()
                    },
                    3 => match x {
                        1 => 'A',
                        2 => 'B',
                        3 => 'C',
                        _ => unreachable!()
                    },
                    _ => 'D',
                }
            };
            for c in line.chars() {
                use std::cmp::min;
                let mut x1 = x;
                let mut y1 = y;
                match c {
                    'U' => { y1 = y.saturating_sub(1); }
                    'L' => { x1 = x.saturating_sub(1); }
                    'R' => { x1 = min(x + 1, 4); }
                    'D' => { y1 = min(y + 1, 4); }
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
