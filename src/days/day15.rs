
struct Ingredient {
    cap: i64,
    dur: i64,
    fla: i64,
    tex: i64, 
    cal: i64,
}

impl Ingredient {
    fn from_str(str: &str) -> Ingredient {
        let split: Vec<_> = str.split(" ").collect();
        let mut cap_s = split[2].to_string(); cap_s.pop();
        let cap: i64 = cap_s.parse::<i64>().unwrap();
        let mut dur_s = split[4].to_string(); dur_s.pop();
        let dur: i64 = dur_s.parse::<i64>().unwrap();
        let mut fla_s = split[6].to_string(); fla_s.pop();
        let fla: i64 = fla_s.parse::<i64>().unwrap();
        let mut tex_s = split[8].to_string(); tex_s.pop();
        let tex: i64 = tex_s.parse::<i64>().unwrap();
        let cal: i64 = split[10].parse::<i64>().unwrap();

        Ingredient {
            cap: cap,
            dur: dur,
            fla: fla,
            tex: tex,
            cal: cal,
        }
    }
}

use std::cmp::max;

fn score(ingredients: &Vec<Ingredient>, weights: &Vec<i64>) -> i64 {
    let mut cap = 0;
    let mut dur = 0;
    let mut fla = 0;
    let mut tex = 0;
    let mut cal = 0;
    // let mut cal = 0;
    for (ing, w) in ingredients.iter().zip(weights.iter()) {
        cap += ing.cap * w;
        dur += ing.dur * w;
        fla += ing.fla * w;
        tex += ing.tex * w;
        cal += ing.cal * w;
    }

    if cal == 500 {
        max(0, cap) * max(0, dur) * max(0, fla) * max(0, tex)
    } else {
        i64::min_value()
    }
}

fn permutations<T>(list: Vec<T>) -> Vec<Vec<T>>
    where T: Copy + Ord { 
    if list.len() == 1 {
        return vec![list];
    }
    let head = list[0];
    let mut perms: Vec<Vec<T>> = Vec::new();
    let sublist: Vec<T> = list.into_iter().skip(1).collect();
    
    let sub_perms: Vec<Vec<T>> = permutations(sublist);

    // for each sub perm
    for perm in sub_perms {
        // for each place to insert head
        for i in 0..perm.len() + 1 {
            let mut cloned: Vec<T> = perm.clone();
            cloned.insert(i, head);
            perms.push(cloned);
        }
    }
    perms.sort();
    perms.dedup();
    perms
}

const SUM: i64 = 100;

fn sum100lists() -> Vec<Vec<i64>> {
    let mut v = Vec::new();
    for a in 0..(SUM + 1) {
        for b in a..(SUM + 1) {
            for c in b..(SUM + 1) {
                for d in c..(SUM + 1) {
                    if a + b + c + d == SUM {
                        v.push(vec![a, b, c, d]);
                    }
                }
            }
        }
    }
    v
}

fn flatten<T>(vec: Vec<Vec<T>>) -> Vec<T> 
    where T: Clone {
    let mut v: Vec<T> = Vec::new();

    for list in vec {
        let mut c = list.clone();
        v.append(&mut c);
    }
    v
}

pub fn day_15(input: String) {
    let ingredients: Vec<Ingredient> = input.lines().map(Ingredient::from_str).collect();

    let all_weights = flatten(sum100lists().into_iter().map(|v| permutations(v)).collect());
    let mut max = i64::min_value();

    for w in all_weights {
        let score = score(&ingredients, &w);
        if score > max {
            max = score;
            println!("best weight: {:?}", w);
        }
    }
    println!("max: {}", max);
}
