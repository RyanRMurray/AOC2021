use crate::utils::Answer;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

type TCounts = HashMap<(char, char), usize>;
type Recipies = HashMap<(char, char), char>;

fn to_char<'a>(c: &'a str) -> char {
    c.chars().nth(0).unwrap()
}

fn step(tuples: TCounts, recipies: &Recipies) -> TCounts {
    let mut next: TCounts = HashMap::new();

    for ((a, b), n) in tuples.into_iter() {
        let c = recipies.get(&(a, b)).unwrap();
        *next.entry((a, *c)).or_insert(0) += n;
        *next.entry((*c, b)).or_insert(0) += n;
    }

    next
}

fn score(tuples: &TCounts, last: &char) -> usize {
    let mut c_counts: HashMap<char, usize> = HashMap::new();
    c_counts.insert(*last, 1);
    for ((a, _), n) in tuples {
        *c_counts.entry(*a).or_insert(0) += n;
    }
    c_counts.values().max().unwrap() - c_counts.values().min().unwrap()
}

pub fn day14(input: String) -> Answer {
    let mut answer = Answer::default();
    let re = Regex::new(r"(\w)(\w) -> (\w)").unwrap();

    //parse into starting tuple gangs and recipies
    let sp = input.split("\n\n").collect::<Vec<_>>();
    let mut tuples: TCounts = sp[0].chars().tuple_windows::<(char, char)>().counts();
    let last = sp[0].chars().last().unwrap(); //get the last for edge cases where the last char is the most/least common

    let recipies: HashMap<(char, char), char> = re
        .captures_iter(sp[1])
        .map(|c| ((to_char(&c[1]), to_char(&c[2])), to_char(&c[3])))
        .collect();

    //since constructing the whole string would take forever, we instead start with
    //a series of tuple counts, and then extrapolate what tuple counts they should create
    //based on our recipies.

    //part 1: 10 steps
    for _ in 0..10 {
        tuples = step(tuples, &recipies)
    }
    answer.record(&(score(&tuples, &last)));

    //part 2: 30 steps more
    for _ in 0..30 {
        tuples = step(tuples, &recipies)
    }
    answer.record(&(score(&tuples, &last)));

    answer.record_parsed();

    answer
}
