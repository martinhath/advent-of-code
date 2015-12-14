use std::collections::HashMap;

#[derive(Clone)]
struct Gate {
    output: String,
    input: Vec<String>,
    operator: Option<String>

}

fn apply(operator: &str, operands: &Vec<u16>) -> u16 {
    match operator {
        "AND"    => operands[0] & operands[1],
        "OR"     => operands[0] | operands[1],
        "LSHIFT" => operands[0] << operands[1],
        "RSHIFT" => operands[0] >> operands[1],
        "NOT"    => !operands[0],
        s        => panic!("Unknown operator: {}", s)
    }
}

fn solve(target: &str, gates: &Vec<Gate>, memo: &mut HashMap<String, u16>) -> u16 {
    if memo.contains_key(target) {
        return *memo.get(target).unwrap();
    }
    let mut memo = memo;
    let g = gates.iter().find(|g| g.output == target);
    if g.is_none() {
        panic!("Can't find symbol {}", target);
    }
    let gate = g.unwrap();


    match gate.operator {
        None     => {
            let p = gate.input[0].parse::<u16>();
            match p {
                Ok(n)  => n,
                Err(_) => {
                    let res = solve(&gate.input[0], gates, &mut memo);
                    memo.insert(target.to_string(), res);
                    res
                }
            }
        }
        Some(ref op) => {

            let mut ops = Vec::new();

            for e in gate.input.iter() {
                let parsed = e.parse::<u16>();
                ops.push(match parsed {
                    Err(_) => {
                        let res = solve(e, &gates, &mut memo);
                        memo.insert(target.to_string(), res);
                        res
                    },
                    Ok(n)  => n
                });
            }
            let res = apply(op, &ops);
            memo.insert(target.to_string(), res);
            res
        }

    }
}

pub fn day_7(input: String) {
    let mut gates = Vec::new();

    for line in input.lines() {
        // a OR b -> c

        let mut arrow_it = line.split(" -> ");
        let lhs = arrow_it.next().unwrap();
        let rhs = arrow_it.next().unwrap();

        let lhs: Vec<_> = lhs.split(" ").collect();
        let input;
        let mut operator = None;
        match lhs.len() {
            1 => {
                input = vec![lhs[0].to_string()];
            }
            2 => {
                operator = Some(lhs[0].to_string());
                input = vec![lhs[1].to_string()];
            }
            3 => {
                let mut v = Vec::new();
                v.push(lhs[0].to_string());
                operator = Some(lhs[1].to_string());
                v.push(lhs[2].to_string());
                input = v;
            }
            _ => {
                panic!("lhs was {:?}", lhs);
            }
        };

        gates.push(Gate {
            output: rhs.to_string(),
            input: input,
            operator: operator,
        });
    }

    let target = "a";
    let mut memo = HashMap::new();
    println!("{} = {}", target, solve(target, &gates, &mut memo));
}
