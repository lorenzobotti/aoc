#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coord {
    pub row: isize,
    pub col: isize,
}

impl Coord {
    pub fn new(row: isize, col: isize) -> Self { Self{ row, col }}
    pub fn plus(self, other: &Self) -> Self {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }

    pub fn neighbours(&self) -> [Self; 9] {
        [
            self.plus(&Self::new(-1, -1)),
            self.plus(&Self::new(-1, 0)),
            self.plus(&Self::new(-1, 1)),
            self.plus(&Self::new(0, -1)),
            self.plus(&Self::new(0, 0)),
            self.plus(&Self::new(0, 1)),
            self.plus(&Self::new(1, -1)),
            self.plus(&Self::new(1, 0)),
            self.plus(&Self::new(1, 1)),
        ]
    }
}


pub struct CoordIter {
    pub rows: isize,
    pub cols: isize,
    current: isize,
}

impl CoordIter {
    pub fn new(rows: isize, cols: isize) -> Self {
        Self {
            rows, cols,
            current: 0,
        }
    }
}

impl Iterator for CoordIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        let end = self.rows * self.cols;
        if self.current == end {
            return None;
        }

        let row = self.current / self.cols;
        let col = self.current % self.cols;

        self.current += 1;

        Some(Coord::new(row, col))
    }
}