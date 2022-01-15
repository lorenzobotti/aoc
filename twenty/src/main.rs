use std::io::Read;

use parse::parse_input;

use crate::coord::Coord;

mod parse;
mod coord;

fn main() {
    let input = read_input();

    let (alg, mut img) = parse_input(&input);
    // println!("{}", img);

    for i in 0..2 {
        // dbg!(i);
        img = img.decompress(&alg);
        // println!("\n\n");
        // println!("{}", img);
    }

    dbg!(img.count());
}

fn read_input() -> String {
    let mut buf = Vec::new();
    std::io::stdin().read_to_end(&mut buf).unwrap();

    String::from_utf8(buf).unwrap()
}