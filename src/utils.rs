#![allow(dead_code)]
use std::fmt::Debug;
use std::fmt::{Display, Formatter, Result};
use std::str::FromStr;
use std::time::Instant;

//Types

pub type Solution = fn(String) -> Answer; // Solution functions

type Record = (String, Instant); // Recording an answer and its timestamp
type DisplayableRef<'a> = &'a dyn Display;

//Functions

pub fn simple_parse<T>(input: String) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input
        .lines()
        .map(|x| {
            x.parse()
                .expect("Error parsing input - are you sure it's a simple list?")
        })
        .collect::<Vec<T>>()
}

//Structs

pub struct Answer {
    start: Instant,
    part1: Option<Record>,
    part2: Option<Record>,
}

impl Default for Answer {
    fn default() -> Self {
        Self {
            start: Instant::now(),
            part1: None,
            part2: None,
        }
    }
}

impl Display for Answer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match (self.part1.as_ref(), self.part2.as_ref()) {
            (None, _) => write!(f, "Part 1 has not yet been implemented"),

            (Some((r, t)), None) => write!(
                f,
                "Part 1 Result: {}\n
            Part 1 Runtime: {:?}\n
            Part 2 has not yet been implemented.\n",
                r,
                t.saturating_duration_since(self.start)
            ),
            (Some((r1, t1)), Some((r2, t2))) => {
                if t1 != t2 {
                    write!(
                        f,
                        "Part 1 Result: {}\n\
                    Part 1 Runtime: {:?}\n\
                    Part 2 Result: {}\n\
                    Part 2 Runtime: {:?}\n\
                    Overall Runtime: {:?}\n",
                        r1,
                        t1.saturating_duration_since(self.start),
                        r2,
                        t2.saturating_duration_since(*t1),
                        t2.saturating_duration_since(self.start)
                    )
                } else {
                    write!(
                        f,
                        "Part 1 Result: {}\n\
                    Part 2 Result: {}\n\
                    Overall Runtime: {:?}\n",
                        r1,
                        r2,
                        t1.saturating_duration_since(self.start)
                    )
                }
            }
        }
    }
}

impl<'a> Answer {
    pub fn record(&mut self, res: DisplayableRef) {
        match self.part1 {
            None => self.part1 = Some((res.to_string(), Instant::now())),
            Some(_) => match self.part2 {
                None => self.part2 = Some((res.to_string(), Instant::now())),
                Some(_) => panic!("Cannot write third part to an answer!"),
            },
        }
    }

    pub fn record_both(&mut self, res1: DisplayableRef, res2: DisplayableRef) {
        if self.part1.is_some() {
            panic!("Cannot write both parts to partially recorded answer!")
        }

        let i = Instant::now();
        self.part1 = Some((res1.to_string(), i));
        self.part2 = Some((res2.to_string(), i));
    }
}
