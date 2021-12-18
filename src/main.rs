#![feature(map_first_last)]
use std::env;
use std::fs;
use std::io;
mod solutions;
mod utils;
extern crate eval;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut day_arg = String::new();
    let input_arg;

    //get day number if none provided
    match args.get(1) {
        None => {
            println!("Enter day number: ");
            io::stdin()
                .read_line(&mut day_arg)
                .expect("Failed to read day number.");
        }
        Some(a) => day_arg = a.to_string(),
    }

    let day: usize = day_arg.trim().parse().expect("Invalid day number format.");

    if (day < 1) || (day > 25) {
        println!("Not a valid day >:[");
        return;
    }

    //get input file
    match args.get(2) {
        Some(a) => input_arg = a.to_string(),
        None => input_arg = "./input.txt".to_string(),
    }

    let input = fs::read_to_string(input_arg).expect("Could not open input file.");

    //get solution if one exists
    match solutions::DAYS.get(day - 1) {
        None => println!("No solution for that day yet."),
        Some(sol) => {
            let a = sol(input);
            println!("===========Day {}===========\n{}", day, a)
        }
    }
}
