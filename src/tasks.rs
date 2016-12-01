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
        for n in 0..n {
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
