// #![feature(test)]
// extern crate test;

extern crate regex;
extern crate crypto;
#[macro_use]
extern crate lazy_static;
extern crate permutohedron;

use std::env;

mod tasks;

mod task11;

fn main() {
    let mut args = env::args();
    let which_task = args.nth(1).unwrap().parse::<usize>().unwrap();

    match which_task {
        1 => tasks::task_1(),
        2 => tasks::task_2(),
        3 => tasks::task_3(),
        4 => tasks::task_4(),
        5 => tasks::task_5(),
        6 => tasks::task_6(),
        7 => tasks::task_7(),
        8 => tasks::task_8(),
        9 => tasks::task_9(),
        10 => tasks::task_10(),
        11 => tasks::task_11(),
        12 => tasks::task_12(),
        13 => tasks::task_13(),
        14 => tasks::task_14(),
        15 => tasks::task_15(),
        16 => tasks::task_16(),
        17 => tasks::task_17(),
        18 => tasks::task_18(),
        19 => tasks::task_19(),
        20 => tasks::task_20(),
        21 => tasks::task_21(),
        22 => tasks::task_22(),
        23 => tasks::task_23(),
        24 => tasks::task_24(),
        25 => tasks::task_25(),
        _ => {}
    }
}
