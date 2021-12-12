use rust_aoc::*;
use std::env;
use std::io;
use std::process;

const NUM_DAYS: u8 = 25;

fn main() {
    let day_num: u8;

    let args: Vec<String> = env::args().collect();
    let argc = args.len();
    if argc == 1 {
        println!("Please input AoC day");
        loop {
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            day_num = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please put in a number from 1 to {}.", NUM_DAYS);
                    continue;
                }
            };
            break;
        }
    } else {
        day_num = match args[1].parse() {
            Ok(num) => num,
            Err(_) => 0,
        };
    }

    let visualize =
        args.contains(&String::from("--visualize")) || args.contains(&String::from("-v"));

    let filename = format!("data/input-{}.txt", day_num);

    let lines = common::read_lines(&filename).unwrap_or_else(|_| {
        common::download_input(day_num).unwrap();
        common::read_lines(&filename).unwrap_or_else(|error| {
            println!("{:?}", error);
            process::exit(1);
        })
    });
    let result: (u64, u64) = match day_num {
        1 => day_1::run(&lines),
        2 => day_2::run(&lines),
        3 => day_3::run(&lines),
        4 => day_4::run(&lines),
        5 => day_5::run(&lines),
        6 => day_6::run(&lines),
        7 => day_7::run(&lines),
        8 => day_8::run(&lines),
        9 => day_9::run(&lines, visualize),
        10 => day_10::run(&lines),
        11 => day_11::run(&lines),
        12 => day_12::run(&lines),
        _ => (0, 0),
    };
    println!("part 1: {}\npart 2: {}", result.0, result.1);
}
