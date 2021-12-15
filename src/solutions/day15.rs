use crate::utils::{Answer, Grid, Point, Pt2d};
use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, HashSet};

fn in_bounds(b: i32, (x, y): &Pt2d) -> bool {
    (*x >= 0) && (*x <= b) && (*y >= 0) && (*y <= b)
}

fn find(g: &Grid<Pt2d, usize>, max: i32) -> usize {
    let mut distances: BTreeSet<(usize, Pt2d)> = BTreeSet::from([(0, (0, 0))]);
    let mut visited: HashSet<Pt2d> = HashSet::new();

    while distances.first().unwrap().1 != (max, max) {
        //visit closest
        let (v, c) = distances.pop_first().unwrap();
        visited.insert(c);

        //find distance to neighbours
        let ns = c
            .neighbours_4()
            .into_iter()
            .filter(|n| in_bounds(max, n) && !visited.contains(n))
            .map(|n| (v + g.get_def(&n), n));

        //append neighbours to distances btreeset
        distances.extend(ns);
    }
    distances.pop_first().unwrap().0
}

fn meta_val(v: usize, x: usize, y: usize) -> usize {
    let new_v = v + x + y;

    if new_v > 9 {
        new_v % 9
    } else {
        new_v
    }
}

pub fn day15(input: String) -> Answer {
    let mut answer = Answer::default();

    //parse
    let size = f64::sqrt(input.len() as f64) as i32;
    let g = Grid::from(input, 10, |c| c.to_digit(10).unwrap() as usize);

    answer.record_parsed();

    //part 1: find cheapest route to bottom-right
    let p1 = find(&g, size - 1);
    answer.record(&p1);

    //part 2: inflate grid and find cheapest route again
    let m: HashMap<Pt2d, usize> = g
        .grid
        .iter()
        .map(|((x, y), v)| {
            (0..5)
                .cartesian_product(0..5)
                .map(|(meta_x, meta_y)| {
                    (
                        (x + (size * meta_x), y + (size * meta_y)),
                        meta_val(*v, meta_x as usize, meta_y as usize),
                    )
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();
    let g2 = Grid::new(m, (0, 0), 10);

    let p2 = find(&g2, (size * 5) - 1);
    answer.record(&p2);

    answer
}
