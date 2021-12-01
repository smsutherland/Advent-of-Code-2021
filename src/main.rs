use rust_aoc::*;
use std::env;
use std::io;
use std::process;

const NUM_DAYS: u8 = 25;

fn main() {
    let day_num: u32;

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

    let day_num = day_num;

    let filename = format!("data/input-{}.txt", day_num);

    let lines = common::read_lines(filename).unwrap_or_else(|error| {
        println!("{:?}", error);
        process::exit(1);
    });
    let result: (u32, u32) = match day_num {
        1 => day_1::day_1(&lines),
        _ => (0, 0),
    };
    println!("part 1: {}\npart 2: {}", result.0, result.1);
}
