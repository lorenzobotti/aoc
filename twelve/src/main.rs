use core::fmt;
use std::io::{self, Read};

use compass::Compass;

use crate::walk::print_path;

mod cave;
mod walk;
mod field;
mod utils;
mod compass;

fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let input = String::from_utf8(input).unwrap();

    let field = field::Field::from_str(&input);
    // println!("{}", field);
    // println!("starting walk");

    let paths = walk::walk_twice(field.start());
    // for path in &paths {
    //     println!("{}", walk::print_path(path));
    // }
    
    let solution = paths
        .iter()
        .map(|path| print_path(path))
        .filter(|path| valid_path(&path))
        .count();

    println!("solution: {}", solution);
}

fn valid_path(path: &str) -> bool {
    let mut comp = Compass::new();

    for piece in path.split('-') {
        if !comp.can_visit(piece) {
            return false;
        }
        comp.visit(piece);
    }

    true
}