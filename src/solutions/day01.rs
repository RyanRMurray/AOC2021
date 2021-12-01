use crate::utils::{Answer,simple_parse};

pub fn day01(input: String) -> Answer {
    let mut answer = Answer::default();

    //parse
    let vals = simple_parse::<u32>(input);

    //part 1: find number of ascending steps
    let mut ascending = 0;
    let mut last = vals[0];

    for d in vals[1..].iter(){
        if d > &last{
            ascending += 1;
        }
        last = *d;
    }
    answer.record(&ascending);

    //part 2: find ascending steps using 3-length window
    let mut ascending_window = 0;
    let mut last_sum : u32 = vals[0..3].iter().sum();
    
    for i in 2..vals.len(){
        let s: u32 = vals[i-2..i+1].iter().sum();
        if s > last_sum{
            ascending_window += 1;
        }
        last_sum = s;
    }
    answer.record(&ascending_window);

    return answer;
}
