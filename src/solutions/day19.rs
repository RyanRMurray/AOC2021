use crate::utils::{Answer, Point, Pt3d};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;

fn ind(p: &Pt3d, n: i32) -> i32 {
    match n {
        0 => p.0,
        1 => p.1,
        2 => p.2,
        _ => panic!("invalid index for 3-tuple"),
    }
}

fn transform(h: &HashSet<Pt3d>, p: &Pt3d) -> HashSet<Pt3d> {
    h.iter().map(|x| x.add(p)).collect()
}

fn axis_mutations(pts: &HashSet<Pt3d>) -> Vec<HashSet<Pt3d>> {
    let orgs = (0..3).permutations(3).collect::<Vec<_>>();
    let mirrors = [1, 1, 1, -1, -1, -1]
        .into_iter()
        .permutations(3)
        .collect::<HashSet<_>>();

    orgs.iter()
        .cartesian_product(mirrors.iter())
        .map(|(o, m)| {
            pts.iter()
                .map(|p| {
                    (
                        ind(p, o[0]) * m[0],
                        ind(p, o[1]) * m[1],
                        ind(p, o[2]) * m[2],
                    )
                })
                .collect::<HashSet<Pt3d>>()
        })
        .collect()
}

pub fn day19(input: String) -> Answer {
    let mut answer = Answer::default();
    let re = Regex::new(r"(.+),(.+),(.+)").unwrap();

    //parse into hashmaps
    let mut signals: Vec<HashSet<Pt3d>> = input
        .split("\n\n")
        .map(|s| {
            re.captures_iter(s)
                .map(|c| {
                    (
                        c[1].parse().unwrap(),
                        c[2].parse().unwrap(),
                        c[3].parse().unwrap(),
                    )
                })
                .collect()
        })
        .collect();

    //absolutes is all signals from 0th probe
    let mut absolutes: HashSet<Pt3d> = signals.remove(0);

    answer.record_parsed();

    //part 1: 'attach' all pieces together.
    let mut ts = vec![];
    while signals.len() > 0 {
        println!("Segments remaining: {}", signals.len());
        'outermost: for i in 0..signals.len() {
            for s in axis_mutations(&signals[i]) {
                for p1 in absolutes.iter() {
                    for p2 in s.iter() {
                        let t = p1.sub(p2);
                        let transformed = transform(&s, &p1.sub(p2));

                        if transformed.intersection(&absolutes).count() >= 12 {
                            absolutes.extend(transformed);
                            signals.remove(i);
                            ts.push(t);
                            break 'outermost;
                        }
                    }
                }
            }
        }
    }
    answer.record(&absolutes.len());

    //part 2: find the greatest manhattan distance between two points
    let mut man = 0;
    for ((x1,y1,z1),(x2,y2,z2)) in ts.iter().tuple_combinations(){
        let m = i32::abs(x2-x1) + i32::abs(y2-y1) + i32::abs(z2-z1);
        man = i32::max(man,m);
    }

    answer.record(&man);
    
    answer
}
