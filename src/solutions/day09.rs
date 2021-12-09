use crate::utils::{Answer, Grid, Point, Pt2d};
use std::collections::HashMap;

fn compare_to_ns(g: &Grid<Pt2d, u8>, p: Pt2d, cmp: fn(u8, u8) -> bool) -> bool {
    let v = g.grid.get(&p).unwrap();
    let ns = p.neighbours_4();

    ns.iter()
        .filter_map(|n| g.grid.get(&n))
        .all(|nv| cmp(*v, *nv))
}

pub fn day09(input: String) -> Answer {
    let mut answer = Answer::default();

    //parse into 2d grid
    let m: HashMap<Pt2d, u8> = input
        .lines()
        .zip(0 as i32..)
        .map(|(l, y)| {
            l.chars()
                .zip(0 as i32..)
                .map(|(c, x)| ((x, y), c.to_digit(10).unwrap() as u8))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let g: Grid<Pt2d, u8> = Grid::new(m, (0, 0), 0);

    answer.record_parsed();

    //part 1: find all lowest points
    let p1: u32 = g
        .grid
        .keys()
        .filter(|k| compare_to_ns(&g, **k, |x, y| x < y))
        .map(|k| (g.grid.get(k).unwrap() + 1) as u32)
        .sum();

    answer.record(&p1);

    return answer;
}
