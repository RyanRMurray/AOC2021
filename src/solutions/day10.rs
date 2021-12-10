use crate::utils::Answer;
use std::collections::VecDeque;

const OPENS: [char; 4] = ['[', '<', '{', '('];
const MATCHES: [(char, char); 4] = [('[', ']'), ('<', '>'), ('(', ')'), ('{', '}')];

#[derive(Debug, Clone)]
enum B {
    Open(char),
    Close(char),
}

fn score_debugger(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Invalid symbol!"),
    }
}

fn score_autocomplete(mut chars: Vec<B>) -> u64 {
    chars.reverse();
    chars.iter().fold(0, |total, c| {
        (5 * total)
            + match c {
                B::Open('(') => 1,
                B::Open('[') => 2,
                B::Open('{') => 3,
                B::Open('<') => 4,
                _ => panic!("Invalid symbol!"),
            }
    })
}

fn validate_line(stack: &mut Vec<B>, next: B) -> Option<u32> {
    match (stack.pop(), next) {
        //open braces
        (None, B::Open(y)) => {
            stack.push(B::Open(y));
            None
        }
        (Some(x), B::Open(y)) => {
            stack.extend([x, B::Open(y)]);
            None
        }
        //closer with no opener
        (None, B::Close(y)) => Some(score_debugger(y)),
        //closer with opener
        (Some(B::Open(x)), B::Close(y)) => {
            if MATCHES.contains(&(x, y)) {
                None
            } else {
                Some(score_debugger(y))
            }
        }
        _ => panic!("Invalid state!"),
    }
}

pub fn day10(input: String) -> Answer {
    let mut answer = Answer::default();

    //parse into character vectors
    let lines: Vec<VecDeque<B>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    if OPENS.contains(&c) {
                        B::Open(c)
                    } else {
                        B::Close(c)
                    }
                })
                .collect()
        })
        .collect();

    answer.record_parsed();

    //part 1: record and score the first corrupt char on each line (retain valid queues for part 2)
    let mut score1: u32 = 0;
    let mut retained: Vec<Vec<B>> = vec![];
    for l in lines {
        let mut stack: Vec<B> = vec![];
        let mut res: Option<u32> = None;
        let mut queue = l.clone();

        while queue.len() > 0 && res.is_none() {
            res = validate_line(&mut stack, queue.pop_front().unwrap());
        }

        match res {
            Some(v) => score1 += v,
            None => retained.push(stack),
        }
    }

    answer.record(&score1);

    //part 2: get autocomplete score, record middle-most value
    let mid: usize = retained.len() / 2;
    let mut scores: Vec<_> = retained
        .into_iter()
        .map(|r| score_autocomplete(r))
        .collect();

    scores.sort();

    answer.record(&scores[mid]);

    answer
}
