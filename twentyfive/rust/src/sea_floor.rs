use std::fmt;

pub enum Squid {
    Nothing,
    East,
    South,
}


impl fmt::Display for Squid {
    fn 
}





pub struct SeaFloor {
    squids: Vec<Squid>,
    rows: usize,
    cols: usize,
}


impl SeaFloor {
    pub fn new(input: &str) -> SeaFloor {
        let mut out = Vec::new();
        let mut lines = input.lines().count();
        let mut columns = input.lines().next().unwrap().chars().count();



        for line in input.lines() {
            for ch in line.chars() {
                out.push(match ch {
                    '>' => Squid::East,
                    'v' => Squid::South,
                    '.' => Squid::Nothing,
                    other => panic!("unexpected: {}", other),
                })
            }
        }

        Self {
            squids: out,
            rows: lines,
            cols: columns,
        }
    }
}