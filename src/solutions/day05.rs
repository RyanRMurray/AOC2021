use crate::utils::{Answer, Grid, Pt2d};
use regex::Regex;
use std::cmp::max;

fn diff(a: i32, b: i32) -> i32 {
    if a < b {
        1
    } else if a == b {
        0
    } else {
        -1
    }
}

#[derive(Debug)]
enum Line {
    Diag(Vec<Pt2d>),
    Oth(Vec<Pt2d>),
}

pub fn day05(input: String) -> Answer {
    let mut answer = Answer::default();

    //parse coordinates into a series of points
    //and organise by diagonal and nondiagonal
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

    let lines: Vec<Line> = input
        .lines()
        .map(|l| re.captures(l).unwrap())
        .map(|c| {
            let x1: i32 = c[1].parse().unwrap();
            let y1: i32 = c[2].parse().unwrap();
            let x2: i32 = c[3].parse().unwrap();
            let y2: i32 = c[4].parse().unwrap();
            let dx = diff(x1, x2);
            let dy = diff(y1, y2);
            let mut pt = (x1, y1);
            let mut pts = vec![];

            for _ in 0..max(i32::abs(x1 - x2), i32::abs(y1 - y2)) + 1 {
                pts.push(pt);
                pt = (pt.0 + dx, pt.1 + dy);
            }

            if (x1 == x2) || (y1 == y2) {
                Line::Oth(pts)
            } else {
                Line::Diag(pts)
            }
        })
        .collect();

    answer.record_parsed();

    //part 1: find overlapping straight line points
    let mut g = Grid::new((0, 0), 0);

    for l in &lines {
        match l {
            Line::Diag(_) => {}
            Line::Oth(pts) => g.updates(pts.to_vec(), |v| v + 1),
        }
    }

    let p1 = g.grid.values().filter(|v| **v > 1).count();
    answer.record(&p1);

    //part 2: find overlapping for all
    for l in lines {
        match l {
            Line::Oth(_) => {}
            Line::Diag(pts) => g.updates(pts, |v| v + 1),
        }
    }

    let p2 = g.grid.values().filter(|v| **v > 1).count();
    answer.record(&p2);

    return answer;
}
