extern crate regex;

use std::collections::HashMap;
use std::collections::hash_map::Entry;

use self::regex::Regex;

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

fn dist(map: &HashMap<(usize, usize), usize>, from: usize, to: usize) -> usize {
    let from_to = map.get(&(from, to));
    match from_to {
        Some(d) => *d,
        None    => {
            match map.get(&(to, from)) {
                Some(d) => *d,
                None    => {
                    0 // high number 
                } 
            }
        }
    }
}

pub fn day_9(input: String) {
    let input_regex = Regex::new(r"(\w*) to (\w*) = (\d*)").unwrap();

    let mut string_to_id: HashMap<String, usize> = HashMap::new();
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    let mut id_count = 0;

    for line in input.lines() {
        let captures = input_regex.captures(line).unwrap();
        let from = captures.at(1).unwrap();
        let to   = captures.at(2).unwrap().clone();
        let dist = captures.at(3).unwrap().parse::<usize>().unwrap();

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
        distances.insert((from_id, to_id), dist);
        println!("insert {}, {}", from_id, to_id);
    }

    let n = id_count;
    let v: Vec<_> = (0..n).collect();
    println!("numbers: {:?}", v);
    // TSP pretty much sucks anyways, so we'll be extra dumb
    let mut best = usize::min_value();
    for perm in permutations(v) {

        let mut current = 0;
        let zipped = perm.iter().zip(perm.iter().skip(1));

        for (a, b) in zipped {
            current += dist(&distances, *a, *b);
        }
        if current > best {
            best = current;
        }
    }
    println!("solution: {}", best);
    
}
