use crate::utils::Answer;
use regex::Regex;

enum I {
    Up(u32),
    Down(u32),
    Forward(u32),
}

pub fn day02(input: String) -> Answer {
    let mut answer = Answer::default();

    //parse instructions
    let re = Regex::new(r"(\w+) (\d+)").unwrap();

    let instrs: Vec<I> = re
        .captures_iter(&input)
        .map(|m| {
            let n: u32 = m[2].parse().expect("Could not read number");
            match &m[1] {
                "forward" => I::Forward(n),
                "up" => I::Up(n),
                "down" => I::Down(n),
                _ => panic!("Could not read instruction"),
            }
        })
        .collect();

    //part 1: run instrs, multiply depth and distance
    let mut p1_dist = 0;
    let mut p1_depth = 0;
    //part 2: include aim mechanic, meaning up and down only alter depth by aim on a forward command
    let mut p2_dist = 0;
    let mut p2_depth = 0;
    let mut aim = 0;

    for i in instrs {
        match i {
            I::Forward(n) => {
                p1_dist += n;
                p2_dist += n;
                p2_depth += aim * n
            }
            I::Up(n) => {
                p1_depth -= n;
                aim -= n
            }
            I::Down(n) => {
                p1_depth += n;
                aim += n
            }
        }
    }

    answer.record_both(&(p1_dist * p1_depth), &(p2_dist * p2_depth));

    return answer;
}
