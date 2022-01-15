use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone)]
pub struct HeightMap(Vec<Vec<u32>>);

impl Coord {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl HeightMap {
    pub fn from_str(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|ch| ch.to_digit(10).unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>(),
        )
    }

    pub fn rows(&self) -> usize {
        self.0.len()
    }

    pub fn cols(&self) -> usize {
        match self.0.len() {
            0 => 0,
            other => self.0[0].len(),
        }
    }

    pub fn in_bounds(&self, coord: &Coord) -> bool {
        let row = coord.row;
        let col = coord.col;

        if row < 0 || row >= self.rows() {
            false
        } else {
            if col < 0 || col >= self.cols() {
                false
            } else {
                true
            }
        }
    }

    pub fn get_at(&self, coord: &Coord) -> Option<u32> {
        let row = coord.row;
        let col = coord.col;

        if self.in_bounds(coord) {
            Some(self.0[row][col])
        } else {
            None
        }
    }

    fn neighbours(&self, c: &Coord) -> Vec<Coord> {
        let row = c.row;
        let col = c.col;

        let low_bound_row = if row == 0 { row } else { row - 1 };
        let low_bound_col = if col == 0 { col } else { col - 1 };

        let high_bound_row = if row + 1 == self.rows() { row } else { row + 1 };
        let high_bound_col = if col + 1 == self.cols() { col } else { col + 1 };

        let mut out = Vec::new();

        for r in low_bound_row..=high_bound_row {
            for c in low_bound_col..=high_bound_col {
                if r == row && c == col {
                    continue;
                }

                let coord = Coord::new(r, c);
                if self.in_bounds(&coord) {
                    out.push(coord)
                }
            }
        }

        out
    }

    fn arrow_neighbours(&self, c: &Coord) -> Vec<Coord> {
        let row = c.row;
        let col = c.col;

        let low_bound_row = if row == 0 { row } else { row - 1 };
        let low_bound_col = if col == 0 { col } else { col - 1 };

        let high_bound_row = if row + 1 == self.rows() { row } else { row + 1 };
        let high_bound_col = if col + 1 == self.cols() { col } else { col + 1 };

        let mut out = Vec::new();

        for r in low_bound_row..=high_bound_row {
            out.push(Coord::new(r, col))
        }

        for c in low_bound_col..=high_bound_col {
            out.push(Coord::new(row, c))
        }

        out
    }

    fn neighbours_val(&self, coord: &Coord) -> Vec<u32> {
        self.neighbours(coord)
            .iter()
            .map(|c| self.get_at(c).unwrap())
            .collect()
    }

    fn arrow_neighbours_val(&self, coord: &Coord) -> Vec<u32> {
        self.arrow_neighbours(coord)
            .iter()
            .map(|c| self.get_at(c).unwrap())
            .collect()
    }

    pub fn is_local_minima(&self, coord: &Coord) -> bool {
        let at = self.get_at(coord).unwrap();

        at < *self
            .neighbours_val(coord)
            .iter()
            .reduce(|acc, cur| if cur < acc { cur } else { acc })
            .unwrap()
    }

    pub fn coords(&self) -> Vec<Coord> {
        let mut out = Vec::new();
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                let coord = Coord::new(row, col);
                out.push(coord);
            }
        }

        out
    }

    pub fn risk_sum(&self) -> u32 {
        let mut risk = 0;
        for coord in self.coords() {
            if self.is_local_minima(&coord) {
                risk += self.get_at(&coord).unwrap() + 1;
            }
        }

        risk
    }

    fn roll_down(&self, c: &Coord) -> Coord {
        let neighbours = self.arrow_neighbours(c);
        let lowest_neighbour = neighbours
            .iter()
            .map(|n| (n, self.get_at(n).unwrap()))
            .reduce(|acc, cur| if acc.1 < cur.1 { acc } else { cur })
            .unwrap();

        let where_i_am = self.get_at(c).unwrap();

        if lowest_neighbour.1 < where_i_am {
            self.roll_down(lowest_neighbour.0)
        } else {
            *c
        }
    }

    pub fn find_basins(&self) -> Vec<(Coord, usize)> {
        let mut basins = HashMap::new();
        for coord in self.coords() {
            if self.get_at(&coord) == Some(9) {
                continue;
            }
            let bottom = self.roll_down(&coord);

            let count = basins.get(&bottom);
            let count = count.unwrap_or(&0) + 1;

            basins.insert(bottom, count);
        }

        basins.iter().map(|(c, count)| (*c, *count)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static input: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

    #[test]
    fn risk_sum() {
        let hm = HeightMap::from_str(input);
        assert_eq!(hm.risk_sum(), 15);
    }

    #[test]
    fn neighbours() {
        let hm = HeightMap::from_str(input);
        assert_eq!(hm.cols(), 10);
        assert_eq!(hm.rows(), 5);
        assert_eq!(hm.neighbours(&Coord::new(0, 0)).len(), 3);
        assert_eq!(hm.neighbours(&Coord::new(1, 0)).len(), 5);
        assert_eq!(hm.neighbours(&Coord::new(0, 1)).len(), 5);
        assert_eq!(hm.neighbours(&Coord::new(1, 1)).len(), 8);
        assert_eq!(hm.neighbours(&Coord::new(1, 9)).len(), 5);
        assert_eq!(hm.neighbours(&Coord::new(13, 1)).len(), 0);
    }

    #[test]
    fn roll() {
        let hm = HeightMap::from_str(input);
        assert_eq!(hm.roll_down(&Coord::new(0, 5)), Coord::new(0, 9));
    }
}
