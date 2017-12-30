use std::cmp;

fn get_square_feet(w: i32, h: i32, l: i32) -> i32 {
    2*l*w + 2*w*h + 2*h*l + cmp::min(2*l*w, cmp::min(2*w*h, 2*h*l))/2
}

pub fn day_2(input: String) {
    let mut sum = 0;
    let mut ribbot_feet = 0;

    for line in input.lines() {
        let mut whl: Vec<_>  = line.split("x")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        sum += get_square_feet(whl[0], whl[1], whl[2]);

        whl.sort();
        let prod = whl.iter().fold(1, |a, e| a * e);
        ribbot_feet += prod + whl[0] * 2 + whl[1] * 2;
    }

    println!("sum: {}", sum);
    println!("ribbon: {}", ribbot_feet);
}
