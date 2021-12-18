use crate::eval::{eval, Value};
use crate::utils::Answer;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone)]
enum Shnumber {
    Single(u32),
    Dub(Box<Shnumber>, Box<Shnumber>),
}

//parsing
fn to_shnumber(arr: Value) -> Shnumber {
    match arr {
        Value::Array(xs) => Shnumber::Dub(
            Box::new(to_shnumber(xs[0].clone())),
            Box::new(to_shnumber(xs[1].clone())),
        ),
        Value::Number(x) => Shnumber::Single(x.as_u64().unwrap() as u32),
        _ => panic!("Invalid input!"),
    }
}

fn shparse(input: &str) -> Shnumber {
    let op = Regex::new(r"\[").unwrap();
    let cl = Regex::new(r"\]").unwrap();

    let x = op.replace_all(input, "array(");
    let y = cl.replace_all(&x, ")");

    let res = eval(&y).unwrap();

    to_shnumber(res)
}

//reduction
fn to_dub(a: u32, b: u32) -> Shnumber {
    Shnumber::Dub(Box::new(Shnumber::Single(a)), Box::new(Shnumber::Single(b)))
}

fn a_l(a: Shnumber, b: Option<Shnumber>) -> Shnumber {
    match (a, b) {
        (v, None) => v,
        (Shnumber::Single(v1), Some(Shnumber::Single(v2))) => Shnumber::Single(v1 + v2),
        (Shnumber::Dub(v1, v2), v3 @ Some(Shnumber::Single(_))) => {
            Shnumber::Dub(Box::new(a_l(*v1, v3)), v2)
        }
        _ => panic!("Invalid addition!"),
    }
}

fn a_r(a: Shnumber, b: Option<Shnumber>) -> Shnumber {
    match (a, b) {
        (v, None) => v,
        (Shnumber::Single(v1), Some(Shnumber::Single(v2))) => Shnumber::Single(v1 + v2),
        (Shnumber::Dub(v1, v2), v3 @ Some(_)) => Shnumber::Dub(v1, Box::new(a_r(*v2, v3))),
        _ => panic!("Invalid addition!"),
    }
}

fn explode(s: Shnumber, n: u32) -> (bool, Option<Shnumber>, Shnumber, Option<Shnumber>) {
    match (s, n) {
        (v @ Shnumber::Single(_), _) => return (false, None, v, None),
        (Shnumber::Dub(v1, v2), 0) => return (true, Some(*v1), Shnumber::Single(0), Some(*v2)),
        (Shnumber::Dub(v1, v2), m) => {
            let (c1, l, v3, r) = explode(*v1.clone(), m - 1);
            if c1 {
                return (
                    true,
                    l,
                    Shnumber::Dub(Box::new(v3), Box::new(a_l(*v2, r))),
                    None,
                );
            } else {
                let (c2, l, v4, r) = explode(*v2.clone(), m - 1);
                if c2 {
                    return (
                        true,
                        None,
                        Shnumber::Dub(Box::new(a_r(v3, l)), Box::new(v4)),
                        r,
                    );
                } else {
                    (false, None, Shnumber::Dub(v1, v2), None)
                }
            }
        }
    }
}

fn split(s: Shnumber) -> (bool, Shnumber) {
    match s {
        Shnumber::Single(v) => {
            if v >= 10 {
                (true, to_dub(v / 2, (v + 1) / 2))
            } else {
                (false, Shnumber::Single(v))
            }
        }
        Shnumber::Dub(v1, v2) => {
            let (c1, v3) = split(*v1.clone());
            if c1 {
                (true, Shnumber::Dub(Box::new(v3), v2))
            } else {
                let (c2, v4) = split(*v2);
                (c2, Shnumber::Dub(Box::new(v3), Box::new(v4)))
            }
        }
    }
}

fn shadd(a: Shnumber, b: Shnumber) -> Shnumber {
    let mut res = Shnumber::Dub(Box::new(a), Box::new(b));

    loop {
        let (c1, _, s, _) = explode(res, 4);
        res = s;
        if c1 {
            continue;
        }
        let (c2, s) = split(res);
        res = s;
        if !c2 {
            break;
        }
    }

    res
}

//operations
fn shmagnitude(s: Shnumber) -> u32 {
    match s {
        Shnumber::Single(v) => v,
        Shnumber::Dub(a, b) => 3 * shmagnitude(*a) + 2 * shmagnitude(*b),
    }
}

pub fn day18(input: String) -> Answer {
    let mut answer = Answer::default();

    //parse shnumbers
    let shnumbs: Vec<Shnumber> = input.lines().map(|l| shparse(l)).collect();
    answer.record_parsed();

    //part 1: add-reduce all numbers
    let p1 = shnumbs
        .clone()
        .into_iter()
        .reduce(|a, b| shadd(a, b))
        .unwrap();

    answer.record(&shmagnitude(p1));

    //part 2: find highest magnitude of two additions
    let l = shnumbs.len();
    let mut max = 0;

    for (x, y) in (0..l).cartesian_product(0..l) {
        if x == y {
            continue;
        }
        max = u32::max(
            max,
            shmagnitude(shadd(shnumbs[x].clone(), shnumbs[y].clone())),
        )
    }

    answer.record(&max);

    answer
}
