fn next_char(c: u8) -> u8 {
    let cval = c;
    match cval + 1 {
        105 => 106, // skip i
        111 => 112, // skip o
        108 => 109, // skip l
        123 => 97,  // wrap z to a
        v   => v
    }
}

fn next_password(input: Vec<u8>) -> Vec<u8> {
    let mut inp = input;
    let mut i = inp.len() - 1;
    let mut next = true;

    while next {
        next = false;
        let c = inp[i];
        if c == 122 {      // z
            inp[i] = 97; // a
            i -= 1;
            next = true;
        } else {
            inp[i] = next_char(inp[i]);
        }
    }
    inp
}

// look for invalid chars
// fn quick_fix() { }

fn is_invalid(bytes: &Vec<u8>) -> bool {
    // println!("{:?}", bytes);
    // straight 
    let mut has_straight = false;
    for i in (0..bytes.len() - 2) {
        let a = bytes[i];
        let b = bytes[i + 1];
        let c = bytes[i + 2];
        if a + 1 == b && b + 1 == c {
            has_straight = true;
            break;
        }
    }
    if !has_straight {return true;}
    // two pairs
    // faster to simply identify all pairs?
    for i in (0..bytes.len() - 1) {
        let a = bytes[i];
        let b = bytes[i + 1];
        if a == b {
            for j in (i + 2..bytes.len() - 1) {
                let x = bytes[j];
                let y = bytes[j + 1];
                if x == y {
                    return false;
                }
            }
        }
    }
    true
}

fn is_illegal(c: &u8) -> bool {
    [105, 111, 108].contains(c)
}

pub fn day_11(input: String) {

    for line in input.lines() {
        let mut bytes: Vec<u8> = line.chars().map(|c| c as u8).collect();

        println!("Solving {}", line);
        if let Some(i) = bytes.iter().position(is_illegal) {

            let (_, mut tail) = bytes.split_at_mut(i);
            let first = tail[0];
            for e in tail.iter_mut() {
                *e = 97;
            }

            tail[0] = next_char(first);
        }
        println!("{:?}", String::from_utf8(bytes.clone()).unwrap());

        while is_invalid(&bytes) {
            bytes = next_password(bytes);
        }

        let next = String::from_utf8(bytes).unwrap();
        println!("{:<10} -> {:?}", line, next);
    }
}
