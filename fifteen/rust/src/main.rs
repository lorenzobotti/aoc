use std::io::{self, Read};

use convert::enlarge;
use field::Field;
use walk::Distances;

use crate::coord::Coord;

mod coord;
mod field;
mod walk;
mod convert;

fn main() {
    let original = read_stdin();
    let input = enlarge(&original);
    // println!("{}", input);

    let field = Field::new(&input);

    let start = field.get(&Coord::new(0, 0));
    if start != Some(1) {
        panic!("{:?}", start);
    }

    let mut solver = Distances::new(&field);
    loop {
        let nxt = solver.next();
        // dbg!(nxt);

        if let Some(solution) = nxt{
            println!("{}", solution);
            break;
        }
    }
}

fn read_stdin() -> String {
    let mut contents = Vec::new();
    io::stdin().read_to_end(&mut contents).unwrap();

    String::from_utf8(contents).unwrap()
}
