#![allow(dead_code)]
use std::cmp::Eq;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::{Display, Formatter, Result};
use std::hash::Hash;
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
    parsed: Option<Instant>,
    part1: Option<String>,
    part2: Option<String>,
    time1: Option<Instant>,
    time2: Option<Instant>,
}

impl Default for Answer {
    fn default() -> Self {
        Self {
            start: Instant::now(),
            parsed: None,
            part1: None,
            part2: None,
            time1: None,
            time2: None,
        }
    }
}

impl Display for Answer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        //parsing time
        let (parsed, t_last) = match self.parsed {
            None => (String::from("Parsing not yet implemented\n"), self.start),
            Some(t) => (
                format!(
                    "Parsed input in {:?}\n",
                    t.saturating_duration_since(self.start)
                ),
                t,
            ),
        };

        //parts
        let p1 = match &self.part1 {
            None => String::from("Part 1 not yet implemented.\n"),
            Some(r) => format!("Part 1 Result: {}\n", r),
        };
        let p2 = match &self.part2 {
            None => String::from("Part 2 not yet implemented.\n"),
            Some(r) => format!("Part 2 Result: {}\n", r),
        };

        //times
        let ts = match (self.time1, self.time2) {
            (Some(t), None) => format!(
                "Part 1 Runtime: {:?}\n",
                t.saturating_duration_since(t_last)
            ),
            (Some(t1), Some(t2)) => {
                if t1 == t2 {
                    format!(
                        "Part 1 & 2 Runtime: {:?}\n",
                        t1.saturating_duration_since(t_last)
                    )
                } else {
                    format!(
                        "Part 1 Runtime: {:?}\nPart 2 Runtime: {:?}\n",
                        t1.saturating_duration_since(t_last),
                        t2.saturating_duration_since(t1)
                    )
                }
            }
            _ => String::from(""),
        };

        //overall time
        let overall = format!(
            "Overall runtime: {:?}",
            self.time2
                .or(self.time1)
                .or(Some(t_last))
                .unwrap()
                .saturating_duration_since(self.start)
        );

        write!(f, "{}{}{}{}{}", p1, p2, parsed, ts, overall)
    }
}

impl<'a> Answer {
    pub fn record_parsed(&mut self) {
        self.parsed = Some(Instant::now());
    }

    pub fn record(&mut self, res: DisplayableRef) {
        match self.part1 {
            None => {
                self.part1 = Some(res.to_string());
                self.time1 = Some(Instant::now());
            }
            Some(_) => match self.part2 {
                None => {
                    self.part2 = Some(res.to_string());
                    self.time2 = Some(Instant::now());
                }
                Some(_) => panic!("Cannot write third part to an answer!"),
            },
        }
    }

    pub fn record_both(&mut self, res1: DisplayableRef, res2: DisplayableRef) {
        if self.part1.is_some() {
            panic!("Cannot write both parts to partially recorded answer!")
        }

        let i = Instant::now();
        self.part1 = Some(res1.to_string());
        self.part2 = Some(res2.to_string());
        self.time1 = Some(i);
        self.time2 = Some(i);
    }
}

pub trait Point<Rhs = Self> {
    fn add(self, other: Rhs) -> Self;
    fn mul(self, v: i32) -> Self;
    fn rot90cw(self) -> Self;
    fn rot90acw(self) -> Self;
    fn mag(self) -> i32;
    fn neighbours_4(&self) -> Vec<Rhs>;
}

pub type Pt2d = (i32, i32);

impl Point for Pt2d {
    fn add(self, (ox, oy): Pt2d) -> Self {
        (self.0 + ox, self.1 + oy)
    }

    fn mul(self, v: i32) -> Self {
        (self.0 * v, self.1 * v)
    }

    fn rot90cw(self) -> Self {
        (-self.1, self.0)
    }

    fn rot90acw(self) -> Self {
        (self.1, -self.0)
    }

    fn mag(self) -> i32 {
        self.0.abs() + self.1.abs()
    }

    fn neighbours_4(&self) -> Vec<Pt2d> {
        [(0, -1), (1, 0), (0, 1), (-1, 0)]
            .iter()
            .map(|n| self.add(*n))
            .collect()
    }
}

pub struct Grid<K: Point, V: Default + PartialEq> {
    pub grid: HashMap<K, V>,
    default: V,
    pub ptr: K,
}

impl<K: Point + Eq + Hash + Copy, V: Default + PartialEq + Copy> Grid<K, V> {
    pub fn new(ptr: K, def: V) -> Self {
        Self {
            grid: HashMap::new(),
            default: def,
            ptr: ptr,
        }
    }

    pub fn update(&mut self, k: K, u: fn(V) -> V) {
        self.grid
            .insert(k, u(*self.grid.get(&k).unwrap_or(&self.default)));
    }

    pub fn updates(&mut self, ks: Vec<K>, u: fn(V) -> V) {
        for k in ks {
            self.update(k, u)
        }
    }
}
