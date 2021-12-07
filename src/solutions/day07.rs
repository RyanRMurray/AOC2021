use crate::utils::{simple_parse, Answer};

pub fn day07(input: String) -> Answer {
    let mut answer = Answer::default();

    //parse
    let mut locs = simple_parse::<i32>(input, ",");
    locs.sort();

    answer.record_parsed();

    //part 1: minimum modification needed to align all elements
    let crab_max = locs.last().unwrap();
    let crab_min = locs[0];

    let mut min_fuel = std::i32::MAX;

    for mid in crab_min..*crab_max {
        min_fuel = i32::min(min_fuel, locs.iter().map(|x| i32::abs(x - mid)).sum());
    }

    answer.record(&min_fuel);

    //part 2: minimum when distance accumulates more fuel usage

    min_fuel = std::i32::MAX;

    for mid in crab_min..*crab_max {
        min_fuel = i32::min(
            min_fuel,
            locs.iter()
                .map(|x| {
                    let n = i32::abs(x - mid);
                    n * (n + 1) / 2
                })
                .sum(),
        );
    }

    answer.record(&min_fuel);

    return answer;
}
