use std::fs::File;
use std::io::Read;

static FILE_NAME: &'static str = "./input_day_1";

pub fn day_1() {
    let mut file = File::open(FILE_NAME).unwrap();
    let mut input = String::new();

    let res = file.read_to_string(&mut input);
    if let Err(e) = res {
        println!("Error: {:?}", e);
    }

    let floor = input.chars()
        .fold(0, |acc, item|
              if item == '(' {
                  acc+1
              } else if item == ')' {
                  acc - 1
              } else {acc});

    let mut flr = 0;
    let mut i = 0;
    for c in input.chars() {
        i += 1;
        match c {
            '(' => flr += 1,
            ')' => flr -= 1,
            _ => ()
        };
        if flr == -1 {
            break
        }
        
    }

    println!("end floor: {}", floor);
    println!("basement: {}", i);
}
