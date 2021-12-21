use crate::utils::Answer;
use itertools::Itertools;
use std::convert::TryInto;

fn toggle(n: usize) -> usize {
    match n {
        0 => 1,
        _ => 0,
    }
}

fn to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into().unwrap_or_else(|_v: Vec<T>| panic!(":("))
}

fn play(
    mut positions: [usize; 2],
    mut scores: [i64; 2],
    player: usize,
    steps: usize,
) -> ([usize; 2], [i64; 2]) {
    let to = positions[player] + steps;
    if to > 10 {
        if to % 10 == 0 {
            positions[player] = 10;
        } else {
            positions[player] = to % 10;
        }
    } else {
        positions[player] = to;
    }

    scores[player] += positions[player] as i64;

    (positions, scores)
}

fn pw_add([a, b]: [usize; 2], [c, d]: [usize; 2]) -> [usize; 2] {
    [a + c, b + d]
}

fn pw_mul(c: usize, [a, b]: [usize; 2]) -> [usize; 2] {
    [a * c, b * c]
}

//returns the number of wins for each player in each sub-tree
fn quantum_game(
    roll_matrix: &Vec<(usize, usize)>,
    steps: (usize, usize),
    player: usize,
    mut positions: [usize; 2],
    mut scores: [i64; 2],
) -> [usize; 2] {
    let mut wins = [0, 0];

    (positions, scores) = play(positions, scores, player, steps.0);

    if scores[player] >= 21 {
        wins[player] += steps.1;
        return wins;
    }

    pw_mul(
        steps.1,
        roll_matrix.iter().fold(wins, |wincounts, next_steps| {
            pw_add(
                wincounts,
                quantum_game(roll_matrix, *next_steps, toggle(player), positions, scores),
            )
        }),
    )
}

pub fn day21(input: String) -> Answer {
    let mut answer = Answer::default();
    //parse inputs
    let positions: [usize; 2] = to_array(
        input
            .lines()
            .map(|l| l.chars().last().unwrap().to_digit(10).unwrap() as usize)
            .collect(),
    );

    //part 1: multiply loser's score by number of rolls on the deterministic dice
    let mut p1_positions = positions.clone();
    let mut p1_scores: [i64; 2] = [0, 0];
    let mut p1 = 0;
    let mut player = 0;
    let det_rolls = (1..101).cycle();
    let mut rolled = 0;

    for rs in det_rolls.chunks(3).into_iter() {
        let steps: Vec<usize> = rs.collect();
        rolled += 3;
        (p1_positions, p1_scores) =
            play(p1_positions, p1_scores, player, steps.iter().sum::<usize>());
        if p1_scores[player] >= 1000 {
            player = toggle(player);
            p1 = p1_scores[player] * rolled;
            break;
        }
        player = toggle(player);
    }

    answer.record(&p1);

    //part 2: Produce a tree of universes and count the winners wins
    let roll_matrix: Vec<(usize, usize)> = (1..4)
        .cartesian_product(1..4)
        .cartesian_product(1..4)
        .map(|((a, b), c)| a + b + c)
        .counts()
        .into_iter()
        .collect();

    let res = quantum_game(
        &roll_matrix,
        (0, 1),
        1,
        positions,
        [0, -(positions[1] as i64)],
    );

    answer.record(&usize::max(res[0], res[1]));

    answer
}
