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

    let g: Grid<Pt2d, u8> = Grid::new(m, (0, 0), 9);

    answer.record_parsed();

    //part 1: find all lowest points, summing their depths +1
    //we retain the lowest points for use in part 2
    let lowests: Vec<Pt2d> = g
        .grid
        .keys()
        .filter(|k| compare_to_ns(&g, **k, |x, y| x < y))
        .cloned()
        .collect();

    let p1: u32 = lowests
        .iter()
        .map(|k| (g.grid.get(k).unwrap() + 1) as u32)
        .sum();

    answer.record(&p1);

    //part 2: multiply the size of the three largest basins
    //very lenient puzzle, since we are guaranteed each lowest point belonging to one solitary basin with no overlap!
    let mut basin_sizes: Vec<usize> = lowests
        .iter()
        .map(|l| g.flood_find(*l, |x| x < 9).len())
        .collect();
    //sort in descending order
    basin_sizes.sort();
    basin_sizes.reverse();

    let p2: usize = basin_sizes[0..3].iter().product();

    answer.record(&p2);

    return answer;
}
