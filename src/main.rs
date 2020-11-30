use std::env;
use std::io;
use std::fs;
mod day01;

type Solution = fn(String) -> ();

fn main() {
    let solutions: Vec<Solution> = vec![day01::day01];
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

    //get the solution
    match solutions.get(day-1){
        None => println!("Invalid day, or I've not written a solution for that one yet!"),
        Some(solve) => solve(input)
    }
}