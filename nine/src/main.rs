use std::io::Read;

mod heightmap;
use heightmap::*;

fn main() {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();
    let input = String::from_utf8(input).unwrap();
    let hm = HeightMap::from_str(&input);

    // println!("{}", hm.risk_sum());
    // println!("{:?}", hm.find_basins().iter().map(|(c, count)| *count).collect::<Vec<usize>>());
    // dbg!("{:?}", hm.find_basins());

    let mut basins: Vec<usize> = hm.find_basins().iter().map(|(ch, count)| *count).collect();
    
    
    basins.sort();
    basins.reverse();
    dbg!(&basins);
    let solution = basins[..3].iter().map(|b| *b).reduce(|acc, cur| acc * cur);

    dbg!(solution);
}
