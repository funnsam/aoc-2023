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
#![feature(iter_array_chunks)]

use clap::{Parser, ArgAction};
use cpu_time::*;

#[derive(Parser)]
struct Arg {
    day: usize,
    nth: usize,

    #[clap(short, long, default_value = "input.txt")]
    file: String,

    #[clap(short, long)]
    benchmark: Option<usize>,

    #[clap(short, long, action = ArgAction::SetTrue)]
    skip_ans: bool,
}

mod day_01; mod day_02; mod day_03; mod day_04;
mod day_05; mod day_06; mod day_07; mod day_08;
mod day_09;

const TASKS: &[&'static dyn Fn(&str) -> String] = &[
    &day_01::task_1, &day_01::task_2,
    &day_02::task_1, &day_02::task_2,
    &day_03::task_1, &day_03::task_2,
    &day_04::task_1, &day_04::task_2,
    &day_05::task_1, &day_05::task_2,
    &day_06::task_1, &day_06::task_2,
    &day_07::task_1, &day_07::task_2,
    &day_08::task_1, &day_08::task_2,
    &day_09::task_1, &day_09::task_2,
];

fn main() {
    let args = Arg::parse();
    let file = std::fs::read_to_string(args.file).unwrap();

    let task = TASKS[args.day * 2 + args.nth - 3];
    
    if !args.skip_ans {
        let ans = task(&file);
        println!("\x1b[1;92mAns:\x1b[0m {}", ans);
    }

    if let Some(n) = args.benchmark {
        println!("\x1b[90mBenchmarking...\x1b[0m");
        let s = ProcessTime::now();
        for _ in 0..n {
            task(&file);
        }
        let e = s.elapsed();
        println!("\x1b[1;94mAverage time:\x1b[0m {:.03}μs / {:.01}ns", e.as_secs_f64() / 1e-6 / n as f64, e.as_secs_f64() / 1e-9 / n as f64);
    }
}
