use crate::utils::Answer;

type Disp<'a> = (Vec<&'a str>, Vec<&'a str>);

pub fn day08(input: String) -> Answer {
    let mut answer = Answer::default();

    //parse
    let patterns: Vec<Disp> = input
        .lines()
        .map(|l| {
            let x: Vec<&str> = l.split(" ").collect();
            (x[0..10].to_owned(), x[11..].to_owned())
        })
        .collect();

    answer.record_parsed();

    //part 1 identify and count (1,4,7,8)  occurrences in right side
    let mut acc = 0;

    for l in patterns.iter() {
        acc += (l.1)
            .iter()
            .filter(|e| e.len() != 5 && e.len() != 6)
            .count()
    }

    answer.record(&acc);

    //part 2: identify numbers by comparing against overlap with unique-sized numbers
    //1,7,4,8 are of known size, other numbers have subsections of them
    let mut sum: u32 = 0;

    for mut l in patterns {
        l.0.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());
        let mut res = "".to_owned();
        //since we know for certain where one and four will be in the list when we've sorted it by length,
        //we can use them as comparators for other numbers with similar segments
        let one: Vec<char> = l.0[0].chars().collect();
        let four: Vec<char> = l.0[2].chars().collect();

        for c in l.1 {
            res.push_str(
                match (
                    c.len(),
                    c.chars().filter(|x| four.contains(x)).count(), //compare against 4
                    c.chars().filter(|x| one.contains(x)).count(),  //compare against 1
                ) {
                    (6, 3, 1) => "6",
                    (6, 3, 2) => "0",
                    (6, 4, _) => "9",
                    (5, 3, 1) => "5",
                    (5, 3, 2) => "3",
                    (5, 2, _) => "2",
                    (2, _, _) => "1",
                    (3, _, _) => "7",
                    (4, _, _) => "4",
                    _ => "8",
                },
            )
        }
        sum += res.parse::<u32>().unwrap();
    }

    answer.record(&sum);

    return answer;
}
