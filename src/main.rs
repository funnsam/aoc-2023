#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::nursery,
    clippy::suspicious,
    clippy::style,
)]
#![allow(
    clippy::semicolon_inside_block,
    clippy::just_underscores_and_digits,
)]

use clap::Parser;

macro_rules! report {
    ($($fmt: tt)*) => {
        println!("\x1b[1;32mAns:\x1b[0m {}", format!($($fmt)*));
    };
}

#[derive(Parser)]
struct Arg {
    day: usize,
    nth: usize,

    #[clap(short, long, default_value = "input.txt")]
    file: String,
}

mod day_01; mod day_02; mod day_03; mod day_04;

const TASKS: &[&'static dyn Fn(String)] = &[
    &day_01::task_1, &day_01::task_2,
    &day_02::task_1, &day_02::task_2,
    &day_03::task_1, &day_03::task_2,
    &day_04::task_1, &day_04::task_2,
];

fn main() {
    let args = Arg::parse();
    let file = std::fs::read_to_string(args.file).unwrap();

    TASKS[args.day * 2 + args.nth - 3](file);
}
