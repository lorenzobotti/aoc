use std::hash::Hash;

use smallvec::SmallVec;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coord {
    pub row: usize,
    pub column: usize,
}

impl Coord {
    pub fn new(row: usize, column: usize) -> Self {
        Self {
            row,
            column,
        }
    }

    pub fn right(&self) -> Self {
        Self { row: self.row, column: self.column + 1}
    }
    pub fn left(&self) -> Option<Self> {
        if self.column == 0 {
            None
        } else {
            Some(Self { row: self.row, column: self.column - 1})
        }
    }
    pub fn up(&self) -> Option<Self> {
        if self.row == 0 {
            None
        } else {
            Some(Self { row: self.row - 1, column: self.column})
        }
    }
    pub fn down(&self) -> Self {
        Self { row: self.row + 1, column: self.column}
    }

    pub fn neighbors(&self) -> SmallVec<[Coord; 4]> {
        let mut out = SmallVec::new();
        
        out.push(self.down());
        out.push(self.right());
        if let Some(up) = self.up() {
            out.push(up);
        }
        if let Some(left) = self.left() {
            out.push(left);
        }

        out 
    }
}

// impl Hash for Coord {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         state.write_u32(self.row as u32);
//         state.write_u32(self.column as u32);
//     }
// }


// #[derive(Debug, Default)]
// pub struct CoordHasher(u64);

// impl Hasher for CoordHasher {
//     fn write(&mut self, bytes: &[u8]) {
//         for b in bytes {
//             self.0 = (self.0 << 8) | *b as u64;
//         }
//     }

//     fn finish(&self) -> u64 {
//         self.0
//     }
// }