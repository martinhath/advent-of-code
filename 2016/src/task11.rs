extern crate rand;

use regex::*;
use std::collections::HashMap;
use std::collections::BinaryHeap;

pub fn task_11_fast() {
    use std::hash::{Hash, Hasher};

    #[derive(Debug, Hash, Eq, PartialEq, Clone, Ord, PartialOrd)]
    enum E<'a> {
        Generator(&'a str),
        Microchip(&'a str),
    }

    impl<'a> E<'a> {
        fn is_generator(&self) -> bool {
            match self {
                &E::Generator(_) => true,
                &E::Microchip(_) => false,
            }
        }

        fn is_microchip(&self) -> bool {
            !self.is_generator()
        }

        fn label(&self) -> &'a str {
            match self {
                &E::Generator(s) => s,
                &E::Microchip(s) => s,
            }
        }
    }

    #[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd)]
    struct State<'a> {
        game: Vec<(E<'a>, usize)>,
        elevator: usize,
        levels: usize,
        neighbours: Vec<State<'a>>,
    }

    impl<'a> State<'a> {
        fn id(&self) -> usize {
            let mut id = 0;
            id += self.elevator;
            let mut f = 1;
            for &(_, n) in self.game.iter() {
                f *= 4;
                id += f * n;
            }
            id
        }
        fn neighbours(&self) -> Vec<State<'a>> {
            self.neighbours.clone()
        }

        fn generate_neighbours(&mut self) {
            let mut v = Vec::with_capacity(16);
            let inds = self.game.iter()
                .enumerate()
                .filter(|&(_, &(_, n))| n == self.elevator)
                .map(|(i, _)| i)
                .collect::<Vec<usize>>();
            if self.elevator > 1 {
                // Move stuff down
                let n = inds.len();
                for i in 0..n {
                    for j in (i + 1)..n {
                        let mut s = self.clone();
                        s.game[inds[i]].1 -= 1;
                        s.game[inds[j]].1 -= 1;
                        s.elevator -= 1;
                        v.push(s);
                    }
                    let mut s = self.clone();
                    s.game[inds[i]].1 -= 1;
                    s.elevator -= 1;
                    v.push(s);
                }
            }
            if self.elevator < self.levels {
                // Move stuff up
                let n = inds.len();
                for i in 0..n {
                    for j in (i + 1)..n {
                        let mut s = self.clone();
                        s.game[inds[i]].1 += 1;
                        s.game[inds[j]].1 += 1;
                        s.elevator += 1;
                        v.push(s);
                    }
                    let mut s = self.clone();
                    s.game[inds[i]].1 += 1;
                    s.elevator += 1;
                    v.push(s);
                }
            }
            v.retain(State::no_fries);
            self.neighbours = v;
        }

        fn no_fries(&self) -> bool {
            (0..self.levels).all(|level| {
                let gens = || self.game.iter()
                    .filter(|&&(ref e, l)| level == l && e.is_generator());
                gens().next().is_none() || self.game.iter()
                    .filter(|&&(ref e, l)| level == l && e.is_microchip())
                    .all(|&(ref e, _)| {
                        // For each chip, check that its generator is in `gens`.
                        // If not, there is a gen on the same floor that isn't
                        // the correct gen for this chip, and it will be fried.
                        let label = e.label();
                        gens().filter(|&&(ref g, _)| {
                           g.is_generator() && g.label() == label
                        }).next().is_some()
                    })
            })
        }

        fn is_done(&self) -> bool {
            self.game.iter().all(|&(_, n)| n == self.levels)
        }

        // fn print_short(&self) {
        //     print!("{}", self.elevator);
        //     for &(_, e) in &self.game {
        //         print!("{}", e);
        //     }
        //     println!("");
        // }
    }

    impl<'a> Hash for State<'a> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.elevator.hash(state);
            for &(_, n) in &self.game {
                n.hash(state);
            }
        }
    }

//     let input = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
// The second floor contains a hydrogen generator.
// The third floor contains a lithium generator.
// The fourth floor contains nothing relevant.";

    let input = "The first floor contains a thulium generator, a thulium-compatible microchip, a plutonium generator, a strontium generator, a elerium generator, a elerium-compatible microchip.
The third floor contains a promethium generator, a promethium-compatible microchip, a ruthenium generator, and a ruthenium-compatible microchip.
The fourth floor contains nothing relevant.";
    let reg = Regex::new(r"([a-zA-Z-]+) (microchip|generator)").unwrap();
    let mut state = State {
        game: vec![],
        elevator: 1,
        levels: 4,
        neighbours: vec![],
    };
    for (lineno, line) in input.lines().enumerate() {
        for cap in reg.captures_iter(line) {
            let m = cap.at(1).unwrap();
            let t = cap.at(2).unwrap();
            let e = match t {
                "generator" => { E::Generator(m) }
                "microchip" => { E::Microchip(m.split("-").next().unwrap()) }
                _ => unreachable!(&format!("What is {}", t)),
            };
            state.game.push((e, lineno + 1));
        }
    }

    #[derive(PartialEq, Eq, PartialOrd)]
    struct QueueE {
        id: usize,
        steps: isize,
        score: isize,
    }
    use std::cmp;
    impl Ord for QueueE {
        fn cmp(&self, other: &QueueE) -> cmp::Ordering {
            (self.score, self.steps, self.id).cmp(&(other.score, other.steps, other.id))
        }
    }

    state.generate_neighbours();
    // Mapping id -> State
    let mut states: HashMap<usize, State> = HashMap::new();
    let init_state_id = state.id();
    states.insert(init_state_id, state);
    // Mapping id -> score
    let mut table: HashMap<usize, isize> = HashMap::new();
    table.insert(init_state_id, 0);
    // Queue (score, id)
    let mut state_queue: BinaryHeap<QueueE> = BinaryHeap::new();
    state_queue.push( QueueE { steps: 0, id: init_state_id, score: 0 });

    'outer: while let Some(e) = state_queue.pop() {
        let base = e.steps;
        let s_id = e.id;
        for neigh in states.get(&s_id).unwrap().neighbours() {
            let neigh_id = neigh.id();
            states.entry(neigh_id).or_insert_with(|| {
                let mut n = neigh.clone();
                n.generate_neighbours();
                n
            });
            if states.get(&neigh_id).unwrap().is_done() {
                return;
            }

            let mut prev = table.entry(neigh_id).or_insert(-99999);
            if *prev < base - 1 {
                *prev = base - 1;
                let score = *prev + neigh.game.iter().map(|&(_, n)| n as isize).sum::<isize>();
                let e = QueueE {
                    id: neigh_id,
                    steps: *prev,
                    score: score,
                };
                state_queue.push(e);
            }
        }
    }
}

pub fn task_11_slow() {
    use std::hash::{Hash, Hasher};

    #[derive(Debug, Hash, Eq, PartialEq, Clone, Ord, PartialOrd)]
    enum E<'a> {
        Generator(&'a str),
        Microchip(&'a str),
    }

    impl<'a> E<'a> {
        fn is_generator(&self) -> bool {
            match self {
                &E::Generator(_) => true,
                &E::Microchip(_) => false,
            }
        }

        fn is_microchip(&self) -> bool {
            !self.is_generator()
        }

        fn label(&self) -> &'a str {
            match self {
                &E::Generator(s) => s,
                &E::Microchip(s) => s,
            }
        }
    }

    #[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd)]
    struct State<'a> {
        game: Vec<(E<'a>, usize)>,
        elevator: usize,
        levels: usize,
        neighbours: Vec<State<'a>>,
    }

    impl<'a> State<'a> {
        fn id(&self) -> usize {
            let mut id = 0;
            id += self.elevator;
            let mut f = 1;
            for &(_, n) in self.game.iter() {
                f *= 4;
                id += f * n;
            }
            id
        }
        fn neighbours(&self) -> Vec<State<'a>> {
            self.neighbours.clone()
        }

        fn generate_neighbours(&mut self) {
            let mut v = Vec::with_capacity(16);
            let inds = self.game.iter()
                .enumerate()
                .filter(|&(_, &(_, n))| n == self.elevator)
                .map(|(i, _)| i)
                .collect::<Vec<usize>>();
            if self.elevator > 1 {
                // Move stuff down
                let n = inds.len();
                for i in 0..n {
                    for j in (i + 1)..n {
                        let mut s = self.clone();
                        s.game[inds[i]].1 -= 1;
                        s.game[inds[j]].1 -= 1;
                        s.elevator -= 1;
                        v.push(s);
                    }
                    let mut s = self.clone();
                    s.game[inds[i]].1 -= 1;
                    s.elevator -= 1;
                    v.push(s);
                }
            }
            if self.elevator < self.levels {
                // Move stuff up
                let n = inds.len();
                for i in 0..n {
                    for j in (i + 1)..n {
                        let mut s = self.clone();
                        s.game[inds[i]].1 += 1;
                        s.game[inds[j]].1 += 1;
                        s.elevator += 1;
                        v.push(s);
                    }
                    let mut s = self.clone();
                    s.game[inds[i]].1 += 1;
                    s.elevator += 1;
                    v.push(s);
                }
            }
            v.retain(State::no_fries);
            self.neighbours = v;
        }

        fn no_fries(&self) -> bool {
            (0..self.levels).all(|level| {
                let gens = || self.game.iter()
                    .filter(|&&(ref e, l)| level == l && e.is_generator());
                gens().next().is_none() || self.game.iter()
                    .filter(|&&(ref e, l)| level == l && e.is_microchip())
                    .all(|&(ref e, _)| {
                        // For each chip, check that its generator is in `gens`.
                        // If not, there is a gen on the same floor that isn't
                        // the correct gen for this chip, and it will be fried.
                        let label = e.label();
                        gens().filter(|&&(ref g, _)| {
                           g.is_generator() && g.label() == label
                        }).next().is_some()
                    })
            })
        }

        fn is_done(&self) -> bool {
            self.game.iter().all(|&(_, n)| n == self.levels)
        }

        // fn print_short(&self) {
        //     print!("{}", self.elevator);
        //     for &(_, e) in &self.game {
        //         print!("{}", e);
        //     }
        //     println!("");
        // }
    }

    impl<'a> Hash for State<'a> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.elevator.hash(state);
            for &(_, n) in &self.game {
                n.hash(state);
            }
        }
    }

//     let input = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
// The second floor contains a hydrogen generator.
// The third floor contains a lithium generator.
// The fourth floor contains nothing relevant.";

    let input = "The first floor contains a thulium generator, a thulium-compatible microchip, a plutonium generator, a strontium generator, a elerium generator, a elerium-compatible microchip.
The third floor contains a promethium generator, a promethium-compatible microchip, a ruthenium generator, and a ruthenium-compatible microchip.
The fourth floor contains nothing relevant.";
    let reg = Regex::new(r"([a-zA-Z-]+) (microchip|generator)").unwrap();
    let mut state = State {
        game: vec![],
        elevator: 1,
        levels: 4,
        neighbours: vec![],
    };
    for (lineno, line) in input.lines().enumerate() {
        for cap in reg.captures_iter(line) {
            let m = cap.at(1).unwrap();
            let t = cap.at(2).unwrap();
            let e = match t {
                "generator" => { E::Generator(m) }
                "microchip" => { E::Microchip(m.split("-").next().unwrap()) }
                _ => unreachable!(&format!("What is {}", t)),
            };
            state.game.push((e, lineno + 1));
        }
    }

    #[derive(PartialEq, Eq, PartialOrd)]
    struct QueueE {
        id: usize,
        steps: isize,
        score: isize,
    }
    use std::cmp;
    impl Ord for QueueE {
        fn cmp(&self, other: &QueueE) -> cmp::Ordering {
            (self.score, self.steps, self.id).cmp(&(other.score, other.steps, other.id))
        }
    }

    state.generate_neighbours();
    // Mapping id -> State
    let mut states: HashMap<usize, State> = HashMap::new();
    let init_state_id = state.id();
    states.insert(init_state_id, state);
    // Mapping id -> score
    let mut table: HashMap<usize, isize> = HashMap::new();
    table.insert(init_state_id, 0);
    // Queue (score, id)
    let mut state_queue: BinaryHeap<(isize, usize)> = BinaryHeap::new();
    state_queue.push((0, init_state_id));

    'outer: while let Some((base, s_id)) = state_queue.pop() {
        for neigh in states.get(&s_id).unwrap().neighbours() {
            let neigh_id = neigh.id();
            states.entry(neigh_id).or_insert_with(|| {
                let mut n = neigh.clone();
                n.generate_neighbours();
                n
            });
            if states.get(&neigh_id).unwrap().is_done() {
                return;
            }

            let mut prev = table.entry(neigh_id).or_insert(-99999);
            if *prev < base - 1 {
                *prev = base - 1;
                state_queue.push((*prev, neigh_id));
            }
        }
    }
}

mod bench {

    // #[bench]
    // pub fn fast_version(b: &mut Bencher) {
    //     b.iter(|| task_11_fast());
    // }

    #[bench]
    pub fn hash_tuple(b: &mut Bencher) {
        type T = (isize, usize);
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hash;
        let mut hasher = DefaultHasher::new();
        b.iter(|| {
            (0, 0).hash(&mut hasher);
            (0, 0).hash(&mut hasher);
            (0, 0).hash(&mut hasher);
            (0, 0).hash(&mut hasher);
        });
    }

    #[bench]
    pub fn hash_struct(b: &mut Bencher) {
        #[derive(Hash)]
        struct T {
            a: isize,
            b: usize,
        };
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hash;
        let mut hasher = DefaultHasher::new();
        b.iter(|| {
            (0, 0).hash(&mut hasher);
            (0, 0).hash(&mut hasher);
            (0, 0).hash(&mut hasher);
            (0, 0).hash(&mut hasher);
        });
    }

    #[bench]
    pub fn heap_tuple(b: &mut Bencher) {
        const N: usize = 100_000;
        use std::sync::atomic::*;
        use std::iter::repeat;
        let v = repeat(self::rand::random::<i32>()).take(N).collect::<Vec<_>>();

        let mut q: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        let mut n = 0;
        b.iter(|| {
            for _ in 0..100 {
                let a = v[n];
                q.push((a, a));
                if n < N - 1{
                    n += 1;
                } else {
                    n = 0;
                }
            }
        });
    }

    #[bench]
    pub fn heap_struct(b: &mut Bencher) {
        #[derive(PartialOrd, Eq, PartialEq)]
        struct T {
            a: i32,
            b: i32,
        };
        use std::cmp;
        impl Ord for T {
            fn cmp(&self, other: &T) -> cmp::Ordering {
                (self.a, self.b).cmp(&(other.a, other.b))
            }
        }
        const N: usize = 100_000;
        use std::sync::atomic::*;
        use std::iter::repeat;
        let v = repeat(self::rand::random::<i32>()).take(N).collect::<Vec<_>>();

        let mut q: BinaryHeap<T> = BinaryHeap::new();
        let mut n = 0;
        b.iter(|| {
            for _ in 0..100 {
                let a = v[n];
                q.push(T {
                    a: a,
                    b: a,
                });
                if n < N - 1{
                    n += 1;
                } else {
                    n = 0;
                }
            }
        });
    }
    // #[bench]
    // pub fn slow_version(b: &mut Bencher) {
    //     b.iter(|| task_11_slow());
    // }
}
