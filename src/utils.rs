use std::cmp::Eq;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::fmt::{Display, Formatter, Result};
use std::hash::Hash;
use std::str::FromStr;
use std::time::Instant;

use itertools::Itertools;

//Types

pub type Solution = fn(String) -> Answer; // Solution functions

type DisplayableRef<'a> = &'a dyn Display; // Shorthand for Answer struct stuff

//Functions

pub fn simple_parse<T>(input: String, separator: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input
        .split(separator)
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
    fn add(self, other: &Rhs) -> Self;
    fn sub(self, other: &Rhs) -> Self;
    fn mul(self, v: i32) -> Self;
    fn mag(self) -> i32;
    fn neighbours_card(&self) -> Vec<Rhs>;
    fn neighbours_all(&self) -> Vec<Rhs>;
}

pub trait GridKey {}
pub trait GridVal {}

pub type Pt2d = (i32, i32);

impl Point for Pt2d {
    fn add(self, (ox, oy): &Pt2d) -> Self {
        (self.0 + ox, self.1 + oy)
    }

    fn sub(self, (ox, oy): &Pt2d) -> Self {
        (self.0 - ox, self.1 - oy)
    }

    fn mul(self, v: i32) -> Self {
        (self.0 * v, self.1 * v)
    }
    fn mag(self) -> i32 {
        self.0.abs() + self.1.abs()
    }

    fn neighbours_card(&self) -> Vec<Pt2d> {
        [(0, -1), (1, 0), (0, 1), (-1, 0)]
            .iter()
            .map(|n| self.add(n))
            .collect()
    }

    fn neighbours_all(&self) -> Vec<Pt2d> {
        [
            (0, -1),
            (1, 0),
            (0, 1),
            (-1, 0),
            (-1, -1),
            (1, -1),
            (-1, 1),
            (1, 1),
        ]
        .iter()
        .map(|n| self.add(n))
        .collect()
    }
}

pub type Pt3d = (i32, i32, i32);

impl Point for Pt3d {
    fn add(self, (ox, oy, oz): &Pt3d) -> Self {
        (self.0 + ox, self.1 + oy, self.2 + oz)
    }

    fn sub(self, (ox, oy, oz): &Pt3d) -> Self {
        (self.0 - ox, self.1 - oy, self.2 - oz)
    }

    fn mul(self, v: i32) -> Self {
        (self.0 * v, self.1 * v, self.2 * v)
    }

    fn mag(self) -> i32 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }

    fn neighbours_card(&self) -> Vec<Self> {
        [
            (0, 0, 1),
            (0, 0, -1),
            (0, 1, 0),
            (0, -1, 0),
            (1, 0, 0),
            (-1, 0, 0),
        ]
        .iter()
        .map(|o| self.add(o))
        .collect()
    }

    fn neighbours_all(&self) -> Vec<Self> {
        (-1..2)
            .permutations(3)
            .filter(|n| n.iter().filter(|e| **e == 0).count() != 3)
            .map(|o| self.add(&(o[0], o[1], o[2])))
            .collect()
    }
}

pub struct Grid<K: Point, V> {
    pub grid: HashMap<K, V>,
    default: V,
    pub ptr: K,
}

impl<K: Point + Eq + Hash + Copy, V: PartialEq + Copy> Grid<K, V> {
    pub fn new(g: HashMap<K, V>, ptr: K, def: V) -> Self {
        Self {
            grid: g,
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

    //get value or default
    pub fn get_def(&self, p: &K) -> V {
        *self.grid.get(p).unwrap_or(&self.default)
    }

    //finds all points reachable from p in grid under some criteria
    pub fn flood_find(&self, p: K, limiter: fn(V) -> bool) -> HashSet<K> {
        let mut found = HashSet::new();
        found.insert(p);
        let mut search: VecDeque<K> = VecDeque::from([p]);

        while search.len() > 0 {
            let around = search.pop_front().unwrap();

            let ns: HashSet<K> = around
                .neighbours_card()
                .iter()
                .filter(|q| !found.contains(q) && limiter(self.get_def(q)))
                .cloned()
                .collect();

            found.extend(ns.clone());
            search.extend(ns);
        }

        return found;
    }
}

impl<V: PartialEq + Copy> Grid<Pt2d, V> {
    pub fn from(input: String, def: V, p: fn(char) -> V) -> Self {
        let g: HashMap<Pt2d, V> = input
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| ((x as i32, y as i32), p(c)))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();

        Grid::new(g, (0, 0), def)
    }

    pub fn bounds(&self) -> (i32, i32, i32, i32) {
        self.grid.keys().fold((0, 0, 0, 0), |bounds, pt| {
            (
                i32::min(pt.0, bounds.0),
                i32::min(pt.1, bounds.1),
                i32::max(pt.0, bounds.2),
                i32::max(pt.1, bounds.3),
            )
        })
    }

    pub fn print_2d<'a>(&self, pfunc: fn(V) -> char) -> String {
        let mut res = String::from("\n");
        let (min_x, min_y, max_x, max_y) = self.bounds();

        for y in min_y..max_y + 1 {
            for x in min_x..max_x + 1 {
                res.push(pfunc(self.get_def(&(x, y))));
            }
            res.push('\n')
        }
        res
    }
}
