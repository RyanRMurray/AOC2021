use crate::utils::{simple_parse, Answer};
use std::collections::VecDeque;

fn day_passes<'a>(mut population: VecDeque<usize>) -> VecDeque<usize> {
    //we get the population of day 0 fish, which would be the first in the list
    //we can then 'reset' them by adding them to day 6 fish, then appending their spawn
    //at the end of our queue
    let x = population.pop_front().unwrap();
    population[6] += x;
    population.push_back(x);

    population
}

pub fn day06(input: String) -> Answer {
    let mut answer = Answer::default();

    //parse input into populations by age
    let nums = simple_parse::<u8>(input, ",");
    let mut pops: VecDeque<usize> = (0..9)
        .map(|v| nums.iter().filter(|x| **x == v).count())
        .collect();

    answer.record_parsed();

    //part 1: proliferation for 80 days
    for _ in 0..80 {
        pops = day_passes(pops)
    }

    answer.record(&pops.iter().sum::<usize>());

    //part 2: further proliferation
    for _ in 0..(256 - 80) {
        pops = day_passes(pops)
    }

    answer.record(&pops.iter().sum::<usize>());

    return answer;
}
