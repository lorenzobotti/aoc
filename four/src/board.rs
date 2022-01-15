use core::fmt;
use std::fmt::Display;

use smallvec::SmallVec;

#[derive(Debug, Default, Copy, Clone)]
pub struct Cell {
    pub num: usize,
    pub marked: bool,
}

impl Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.marked {
            write!(f, "\x1B[35m{}\x1b[0m", self.num)
        } else {
            write!(f, "{}", self.num)
        }
    }
}

#[derive(Debug, Default)]
pub struct Board(pub [Cell; 25]);

impl Board {
    pub fn from_str(input: &str) -> Self {
        let cells = input
            .trim()
            .lines()
            .map(|line| line.split(" "))
            .flatten()
            .map(|num| num.trim().parse::<usize>())
            .filter(|res| res.is_ok())
            .map(|res| res.unwrap())
            .map(|num| Cell { num: num, marked: false })
            .collect::<Vec<Cell>>();

        let mut out = [Cell::default(); 25];
        cells.into_iter().enumerate().for_each(|(i, cell)| out[i] = cell);
        Board(out)
    }

    pub fn extract(&mut self, num: usize) {
        self.0
            .iter_mut()
            .filter(|cell| cell.num == num)
            .for_each(|cell| cell.marked = true);
    }

    pub fn winning(&self) -> bool {
        for row in self.rows() {
            if row.full() {
                return true;
            }
        }
        
        for column in self.columns() {
            if column.full() {
                return true;
            }
        }

        false
    }

    pub fn score(&self) -> usize {
        self.0
            .iter()
            .filter(|cell| !cell.marked)
            .map(|cell| cell.num)
            .reduce(|acc, cur| acc + cur)
            .unwrap()
    } 

    pub fn row(&self, i: usize) -> Row {
        Row { board: self, row: i, counter: 0 }
    }

    pub fn column(&self, i: usize) -> Column {
        Column { board: self, column: i, counter: 0 }
    } 
    
    pub fn rows(&self) -> Rows {
        Rows { board: self, counter: 0 }
    }

    pub fn columns(&self) -> Columns {
        Columns { board: self, counter: 0 }
    } 
      
}


impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.rows() {
            for cell in row {
                cell.fmt(f)?;
                write!(f, " ")?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}


pub struct Rows<'a> {
    board: &'a Board,
    counter: usize,
}

impl<'a> Iterator for Rows<'a> {
    type Item = Row<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter > 4 {
            None
        } else {
            let row = self.board.row(self.counter);
            self.counter += 1;
            Some(row)
        }
    }
}

pub struct Columns<'a> {
    board: &'a Board,
    counter: usize,
}

impl<'a> Iterator for Columns<'a> {
    type Item = Column<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter > 4 {
            None
        } else {
            let column = self.board.column(self.counter);
            self.counter += 1;
            Some(column)
        }
    }
}


pub struct Row<'a> {
    board: &'a Board,
    row: usize,
    counter: usize,
}

impl<'a> Iterator for Row<'a> {
    type Item = Cell;
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter > 4 {
            None
        } else {
            let idx = (self.row * 5) + self.counter;
            self.counter += 1;
            Some(self.board.0[idx])
        }
    }
}

impl<'a> Row<'a> {
    pub fn full(self) -> bool {
        for cell in self {
            if !cell.marked {
                return false;
            }
        }

        true
    }
}


pub struct Column<'a> {
    board: &'a Board,
    column: usize,
    counter: usize,
}

impl<'a> Iterator for Column<'a> {
    type Item = Cell;
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter > 4 {
            None
        } else {
            let idx = (self.counter * 5) + self.column;
            self.counter += 1;
            Some(self.board.0[idx])
        }
    }
}

impl<'a> Column<'a> {
    pub fn full(self) -> bool {
        for cell in self {
            if !cell.marked {
                return false;
            }
        }

        true
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winning() {
        let input = r#"14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7"#;

        let mut board = Board::from_str(input);
        let nums = [7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1];
        let last_winning_number = 24;
        let score = 188;
        let solution =  4512;

        for num in nums {
            board.extract(num);

            if board.winning() {
                assert_eq!(num, last_winning_number);
                assert_eq!(board.score(), score);
                assert_eq!(board.score() * num, solution);

                return;
            }
        }

        panic!();
    }
}