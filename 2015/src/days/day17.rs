fn part1(nums: &[usize], vol: usize) -> usize {
    if vol == 0 {
        return 1;
    }
    if nums.len() <= 1 {
        if nums[0] == vol {
            return 1;
        } else {
            return 0;
        }
    }

    let (head, tail) = nums.split_at(1);
    let head = head[0];

    let mut res = 0;
    if head <= vol {
        res += part1(tail, vol - head);
    }
    res += part1(tail, vol);
    res
}

fn find_min_num(nums: &[usize], vol: usize) -> usize {
    if nums.contains(&vol) {
        return 1;
    }
    if nums.len() <= 1 { return 987654; }
    let m = (0..nums.len())
        .filter(|&i| vol >= nums[i])
        .map(|i| 1 + find_min_num(&nums[(i + 1)..], vol - nums[i]))
        .min();
    if m .is_none() {
        987654
    } else {
        m.unwrap()
    }
}

fn part2(nums: &[usize], vol: usize, left: usize) -> usize {
    if nums.len() == 0 {
        return 0;
    }
    if left == 1 {
        return nums.iter().filter(|&n| *n == vol).count();
    }
    (0..nums.len())
        .map(|i| part2(&nums[(i + 1)..], vol - nums[i], left - 1))
        .fold(0, |a, e| a + e)
}

pub fn day_17(input: String) {
    let volume = 150;
    let containers: Vec<_> = input.lines()
        .map(|s| s.parse::<usize>().unwrap()).collect();
    // Part I
    // println!("solution: {}", part1(&containers[..], volume));
    
    let n = find_min_num(&containers[..], volume);
    println!("n = {}", n);
    let sol = part2(&containers[..], volume, n);
    println!("solution: {}", sol);
}

