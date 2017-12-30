
fn next(num: Vec<usize>) -> Vec<usize> {

    let mut vec: Vec<usize> = Vec::new();
    let mut iter = num.iter();
    let mut curr = iter.next().expect("empty string to 'next'");
    let mut count = 1;

    loop {
        let next = iter.next();
        if next.is_none() { break; }
        let next = next.unwrap();
        if next != curr {
            vec.push(count);
            vec.push(*curr);
            curr = next;
            count = 1;
        } else {
            count += 1;
        }
    }
    vec.push(count);
    vec.push(*curr);

    vec
}

pub fn day_10(input: String) {
    let mut vec: Vec<_> = input.chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).expect("obs") as usize).collect();
    for i in 0..50 {
        println!("#{} = {}", i, vec.len());
        vec = next(vec);
    }
    println!("solution: {}", vec.len());
}
