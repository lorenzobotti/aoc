mod bin_number;
mod numbers;

use std::io::{prelude::*, stdin};

use bin_number::Bin;
use bin_number::BinBuilder;
use numbers::ProblemInput;

use crate::numbers::into_num;

fn main() {
    let mut input = Vec::new();
    stdin().read_to_end(&mut input).unwrap();
    let input = String::from_utf8(input).unwrap();

    let parsed = ProblemInput::from_str(&input);
    let mut most_common = BinBuilder::new();
    
    for i in 0..parsed.element_size() {
        let most_common_digit = parsed.most_common_at(i).unwrap();
        most_common.add(&most_common_digit);

        let matching = parsed.0
            .iter()
            .filter(|bin| bin.starts_with(&most_common.build()))
            .collect::<Vec<&Bin>>();

        if matching.len() == 1 {
            println!("found! {:?}", matching[0]);
        }
    }
}
