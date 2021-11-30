use std::time::{Instant};
use std::fmt::{Display, Formatter, Result};

type Record<'a> = (&'a dyn Display, Instant);

pub struct Answer<'a>{
    start: Instant,
    part1: Option<Record<'a>>,
    part2: Option<Record<'a>>
}

impl Default for Answer<'_>{
    fn default() -> Self{
        Self {
            start: Instant::now(),
            part1: None,
            part2: None
        }
    }
}

impl Display for Answer<'_>{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result{

        match self.part1 {
            None => write!(f, "Part 1 is not yet implemented\n"),
            Some((res1,t1)) => {
                let d1 = t1.saturating_duration_since(self.start);
                write!(f, "Part 1 Result: {}\nPart 1 Runtime: {:?}\n", res1, d1)
                .expect("Error trying to print Part 1 Results");

                match self.part2{
                    None => write!(f, "Part 2 is not yet implemented\n"),
                    Some((res2,t2)) => {
                        let d2 = t2.saturating_duration_since(t1);
                        let d3 = t2.saturating_duration_since(self.start);
                        write!(f, "Part 2 Result: {}\nPart 2 Runtime:  {:?}\nOverall Runtime: {:?}\n", res2, d2, d3)
                    }
                }

            }
        }
    }
}

impl<'a> Answer<'a>{
    pub fn record(&mut self, res: &'a dyn Display){
        match self.part1{
            None => self.part1 = Some((res, Instant::now())),
            Some(_) => {
                match self.part2{
                    None => self.part2 = Some((res, Instant::now())),
                    Some(_) => panic!("Cannot write third part to an answer!")
                }
            }
        }
    }
}