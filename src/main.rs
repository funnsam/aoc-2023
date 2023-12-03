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

mod day_01; mod day_02; mod day_03;

fn main() {
    let args = Arg::parse();
    let file = std::fs::read_to_string(args.file).unwrap();

    #[allow(clippy::zero_prefixed_literal)]
    match (args.day, args.nth) {
        (01, 1) => day_01::task_1(file),
        (01, 2) => day_01::task_2(file),
        (02, 1) => day_02::task_1(file),
        (02, 2) => day_02::task_2(file),
        (03, 1) => day_03::task_1(file),
        (03, 2) => day_03::task_2(file),
        _ => panic!("no such day and task!"),
    }
}
