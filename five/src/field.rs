use core::fmt;
use std::{collections::HashMap};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn from_str(input: &str) -> Option<Self> {
        let mut parts = input.split(",");
        let x = parts.next()?.trim().parse::<usize>().ok()?;
        let y = parts.next()?.trim().parse::<usize>().ok()?;

        Some(Self { x, y })
    }
}

pub struct Field(HashMap<Coord, usize>);

impl Field {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn at(&self, coord: &Coord) -> Option<usize> {
        self.0.get(coord).map(|n| *n)
    }

    pub fn points(&self) -> Vec<(Coord, usize)> {
        self.0.iter().map(|(c, pts)| (*c, *pts)).collect()
    }

    pub fn add_pt(&mut self, coord: &Coord) {
        let points_already = match self.0.get(coord) {
            Some(n) => *n,
            None => 0,
        };
        self.0.insert(*coord, points_already + 1);
    }

    pub fn add_line(&mut self, line: &Line) {
        line.points().iter().for_each(|pt| self.add_pt(pt));
    }
}

use colored::*;
use crate::range::MyRange;

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..=10 {
            for x in 0..=10 {
                let pt = Coord { x, y };
                let gotten = self.at(&pt).unwrap_or(0);
                if gotten == 0 {
                    write!(f, "{} ", gotten)?;
                } else {
                    write!(f, "{} ", gotten.to_string().blue())?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub start: Coord,
    pub end: Coord,
}

impl Line {
    pub fn from_str(input: &str) -> Option<Self> {
        let mut parts = input.split(" -> ");
        let start = Coord::from_str(parts.next()?)?;
        let end = Coord::from_str(parts.next()?)?;

        Some(Self { start, end })
    }

    pub fn points(&self) -> Vec<Coord> {
        if self.horizontal() {
            make_range(self.start.y, self.end.y)
                .map(|y| Coord {
                    y: y,
                    x: self.start.x,
                })
                .collect()
        } else if self.vertical() {
            make_range(self.start.x, self.end.x)
                .map(|x| Coord {
                    y: self.start.y,
                    x: x,
                })
                .collect()
        } else if self.perfect_diagonal() {
            dbg!("perfect diagonal");

            let mut out = Vec::new();

            let mut x_range = MyRange::inclusive(self.start.x as isize, self.end.x as isize);
            let mut y_range = MyRange::inclusive(self.start.y as isize, self.end.y as isize);

            while let Some(x) = x_range.next() {
                if let Some(y) = y_range.next() {
                    out.push(Coord{ x: x as usize, y: y as usize });
                }
            }
            
            out
        } else {
            dbg!("not a diagonal");
            dbg!(self);
            Vec::new()
        }
    }

    pub fn horizontal(&self) -> bool {
        self.start.x == self.end.x
    }

    pub fn vertical(&self) -> bool {
        self.start.y == self.end.y
    }
    
    pub fn perfect_diagonal(&self) -> bool {
        (self.start.x as isize - self.end.x as isize).abs()
        == 
        (self.start.y as isize - self.end.y as isize).abs()
    }
    
}

pub fn make_range(start: usize, end: usize) -> impl Iterator<Item = usize> {
    if start < end {
        start..=end
    } else {
        end..=start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_in_line() {
        let line = Line {
            start: Coord { x: 3, y: 10 },
            end: Coord { x: 3, y: 6 },
        };
        let expected = vec![
            Coord { x: 3, y: 6 },
            Coord { x: 3, y: 7 },
            Coord { x: 3, y: 8 },
            Coord { x: 3, y: 9 },
            Coord { x: 3, y: 10 },
        ];

        dbg!(line.horizontal());
        dbg!(line.vertical());

        let got = line.points();
        assert_eq!(expected, got);
    }


    #[test]
    fn diagonals() {
        let line = Line{start: Coord{x: 3, y: 8}, end: Coord{x: 8, y: 3}};
        let expected_points = vec![
            Coord{ x: 3, y: 8 },
            Coord{ x: 4, y: 7 },
            Coord{ x: 5, y: 6 },
            Coord{ x: 6, y: 5 },
            Coord{ x: 7, y: 4 },
            Coord{ x: 8, y: 3 },
        ];
        let points_got = line.points();
        assert!(line.perfect_diagonal());
        assert_eq!(points_got, expected_points);

        let line = Line{start: Coord{x: 8, y: 8}, end: Coord{x: 3, y: 3}};
        assert!(line.perfect_diagonal());
    }
}
