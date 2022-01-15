use smallvec::SmallVec;

use crate::coord::Coord;

#[derive(Debug, Clone)]
pub struct Field<'a> {
    data: &'a [u8],

    rows: usize,
    columns: usize,
}

impl<'a> Field<'a> {
    pub fn new(input: &'a str) -> Self {
        let rows = input.lines().count();
        let columns = input.lines().next().unwrap().chars().count();

        Self {
            data: input.as_bytes(),

            rows,
            columns,
        }
    }

    pub fn get(&self, c: &Coord) -> Option<u8> {
        if self.in_bounds(c) {
            let val = self.data[c.row * (self.columns + 1) + c.column];
            if val < b'0' {
                panic!("unknown digit: {}", val);
            }

            Some(val - b'0')
        } else {
            None
        }
    }

    pub fn in_bounds(&self, c: &Coord) -> bool {
        c.row < self.rows && c.column < self.columns
    }

    pub fn neighbors(&self, c: &Coord) -> SmallVec<[Coord; 4]> {
        c
            .neighbors()
            .iter()
            .filter(|neighbor| self.in_bounds(neighbor))
            .copied()
            .collect()
    }

    pub fn start(&self) -> Coord {
        Coord::new(0,0)
    }
    
    pub fn end(&self) -> Coord {
        Coord::new(self.rows - 1,self.columns - 1)
    }
    
}
