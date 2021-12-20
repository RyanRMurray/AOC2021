use itertools::Itertools;

use crate::utils::{bit_to_n, Answer, Grid, Point, Pt2d};

type State = Grid<Pt2d, bool>;

fn enhance(enh: &Vec<bool>, state: &State, p: Pt2d) -> (Pt2d, bool) {
    let pattern: Vec<usize> = p
        .neighbourhood()
        .iter()
        .map(|pn| if state.get_def(pn) { 1 } else { 0 })
        .collect();

    (p, enh[bit_to_n(&pattern)])
}

fn enhance_all(enh: &Vec<bool>, state: &mut State) {
    let (x_min, y_min, x_max, y_max) = state.bounds();

    //update the image and its immediate border
    let updates: Vec<_> = (x_min - 1..x_max + 2)
        .cartesian_product(y_min - 1..y_max + 2)
        .map(|p| enhance(enh, &state, p))
        .collect();

    state.inserts(updates);

    //update the world state, finding what everything else in the world changes to
    let blanks: Vec<usize> = [state.default; 9].iter().map(|b| *b as usize).collect();
    state.default = enh[bit_to_n(&blanks)];
}

fn count_alight(state: &State) -> usize {
    state.grid.values().filter(|v| **v).count()
}

pub fn day20(input: String) -> Answer {
    let mut answer = Answer::default();

    //parse into enhancement seq and starting image
    let ins: Vec<&str> = input.split("\n\n").collect();

    let enh: Vec<bool> = ins[0].chars().map(|c| c == '#').collect();

    let mut state = Grid::from(ins[1].to_string(), false, |c| c == '#');

    answer.record_parsed();

    //perform two passes
    for _ in 0..2 {
        enhance_all(&enh, &mut state);
    }
    answer.record(&count_alight(&state));

    //perform MOAR passes
    for _ in 0..48 {
        enhance_all(&enh, &mut state);
    }
    answer.record(&count_alight(&state));

    answer
}
