use crate::bin_number::{Bin, Digit};

#[derive(Debug, Clone)]
pub struct ProblemInput(pub Vec<Bin>);

impl ProblemInput {
    pub fn from_str(inp: &str) -> Self {
        Self(
            inp
            .lines()
            .map(|l| Bin::from_str(l))
            .collect::<Vec<Bin>>()
        )
    }

    pub fn element_size(&self) -> usize {
        if self.0.len() > 0 {
            self.0[0].len()
        } else {
            0
        }
    }

    pub fn most_common_at(&self, i: usize) -> Option<Digit> {
        let mut zeroes = 0;
        let mut ones = 0;

        for bin in &self.0 {
            match bin.at(i) {
                Some(Digit::Zero) => zeroes += 1,
                Some(Digit::One) => ones += 1,
                None => return None,
            }
        }

        if zeroes >= ones {
            Some(Digit::Zero)
        } else {
            Some(Digit::One)
        }
    }

    pub fn gamma_epsilon(&self) -> (Vec<Digit>, Vec<Digit>) {
        let mut gamma = Vec::new();
        let mut epsilon = Vec::new();

        for i in 0..self.element_size() {
            let bit = self.most_common_at(i).unwrap();

            gamma.push(bit);
            epsilon.push(bit.not());
        }

        (gamma, epsilon)
    }
}

// pub fn into_num(digits: &[Digit]) -> Option<usize> {
//     digits
//         .iter()
//         .map(|d| if *d == Digit::Zero { 0 } else { 1 })
//         .reduce(|acc, cur| (acc << 1) + 1)
// }
pub fn into_num(digits: &[Digit]) -> usize {
    let mut res: usize = 0;
    for d in digits {
        let num = if *d == Digit::Zero { 0 } else { 1 };
        res = (res << 1) + num;
    }

    res
}
