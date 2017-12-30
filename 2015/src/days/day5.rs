fn is_vowel(c: char) -> bool {
    "aeiou".contains(c)
}

fn is_illegal(c1: char, c2: char) -> bool {
    (c1 == 'a' && c2 == 'b') ||
    (c1 == 'c' && c2 == 'd') ||
    (c1 == 'p' && c2 == 'q') ||
    (c1 == 'x' && c2 == 'y')
}

fn solve_5_1(input: String) {
    let mut valids = 0;
    for line in input.lines() {
        let pairs = line.chars().zip(line.chars().skip(1));
        if pairs.clone().filter(|&(a, b)| is_illegal(a, b)).count() != 0 {
            continue;
        }

        let vowel_count = line.chars().filter(|&c| is_vowel(c)).count();
        let doubles = pairs.filter(|&(a, b)| a == b).count();

        if vowel_count > 2 && doubles > 0 {
            valids += 1;
        }
    }
    println!("valids: {}", valids);
}

fn requirement_1(list: Vec<(char, char)>) -> bool {
    if list.len() < 2 { return false; }
    let (head, tail) = list.split_at(2);

    let elem = head[0];
    if tail.contains(&elem) {
        return true;
    }
    let next_list: Vec<_> = list.iter().skip(1).map(|e| e.clone()).collect();
    requirement_1(next_list)
}

pub fn day_5(input: String) {

    let mut valid = 0;
    for line in input.lines() {
        let triples = line.chars()
            .zip(line.chars().skip(1))
            .zip(line.chars().skip(2));


        let doubles = line.chars().zip(line.chars().skip(1)).collect();
        let req1 = requirement_1(doubles);

        let req2 = triples.filter(|&((a, _), c)| a == c).count() != 0;

        println!("{} -> 1: {} 2: {}", line, req1, req2);
        if req1 && req2 {
            valid += 1;
        }
    }

    println!("valid: {}", valid);
}

