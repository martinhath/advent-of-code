fn count_line1(line: &str) -> (usize, usize) {
    // part one
    let mut iter = line.chars();
    let mut len2 = 0;
    loop {
        let chr = iter.next();
        match chr {
            None => {break;}
            Some(c) => {
                if c == '\\' {
                    let x = iter.next().unwrap();
                    if x == 'x' {
                        iter.next();
                        iter.next();
                    }
                }
                len2 += 1;
            }
        }
    }

    len2 -= 2;
    let len = line.len();
    (len, len2)
}

fn count_line2(line: &str) -> (usize, usize) {
    let slashes = line.chars().filter(|&c| c == '\\').count();
    let ticks = line.chars().filter(|&c| c == '\"').count();
    let len = line.len();
    (len + slashes + ticks + 2, len)
}

pub fn day_8(input: String) {
    let mut code_len = 0;
    let mut mem_len = 0;
    for line in input.lines() {
        let (a, b) = count_line2(line);
        code_len += a;
        mem_len += b;

    }
    println!("Solution: {}", code_len - mem_len);
}
