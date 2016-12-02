use std::env;

mod tasks;

fn main() {
    let mut args = env::args();
    let which_task = args.nth(1).unwrap().parse::<usize>().unwrap();

    match which_task {
        1 => tasks::task_1(),
        2 => tasks::task_2(),
        _ => {}
    }
}
