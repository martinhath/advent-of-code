pub fn day_1(input: String) {
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
