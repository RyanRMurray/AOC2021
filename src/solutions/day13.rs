use crate::utils::{Answer, Grid, Pt2d};
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone)]
enum I {
    Y(i32),
    X(i32),
}

fn fold(g: &mut Grid<Pt2d, bool>, instr: I) {
    let pts: Vec<Pt2d> = g
        .grid
        .keys()
        .filter(|(x, y)| match instr {
            I::Y(f) => y > &f,
            I::X(f) => x > &f,
        })
        .cloned()
        .collect();

    let reflected: Vec<Pt2d> = pts
        .iter()
        .map(|(x, y)| match instr {
            I::Y(f) => (*x, f - (y - f)),
            I::X(f) => (f - (x - f), *y),
        })
        .collect();

    g.updates(reflected, |_| true);

    for p in pts {
        g.grid.remove(&p);
    }
}

pub fn day13(input: String) -> Answer {
    let mut answer = Answer::default();
    let re_pt = Regex::new(r"(\d+),(\d+)").unwrap();
    let re_in = Regex::new(r"fold along (\w)=(\d+)").unwrap();

    //parse two instructions; our initial points, and the folds
    let sp = input.split("\n\n").collect::<Vec<_>>();
    let pts: HashMap<Pt2d, bool> = re_pt
        .captures_iter(sp[0])
        .map(|c| ((c[1].parse().unwrap(), c[2].parse().unwrap()), true))
        .collect();

    let mut g: Grid<Pt2d, bool> = Grid::new(pts, (0, 0), false);

    let instrs: Vec<I> = re_in
        .captures_iter(sp[1])
        .map(|c| match &c[1] {
            "x" => I::X(c[2].parse().unwrap()),
            _ => I::Y(c[2].parse().unwrap()),
        })
        .collect();

    answer.record_parsed();

    //part 1: perform one fold, count dots
    fold(&mut g, instrs[0].clone());

    answer.record(&g.grid.len());

    //part 2: perform all folds and retrieve resulting message

    for f in instrs[1..].into_iter().cloned() {
        fold(&mut g, f);
    }
    answer.record(&g.print_2d(|x| if x { '█' } else { ' ' }));

    answer
}
