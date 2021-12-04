use crate::utils::Answer;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Bingo {
    row_counts: [u8; 5],
    col_counts: [u8; 5],
    num_positions: HashMap<u8, (u8, u8)>,
    unmarked: HashSet<u8>,
}

impl Bingo {
    pub fn new(m: HashMap<u8, (u8, u8)>) -> Self {
        Bingo {
            row_counts: [0, 0, 0, 0, 0],
            col_counts: [0, 0, 0, 0, 0],
            unmarked: m.keys().cloned().collect(),
            num_positions: m,
        }
    }

    fn check(&self) -> Option<u32> {
        if self.row_counts.contains(&5) || self.col_counts.contains(&5) {
            Some(self.unmarked.iter().map(|v| *v as u32).sum())
        } else {
            None
        }
    }

    fn mark(&mut self, to_mark: u8) {
        match self.num_positions.get(&to_mark) {
            Some((x, y)) => {
                self.unmarked.remove(&to_mark);
                self.row_counts[*y as usize] += 1;
                self.col_counts[*x as usize] += 1;
            }
            None => {}
        }
    }
}

pub fn day04(input: String) -> Answer {
    let mut answer = Answer::default();
    let n_parse = Regex::new(r"(\d+)").unwrap();

    //parse first line as sequence of calls, rest as bingo cards
    let s_chunks: Vec<&str> = input.split("\n\n").collect();

    let sequence: Vec<u8> = s_chunks[0].split(",").map(|c| c.parse().unwrap()).collect();

    let mut sheets: Vec<Bingo> = s_chunks[1..]
        .into_iter()
        .map(|g| {
            g
                //get each line of a grid
                .split("\n")
                //assigning each a y positon
                .zip(0..)
                .map(|(l, y)| {
                    //parse out each integer sequence
                    n_parse
                        .captures_iter(l)
                        //assigning each an x position
                        .zip(0..)
                        //parse and format for collection into a HashMap
                        .map(|(v, x)| (v[1].parse::<u8>().unwrap(), (x, y)))
                        .collect::<Vec<(u8, (u8, u8))>>()
                })
                .flatten()
                .collect()
        })
        .map(|m| Bingo::new(m))
        .collect();

    answer.record_parsed();

    //part 1: get the sum of the unmarked spaces of the winning sheet times the last number
    //part 2: get last score
    let mut scores: Vec<u32> = vec![];

    for n in sequence {
        sheets.iter_mut().for_each(|s| s.mark(n));

        sheets.retain(|s| match (s.check(), scores.as_slice()) {
            (Some(v), []) => {
                scores.push(v * n as u32);
                answer.record(&(v * n as u32));
                false
            }
            (Some(v), _) => {
                scores.push(v * n as u32);
                false
            }
            _ => true,
        });
    }

    answer.record(scores.last().unwrap());

    return answer;
}
