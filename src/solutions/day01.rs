use crate::utils::{simple_parse, Answer};

pub fn day01(input: String) -> Answer {
    let mut answer = Answer::default();

    //parse into list of ints
    let vals = simple_parse::<u32>(input, "\n");

    answer.record_parsed();

    //part 1: count all ascending steps
    let mut p1_asc = 0;
    let mut p1_last = vals[2];

    //part 2: count all ascending windows of size 3
    let mut p2_asc = 0;
    let mut p2_last: u32 = vals[0..3].iter().sum();

    //unroll first couple steps
    p1_asc += if vals[0] < vals[1] { 1 } else { 0 };
    p1_asc += if vals[1] < vals[2] { 1 } else { 0 };

    for w in vals[1..].windows(3) {
        let l = w.last().unwrap();
        if l > &p1_last {
            p1_asc += 1;
        }

        let s = w.iter().sum::<u32>();
        if s > p2_last {
            p2_asc += 1;
        }

        p1_last = *l;
        p2_last = s;
    }
    answer.record_both(&p1_asc, &p2_asc);

    return answer;
}
