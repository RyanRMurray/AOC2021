use crate::utils::{Answer, Grid, Point, Pt2d};

fn find_bursting(g: &Grid<Pt2d, u8>) -> Vec<Pt2d> {
    g.grid
        .iter()
        .filter(|(_, v)| **v > 9)
        .map(|(k, _)| *k)
        .collect()
}

//step grid and return bursting points
fn step_grid(g: &mut Grid<Pt2d, u8>) {
    g.updates(g.grid.keys().cloned().collect(), |x| x + 1);
}

//perform bursts by setting burst points to 0 and incrementing neighbours
fn do_bursts(g: &mut Grid<Pt2d, u8>, bursting: Vec<Pt2d>) {
    let ns: Vec<Pt2d> = bursting
        .iter()
        .map(|b| b.neighbours_all())
        .flatten()
        .filter(|n| g.grid.contains_key(n))
        .collect();

    g.updates(bursting, |_| 0);

    g.updates(ns, |x| x + 1)
}

//returns (burst_num,burst_history length)
fn do_step(g: &mut Grid<Pt2d, u8>) -> (usize, usize) {
    step_grid(g);

    let mut burst_history = vec![];
    let mut bursts = 0;

    loop {
        let bursting = find_bursting(&g);
        if bursting.len() == 0 {
            break;
        }

        bursts += bursting.len();
        burst_history.extend(bursting.clone());

        do_bursts(g, bursting);
    }

    g.updates(burst_history.clone(), |_| 0);

    (bursts, burst_history.len())
}

pub fn day11(input: String) -> Answer {
    let mut answer = Answer::default();

    //parse into grid
    let mut g = Grid::from(input, 9, |c| c.to_digit(10).unwrap() as u8);

    answer.record_parsed();

    //part 1 100 iterations
    let mut bursts = 0;
    for _ in 0..100 {
        let (b, _) = do_step(&mut g);

        bursts += b;
    }

    answer.record(&bursts);

    //part 2: find step where everything bursts at once
    for s in 101.. {
        let (_, h) = do_step(&mut g);

        if h == 100 {
            answer.record(&s);
            break;
        }
    }

    answer
}
