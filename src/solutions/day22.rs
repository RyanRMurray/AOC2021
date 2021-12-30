use crate::utils::Answer;
use regex::Regex;

#[derive(Debug)]
enum Op {
    On(Cuboid),
    Off(Cuboid),
}

impl Op {
    fn space(&self) -> Cuboid {
        match self {
            Op::Off(c) => c.clone(),
            Op::On(c) => c.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct Cuboid {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    min_z: i64,
    max_z: i64,
}

impl Cuboid {
    fn new(x1: i64, x2: i64, y1: i64, y2: i64, z1: i64, z2: i64) -> Self {
        Cuboid {
            min_x: x1,
            max_x: x2,
            min_y: y1,
            max_y: y2,
            min_z: z1,
            max_z: z2,
        }
    }

    fn vol(&self) -> i64 {
        [
            (self.min_x - 1, self.max_x),
            (self.min_y - 1, self.max_y),
            (self.min_z - 1, self.max_z),
        ]
        .iter()
        .map(|(a, b)| i64::abs(b - a))
        .product()
    }

    fn intersects(&self, oth: &Cuboid) -> bool {
        let i = self.intersection(oth);

        (i.min_x <= i.max_x) && (i.min_y <= i.max_y) && (i.min_z <= i.max_z)
    }

    fn intersection(&self, oth: &Cuboid) -> Cuboid {
        Cuboid::new(
            i64::max(self.min_x, oth.min_x),
            i64::min(self.max_x, oth.max_x),
            i64::max(self.min_y, oth.min_y),
            i64::min(self.max_y, oth.max_y),
            i64::max(self.min_z, oth.min_z),
            i64::min(self.max_z, oth.max_z),
        )
    }

    //returns non-overlapping sections of self with a different cuboid
    fn atomize(&self, neg: &Cuboid) -> Vec<Cuboid> {
        let mut res = vec![];

        //front, back
        if self.max_x > neg.max_x {
            res.push(Cuboid::new(
                neg.max_x + 1,
                self.max_x,
                self.min_y,
                self.max_y,
                self.min_z,
                self.max_z,
            ))
        }
        if self.min_x < neg.min_x {
            res.push(Cuboid::new(
                self.min_x,
                neg.min_x - 1,
                self.min_y,
                self.max_y,
                self.min_z,
                self.max_z,
            ))
        }
        //top, bottom
        if self.max_y > neg.max_y {
            res.push(Cuboid::new(
                neg.min_x,
                neg.max_x,
                neg.max_y + 1,
                self.max_y,
                self.min_z,
                self.max_z,
            ))
        }
        if self.min_y < neg.min_y {
            res.push(Cuboid::new(
                neg.min_x,
                neg.max_x,
                self.min_y,
                neg.min_y - 1,
                self.min_z,
                self.max_z,
            ))
        }
        //sides
        if self.max_z > neg.max_z {
            res.push(Cuboid::new(
                neg.min_x,
                neg.max_x,
                neg.min_y,
                neg.max_y,
                neg.max_z + 1,
                self.max_z,
            ))
        }
        if self.min_z < neg.min_z {
            res.push(Cuboid::new(
                neg.min_x,
                neg.max_x,
                neg.min_y,
                neg.max_y,
                self.min_z,
                neg.min_z - 1,
            ))
        }

        res
    }
}

fn add_cube(mut cuboids: Vec<Cuboid>, i: Op) -> Vec<Cuboid> {
    let newboid = i.space();

    let overlapping: Vec<Cuboid> = cuboids
        .drain_filter(|oth| newboid.intersects(&oth))
        .collect();

    let atomized: Vec<Cuboid> = overlapping
        .into_iter()
        .map(|c| c.atomize(&c.intersection(&newboid)))
        .flatten()
        .collect();

    cuboids.extend(atomized);

    match i {
        Op::Off(_) => (),
        Op::On(n) => cuboids.push(n),
    }

    cuboids
}

fn in_core(cuboid: &Cuboid) -> bool {
    cuboid.min_x >= -50
        && cuboid.max_x <= 50
        && cuboid.min_y >= -50
        && cuboid.max_y <= 50
        && cuboid.min_z >= -50
        && cuboid.max_z <= 50
}

pub fn day22(input: String) -> Answer {
    let mut answer = Answer::default();
    let re =
        Regex::new(r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();

    //parse instructions
    let instrs: Vec<Op> = re
        .captures_iter(&input)
        .map(|c| {
            let x = Cuboid::new(
                c[2].parse().unwrap(),
                c[3].parse().unwrap(),
                c[4].parse().unwrap(),
                c[5].parse().unwrap(),
                c[6].parse().unwrap(),
                c[7].parse().unwrap(),
            );
            if &c[1] == "on" {
                Op::On(x)
            } else {
                Op::Off(x)
            }
        })
        .collect();

    //construct set of on cubes
    let mut ons: Vec<Cuboid> = vec![];
    for i in instrs {
        ons = add_cube(ons, i);
    }

    //get volumes of inner set
    let p1: i64 = ons.iter().filter(|c| in_core(c)).map(|c| c.vol()).sum();
    answer.record(&p1);

    //get all volumes
    let p2: i64 = ons.iter().map(|c| c.vol()).sum();
    answer.record(&p2);

    answer
}
