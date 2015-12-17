use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn permutations<T>(list: Vec<T>) -> Vec<Vec<T>>
    where T: Copy { 
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
    perms
}

pub fn day_13(input: String) {
    let mut string_to_id: HashMap<String, usize> = HashMap::new();
    let mut scores: HashMap<(usize, usize), i64> = HashMap::new();
    let mut id_count = 0;

    for line in input.lines() {
        let split: Vec<_> = line.split(" ").collect();;
        let from = split[0].to_string();
        let mut to = split[10].to_string(); to.pop();
        let sign = split[2].to_string();
        let num  = split[3].parse::<i64>().expect("num parse error");

        let from_id = match string_to_id.entry(from.to_string()) {
            Entry::Occupied(o) => *o.get(),
            Entry::Vacant(v)   => {
                v.insert(id_count);
                id_count += 1;
                id_count - 1
            }
        };
        let to_id = match string_to_id.entry(to.to_string()) {
            Entry::Occupied(o) => *o.get(),
            Entry::Vacant(v)   => {
                v.insert(id_count);
                id_count += 1;
                id_count - 1
            }
        };
        scores.insert((from_id, to_id), num * if sign=="gain" {1} else {-1});
        println!("insert {}, {}", from_id, to_id);
    }

    let n = id_count;
    let v: Vec<_> = (0..n).collect();

    let mut best = i64::min_value();
    for perm in permutations(v) {
        let v = perm;
        // part II: as we generate all permutations instead of all
        //          circles, placing ourself only at the end of the
        //          list doens't remove any combinations, while making
        //          sure the two ends of the tables preferences aren't 
        //          included in the calculation.
        //
        // let f = v[0];
        // v.push(f); // circular
        let zip = v.iter().zip(v.iter().skip(1));

        let mut score = 0;
        for (a, b) in zip {
            score += *scores.get(&(*a, *b)).unwrap();
            score += *scores.get(&(*b, *a)).unwrap();
        }
        if score > best {
            best = score;
        }
    }
    println!("solution: {}", best);
    
}
