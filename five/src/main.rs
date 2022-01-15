use std::io::{self, Read};

use field::{Field, Line};

use crate::field::Coord;

mod field;
mod range;

fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();

    let input = String::from_utf8(input).unwrap();
    // let input = include_str!("../sample.txt");

    dbg!("parsing input"); 
    let lines = input
        .lines()
        .map(|line| Line::from_str(line))
        .filter(|line| line.is_some())
        .map(|line| line.unwrap())
        .collect::<Vec<Line>>();

    let mut field = Field::new();
    lines.iter()
    // .filter(|line| line.horizontal() || line.vertical())
    .for_each(|line| {
        field.add_line(line);
        println!("{:?}", line);
        println!("{}", field);
    });

    dbg!(field
        .points()
        .iter()
        .filter(|(_, pts)| *pts >= 2)
        .count());
}
