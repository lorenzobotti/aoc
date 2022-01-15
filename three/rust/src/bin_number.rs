use core::fmt;

#[derive(Debug, Clone)]
pub struct Bin(Vec<Digit>);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Digit {
    Zero,
    One,
}

impl Bin {
    pub fn from_str(inp: &str) -> Self {
        Self(inp
            .chars()
            .map(|ch| if ch == '0' { Digit::Zero } else { Digit::One })
            .collect())
    }

    pub fn at(&self, i: usize) -> Option<Digit> {
        if i < 0 || i >= self.len() {
            None
        } else {
            Some(self.0[i])
        }
    }

    pub fn starts_with(&self, other: &Self) -> bool {
        self.0[0..other.len()] == other.0 
    }

    pub fn len(&self) -> usize {
        self.0.len()
    } 

    pub fn iter(&self) -> BinIter {
        BinIter{ bin: self, at: 0 }
    }
}

impl fmt::Display for Bin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for digit in self.0.iter() {
            let ch: char = digit.char();
            write!(f, "{}", ch)?;
        }
        
        Ok(())
    }
}

pub struct BinIter<'a> {
    bin: &'a Bin,
    at: usize,
}

impl<'a> Iterator for BinIter<'a> {
    type Item = Digit;
    fn next(&mut self) -> Option<Self::Item> {
        let dig = self.bin.at(self.at);
        self.at += 1;
        dig
    }
}


pub struct BinBuilder {
    bin: Vec<Digit>
}

impl BinBuilder {
    pub fn new() -> Self { Self { bin: Vec::new() }}
    pub fn add(&mut self, dig: &Digit) {
        self.bin.push(*dig)
    }
    pub fn build(&self) -> Bin {
        Bin(self.bin.clone())
    }
}

impl From<char> for Digit {
    fn from(ch: char) -> Self {
        match ch {
            '0' => Self::Zero,
            '1' => Self::One,
            other => panic!("unexpected: {}", other),
        }
    }
}


impl Into<usize> for Bin {
    fn into(self) -> usize {
        self.iter()
            .map(|dig| if dig == Digit::Zero { 0 } else { 1 })
            .reduce(|acc, cur| (acc << 1) + cur)
            .unwrap_or(0)
    }
}

impl Digit {
    pub fn not(&self) -> Self {
        match *self {
            Self::One => Self::Zero,
            Self::Zero => Self::One,
        }
    }

    fn char(&self) -> char {
        match *self {
            Self::Zero => '0',
            Self::One => '1',
        }
    }
}