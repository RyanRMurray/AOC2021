use crate::utils::Answer;

#[derive(Debug, Clone)]
enum P {
    P(Vec<Packet>),
    L(usize),
}

#[derive(Debug, Clone)]
struct Packet {
    version: usize,
    p_type: usize,
    payload: P,
}

fn to_number(slice: &[usize]) -> usize {
    (0..slice.len())
        .rev()
        .zip(slice)
        .fold(0, |sum, (pow, val)| sum + val * usize::pow(2, pow as u32))
}

fn version_sum(p: &Packet) -> usize {
    p.version
        + match &p.payload {
            P::L(_) => 0,
            P::P(subs) => subs.iter().map(|s| version_sum(s)).sum(),
        }
}

fn execute_packet(p: Packet) -> usize {
    match p.payload {
        P::L(x) => x,
        P::P(xs) => {
            if p.p_type < 4 {
                let vs = xs.into_iter().map(|x| execute_packet(x));
                match p.p_type {
                    0 => vs.sum(),
                    1 => vs.product(),
                    2 => vs.min().unwrap(),
                    _ => vs.max().unwrap(),
                }
            } else {
                let a = execute_packet(xs[0].clone());
                let b = execute_packet(xs[1].clone());
                let res = match p.p_type {
                    5 => a > b,
                    6 => a < b,
                    _ => a == b,
                };
                if res {
                    1
                } else {
                    0
                }
            }
        }
    }
}

fn construct_packet(queue: Vec<usize>) -> (Packet, Vec<usize>) {
    let ver = to_number(&queue[0..3]);
    let p_t = to_number(&queue[3..6]);
    let mut remain;

    let pl = match (p_t, queue[6]) {
        (4, _) => {
            let mut i = 6;
            let mut res = vec![];
            loop {
                res.extend(queue[i + 1..i + 5].to_vec());
                if queue[i] == 1 {
                    i += 5;
                } else {
                    remain = queue[i + 5..].to_vec();
                    break;
                }
            }
            P::L(to_number(&res))
        }
        (_, 0) => {
            let pl_sz = to_number(&queue[7..22]) as usize;
            remain = queue[22 + pl_sz..].to_vec();
            let mut sub = queue[22..22 + pl_sz].to_vec();
            let mut pls = vec![];
            while sub.len() > 0 {
                let (new_pl, new_sub) = construct_packet(sub);
                pls.push(new_pl);
                sub = new_sub;
            }
            P::P(pls)
        }
        _ => {
            let sub_num = to_number(&queue[7..18]);
            let mut pls = vec![];
            remain = queue[18..].to_vec();
            for _ in 0..sub_num {
                let (new_pl, new_sub) = construct_packet(remain);
                pls.push(new_pl);
                remain = new_sub;
            }
            P::P(pls)
        }
    };

    let p = Packet {
        version: ver,
        p_type: p_t,
        payload: pl,
    };

    (p, remain)
}

pub fn day16(input: String) -> Answer {
    let mut answer = Answer::default();

    //parse hex to bit vec
    let b_vec: Vec<usize> = input
        .chars()
        .map(|c| {
            let x = format!("0000{:b}", c.to_digit(16).unwrap());
            x.chars()
                .map(|cc| cc.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()[x.len() - 4..]
                .to_vec()
        })
        .flatten()
        .collect();

    //parse into packets
    let (ps, _) = construct_packet(b_vec);
    answer.record_parsed();

    //part 1: sum version numbers
    let p1 = version_sum(&ps);
    answer.record(&p1);

    //part 2: execute
    let p2 = execute_packet(ps);
    answer.record(&p2);

    answer
}
