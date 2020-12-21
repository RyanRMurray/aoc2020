use std::env;
use std::io;
use std::fs;
use std::time::{Instant};
mod utils;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;

type Solution = fn(String) -> (String,String);

fn main() {
    let solutions: Vec<Solution> = 
        vec![
            day01::day01, day02::day02, day03::day03, day04::day04,
            day05::day05, day06::day06, day07::day07, day08::day08,
            day09::day09, day10::day10, day11::day11, day12::day12,
            day13::day13, day14::day14, day15::day15, day16::day16,
            day17::day17, day18::day18, day19::day19, day20::day20,
            day21::day21
        ];
    let args: Vec<String> = env::args().collect();
    let mut day_arg = String::new();
    let input_arg;

    //get day number
    match args.get(1){
        None => {
            println!("Enter Day Number: "); 
            io::stdin()
                .read_line(&mut day_arg)
                .expect("Failed to read line.");
        }
        Some(a) => day_arg = a.to_string()
    }

    let day: usize = day_arg.trim().parse()
        .expect("Day number invalid.");

    //get input file
    match args.get(2){
        Some(a) => input_arg = a.to_string(),
        None    => input_arg = "./input.txt".to_string()
    }

    let input = fs::read_to_string(input_arg)
        .expect("Could not open input file.");

    let curr = Instant::now();

    //get the solution
    match solutions.get(day-1){
        None => println!("Invalid day, or I've not written a solution for that one yet!"),
        Some(solve) => {
            let (p1,p2) = solve(input);
            println!("Solved in {:.}ms", (curr.elapsed().as_nanos() as f64 )/1000000.0);
            println!("Part 1: {}\nPart 2: {}", p1, p2)
        }
    }
}
