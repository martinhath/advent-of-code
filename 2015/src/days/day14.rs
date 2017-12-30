extern crate regex;

use self::regex::Regex;

struct Bird {
    speed: u32,
    duration: u32,
    rest: u32,
    distance: u32,
    flying: bool,
    since_last_change: u32,
}

fn bird_step(bird: &mut Bird) {
    // bird step confirmed as new music genre?
    bird.since_last_change += 1;
    if bird.flying {
        bird.distance += bird.speed;
        if bird.since_last_change >= bird.duration {
            bird.flying = false;
            bird.since_last_change = 0;
        }
    } else {
        if bird.since_last_change >= bird.rest {
            bird.flying = true;
            bird.since_last_change = 0;
        }
    }
}

pub fn day_14(input: String) {
    let time = 2503;
    let regex = Regex::new(r"(\d*) km/s for (\d*)\D*(\d*) seconds.").expect("regex error");
    // let mut max_dist = u32::min_value();
    let mut birds = Vec::new();

    for line in input.lines() {
        let captures = regex.captures(line).unwrap();
        let speed    = captures.at(1).unwrap().parse::<u32>().unwrap();
        let duration = captures.at(2).unwrap().parse::<u32>().unwrap();
        let rest     = captures.at(3).unwrap().parse::<u32>().unwrap();

        // Part I
        // let mut len: u32 = 0;
        // // whole 'rounds'
        // len += duration * speed * (time / (duration + rest));
        // let rest_time = min(duration, time % (duration + rest));
        // len += speed * rest_time;
        // 
        // if len > max_dist {
        //     max_dist = len;
        // }
        birds.push(Bird {
            speed: speed,
            duration: duration,
            rest: rest,
            distance: 0,
            flying: true,
            since_last_change: 0,
        });
    }

    let num_birds = birds.len();
    let indices: Vec<_> = (0..num_birds).collect();
    let mut points: Vec<usize> = Vec::new();
    for _ in 0..num_birds {
        points.push(0);
    }
    for _ in 1..time+1 {
        let _: Vec<_> = birds.iter_mut().map(bird_step).collect();
        let max = birds.iter().map(|b| b.distance).max().unwrap();

        for (_, &i) in birds.iter().zip(&indices).filter(|&(b, _)| b.distance == max) {
            points[i] += 1;
        }
    }

    println!("{} {}", time, points.iter().fold(0, |a, e| a + e));

    for (p, bird) in points.iter().zip(birds) {
        println!("Distance: {}\tPoints: {}", bird.distance, p);
    }

    // println!("max dist: {}", max_dist);
}

// Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
// Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
