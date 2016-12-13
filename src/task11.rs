use regex::*;
use std::collections::HashMap;


/// Hard coded for input:
///
///     thulium generator
///     thulium-compatible microchip
///     plutonium generator
///     plutonium-compatible microchip
///     strontium generator
///     strontium-compatible microchip.
///     elerium generator
///     elerium-compatible microchip
///     dilithium generator
///     dilithium-compatible microchip.
///     promethium generator
///     promethium-compatible microchip
///     ruthenium generator
///     ruthenium-compatible microchip.
///
///     7 * 2 + 1 = 15 items.
///     4 floors -> total of 60 bits.

// #[derive(Clone)]
// pub struct State {
//     state: u64
// }
// 
// impl State {
// 
//     fn from_vec(vec: &Vec<u64>) -> Self {
//         let mut n = 0usize;
// 
//         for (i, &a) in vec.iter().enumerate() {
//             assert!(a < 4);
//             n |=  a << (2 *  (i + 1));
//         }
// 
//         State {
//             state: n,
//         }
//     }
// 
//     fn floor(&self) -> u8 {
//         (self.state & 0b11) as u8
//     }
// 
//     fn is_top(&self) -> bool {
//         self.floor == 3
//     }
// 
//     fn is_bottom(&self) -> bool {
//         self.floor == 0
//     }
// 
//     fn has_generators(&self) -> bool {
//         const MASK: u64 = 0xcccccccccccccccc;
//         let and = MAKS & self.state;
//         and >> 0 & 0xc == 0xc ||
//             and >> 4
// 
//     }
// 
//     fn next_states(&self) -> Vec<State> {
// 
//     }
// }
// 
// #[cfg(test)]
// mod tests {
//     use super::*;
// 
//     #[test]
//     fn state_from_vec() {
//         let v = vec![0, 0, 1, 3];
//         let state = State::from_vec(&v);
//         assert_eq!(state.state, 0b1101000000);
//     }
// }
// 
// 
// 
// pub fn task_11() {
//     use std::hash::{Hash, Hasher};
// 
// 
//     #[derive(Debug, Hash, Eq, PartialEq, Clone, Ord, PartialOrd)]
//     enum E<'a> {
//         Generator(&'a str),
//         Microchip(&'a str),
//     }
// 
//     impl<'a> E<'a> {
//         fn is_generator(&self) -> bool {
//             match self {
//                 &E::Generator(_) => true,
//                 &E::Microchip(_) => false,
//             }
//         }
// 
//         fn is_microchip(&self) -> bool {
//             !self.is_generator()
//         }
// 
//         fn label(&self) -> &'a str {
//             match self {
//                 &E::Generator(s) => s,
//                 &E::Microchip(s) => s,
//             }
//         }
//     }
// 
//     #[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd)]
//     struct State<'a> {
//         game: Vec<(E<'a>, usize)>,
//         elevator: usize,
//         levels: usize,
//         neighbours: Vec<State<'a>>,
//     }
// 
//     impl<'a> State<'a> {
//         fn id(&self) -> usize {
//             let mut id = 0;
//             id += self.elevator;
//             let mut f = 1;
//             for &(_, n) in self.game.iter() {
//                 f *= 4;
//                 id += f * n;
//             }
//             id
//         }
//         fn neighbours(&self) -> Vec<State<'a>> {
//             self.neighbours.clone()
//         }
// 
//         fn generate_neighbours(&mut self) {
//             let mut v = Vec::with_capacity(16);
//             let inds = self.game.iter()
//                 .enumerate()
//                 .filter(|&(_, &(_, n))| n == self.elevator)
//                 .map(|(i, _)| i)
//                 .collect::<Vec<usize>>();
//             if self.elevator > 1 {
//                 // Move stuff down
//                 let n = inds.len();
//                 for i in 0..n {
//                     for j in (i + 1)..n {
//                         let mut s = self.clone();
//                         s.game[inds[i]].1 -= 1;
//                         s.game[inds[j]].1 -= 1;
//                         s.elevator -= 1;
//                         v.push(s);
//                     }
//                     let mut s = self.clone();
//                     s.game[inds[i]].1 -= 1;
//                     s.elevator -= 1;
//                     v.push(s);
//                 }
//             }
//             if self.elevator < self.levels {
//                 // Move stuff up
//                 let n = inds.len();
//                 for i in 0..n {
//                     for j in (i + 1)..n {
//                         let mut s = self.clone();
//                         s.game[inds[i]].1 += 1;
//                         s.game[inds[j]].1 += 1;
//                         s.elevator += 1;
//                         v.push(s);
//                     }
//                     let mut s = self.clone();
//                     s.game[inds[i]].1 += 1;
//                     s.elevator += 1;
//                     v.push(s);
//                 }
//             }
//             v.retain(State::no_fries);
//             self.neighbours = v;
//         }
// 
//         fn no_fries(&self) -> bool {
//             (0..self.levels).all(|level| {
//                 let gens = || self.game.iter()
//                     .filter(|&&(ref e, l)| level == l && e.is_generator());
//                 gens().next().is_none() || self.game.iter()
//                     .filter(|&&(ref e, l)| level == l && e.is_microchip())
//                     .all(|&(ref e, _)| {
//                         // For each chip, check that its generator is in `gens`.
//                         // If not, there is a gen on the same floor that isn't
//                         // the correct gen for this chip, and it will be fried.
//                         let label = e.label();
//                         gens().filter(|&&(ref g, _)| {
//                            g.is_generator() && g.label() == label
//                         }).next().is_some()
//                     })
//             })
//         }
// 
//         fn is_done(&self) -> bool {
//             self.game.iter().all(|&(_, n)| n == self.levels)
//         }
// 
//         fn print_short(&self) {
//             print!("{}", self.elevator);
//             for &(_, e) in &self.game {
//                 print!("{}", e);
//             }
//             println!("");
//         }
//     }
// 
//     impl<'a> Hash for State<'a> {
//         fn hash<H: Hasher>(&self, state: &mut H) {
//             self.elevator.hash(state);
//             for &(_, n) in &self.game {
//                 n.hash(state);
//             }
//         }
//     }
// 
// //     let input = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
// // The second floor contains a hydrogen generator.
// // The third floor contains a lithium generator.
// // The fourth floor contains nothing relevant.";
// 
//     // let input = "The first floor contains a thulium generator, a thulium-compatible microchip, a plutonium generator, a strontium generator, a elerium generator, a elerium-compatible microchip, a dilithium generator, and a dilithium-compatible microchip.
//     let input = "The first floor contains a thulium generator, a thulium-compatible microchip, a plutonium generator, a strontium generator.
// The second floor contains a plutonium-compatible microchip and a strontium-compatible microchip.
// The third floor contains a promethium generator, a promethium-compatible microchip, a ruthenium generator, and a ruthenium-compatible microchip.
// The fourth floor contains nothing relevant.";
//     let reg = Regex::new(r"([a-zA-Z-]+) (microchip|generator)").unwrap();
//     let mut state = State {
//         game: vec![],
//         elevator: 1,
//         levels: 4,
//         neighbours: vec![],
//     };
//     for (lineno, line) in input.lines().enumerate() {
//         for cap in reg.captures_iter(line) {
//             let m = cap.at(1).unwrap();
//             let t = cap.at(2).unwrap();
//             let e = match t {
//                 "generator" => { E::Generator(m) }
//                 "microchip" => { E::Microchip(m.split("-").next().unwrap()) }
//                 _ => unreachable!(&format!("What is {}", t)),
//             };
//             state.game.push((e, lineno + 1));
//         }
//     }
// 
//     use std::collections::BinaryHeap;
// 
//     state.generate_neighbours();
//     // Mapping id -> State
//     let mut states: HashMap<usize, State> = HashMap::new();
//     let init_state_id = state.id();
//     states.insert(init_state_id, state);
//     // Mapping id -> score
//     let mut table: HashMap<usize, isize> = HashMap::new();
//     table.insert(init_state_id, 0);
//     // Queue (score, id)
//     let mut state_queue: BinaryHeap<(isize, usize)> = BinaryHeap::new();
//     state_queue.push((0, init_state_id));
// 
//     let mut iters = 0;
//     'outer: while let Some((base, s_id)) = state_queue.pop() {
//         for neigh in states.get(&s_id).unwrap().neighbours() {
//             let neigh_id = neigh.id();
//             states.entry(neigh_id).or_insert_with(|| {
//                 let mut n = neigh.clone();
//                 n.generate_neighbours();
//                 if neigh.is_done() {
//                     println!("iters: {}", iters);
//                     println!("{}", -base + 1);
//                     panic!();
//                 }
//                 n
//             });
// 
//             let mut prev = table.entry(neigh_id).or_insert(-99999);
//             if *prev < base - 1 {
//                 *prev = base - 1;
//                 state_queue.push((*prev, neigh_id));
//             }
//         }
//         iters += 1;
//     }
// }
