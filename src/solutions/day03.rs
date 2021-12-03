use crate::utils::Answer;

fn bvec_to_u32(v: Vec<u32>) -> u32 {
    v.iter()
        .rev()
        .zip(0..v.len())
        .map(|(v, p)| v * u32::pow(2, p as u32))
        .sum()
}

pub fn day03(input: String) -> Answer {
    let mut answer = Answer::default();

    //parse into vec of lists of ints
    let nums: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(2).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let bit_len = nums[0].len() as u32;
    let half: u32 = nums.len() as u32 / 2;

    //part 1: get modal result by first summing everything and then constructing final vector
    let sums = nums.iter().fold(vec![0; bit_len as usize], |s, v| {
        s.iter().zip(v.iter()).map(|(&a, &b)| a + b).collect()
    });

    let (gamma, epsilon): (Vec<u32>, Vec<u32>) = sums
        .iter()
        .map(|v| if v >= &half { (1, 0) } else { (0, 1) })
        .unzip();

    answer.record(&(bvec_to_u32(gamma) * bvec_to_u32(epsilon)));

    return answer;
}
