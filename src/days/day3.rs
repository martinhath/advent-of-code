use std::collections::HashSet;

fn next(acc: Vec<(i32, i32)>, elem: &char) -> Vec<(i32, i32)> {
    let mut points = acc;
    let (x, y): (i32, i32) = *points.last().unwrap_or(&(0 ,0));
    let next = match *elem {
        '>' => (x + 1, y),
        '<' => (x - 1, y),
        'v' => (x, y - 1),
        '^' => (x, y + 1),
        _   => {
           return points;
        }
    };
    points.push(next);
    points
}

pub fn day_3(input: String) {
    let mut santa_input = Vec::new();
    let mut robot_input = Vec::new();

    let mut f = false;
    for c in input.chars() {
        if f {
            santa_input.push(c);
        } else {
            robot_input.push(c);
        }
        f = !f;
    }


    let mut positions = santa_input.iter().fold(Vec::new(), next);
    positions.extend(robot_input.iter().fold(Vec::new(), next));

    let mut uniques = HashSet::new();
    for pos in positions {
        uniques.insert(pos);
    }

    println!("Set size: {:?}", uniques.len());
}
