use std::cmp::{max, min};

const HEIGHT: usize = 100;
const WIDTH:  usize = 100;

type Board = [[bool; WIDTH]; HEIGHT];

fn neighbors(x: i32, y: i32) -> Vec<(i32, i32)> {
    let mut v = Vec::new();
    for j in max(0, y - 1)..min(HEIGHT as i32, y + 2) {
        for i in max(0, x - 1)..min(WIDTH as i32, x + 2) {
            if j == y && i == x {
                continue;
            }
            v.push((i, j));
        }
    }
    v
}

fn step(board: &Board) -> Board {
    let mut new_board: Board = [[false; WIDTH]; HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {

            let lit = neighbors(x as i32, y as i32).into_iter()
                .filter(|&(i, j)| board[j as usize][i as usize]).count();

            if board[y][x] {
                if lit == 2 || lit == 3 {
                    new_board[y][x] = true;
                }
            } else {
                if lit == 3 {
                    new_board[y][x] = true;
                }
            }
        }
    }
    // corner lights are allways on
    new_board[0][0] = true;
    new_board[0][WIDTH - 1] = true;
    new_board[HEIGHT - 1][0] = true;
    new_board[HEIGHT - 1][WIDTH - 1] = true;
    new_board
}

fn print_board(board: &Board) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let c = if board[y][x] {'#'} else {'.'};
            print!("{}", c);
        }
        print!("\n");
    }
}


pub fn day_18(input: String) {
    // Game of life
    let mut board: Board = [[false; WIDTH]; HEIGHT];

    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        for c in line.chars() {
            if c == '#' {
                board[y][x] = true;
            }
            x += 1;
        }
        y += 1;
    }

    for _ in 0..100 {
        board = step(&board);
    }

    let mut sol: usize = 0;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if board[y][x] {
                sol += 1;
            }
        }
    }


    println!("solution: {}", sol);
}
