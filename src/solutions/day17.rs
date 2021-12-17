use crate::utils::{Answer, Point, Pt2d};
use regex::Regex;
use std::cmp::Ordering;

//accelerate via gravity and drag
fn accel((x, y): Pt2d) -> Pt2d {
    (
        match x.cmp(&0) {
            Ordering::Less => x + 1,
            Ordering::Equal => x,
            Ordering::Greater => x - 1,
        },
        y - 1,
    )
}

//get the max height if there is eventually an intersection with the target bounds
fn height_from(bounds: &(i32, i32, i32, i32), mut vel: Pt2d) -> i32 {
    let mut res = 0;
    let mut collided = false;
    let mut ptr = (0, 0);

    while ptr.0 <= bounds.1 && ptr.1 >= bounds.2 {
        if !collided && ptr.0 >= bounds.0 && ptr.1 <= bounds.3 {
            collided = true;
        }

        res = i32::max(res, ptr.1);
        ptr = ptr.add(vel);
        vel = accel(vel);
    }

    if collided {
        res
    } else {
        -1
    }
}
pub fn day17(input: String) -> Answer {
    let mut answer = Answer::default();
    let re = Regex::new(r"target area: x=(.+)\.\.(.+), y=(.+)\.\.(.+)").unwrap();

    //parse target area
    let c = re.captures(&input).unwrap();
    let bounds = (
        c[1].parse::<i32>().unwrap(),
        c[2].parse::<i32>().unwrap(),
        c[3].parse::<i32>().unwrap(),
        c[4].parse::<i32>().unwrap(),
    );
    answer.record_parsed();

    let mut max_h = 0;
    let mut on_target = 0;
    for x in 1..bounds.1 + 1 {
        for y in (-(1 - bounds.2)..1 - bounds.2).rev() {
            match height_from(&bounds, (x, y)) {
                -1 => {}
                v => {
                    max_h = i32::max(max_h, v);
                    on_target += 1;
                }
            }
        }
    }
    answer.record_both(&max_h, &on_target);
    answer
}
