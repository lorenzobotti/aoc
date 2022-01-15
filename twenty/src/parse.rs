use core::fmt;

use crate::coord::{Coord, CoordIter};

const ZERO: char = '.';
const ONE: char = '#';


pub fn parse_input(input: &str) -> (Algorithm, Image) {
    let mut parts = input.split("\n\n");

    let alg = Algorithm::from_str(parts.next().unwrap());
    let img = Image::from_str(parts.next().unwrap());

    (alg, img)
}



#[derive(Clone, Debug)]
pub struct Algorithm(Vec<bool>);

impl Algorithm {
    pub fn from_str(input: &str) -> Self {
        Self(
            input.chars()
            .map(|ch| match ch {
                ZERO => false,
                ONE => true,
                other => panic!("unexpected: {:?}", other),
            })
            .collect()
        )
    }
}

#[derive(Clone, Debug)]
pub struct Image {
    pixels: Vec<bool>,
    rows: isize,
    cols: isize,
}

impl Image {
    pub fn from_str(input: &str) -> Self {
        let mut pixels = Vec::with_capacity(input.len());
        let mut rows = 0;
        let mut cols = 0;

        for ch in input.chars() {
            if ch == '\n' {
                rows += 1;
            }

            if rows == 0 {
                cols += 1
            }

            if ch.is_whitespace() {
                continue;
            }

            pixels.push(match ch {
                ZERO => false,
                ONE => true,
                other => panic!("unexpected: {}", other),
            });
        }

        Self { pixels, rows, cols }
    }

    pub fn get(&self, c: &Coord) -> Option<bool> {
        if self.in_bounds(c) {
            let idx = (self.cols * c.row + c.col);
            if idx < 0 {
                None
            } else {
                Some(self.pixels[idx as usize])
            }
        } else {
            None
        }
    }

    pub fn in_bounds(&self, c: &Coord) -> bool {
        c.row < self.rows && c.col < self.cols && c.row >= 0 && c.col >= 0
    }

    pub fn neighbours(&self, c: &Coord) -> Vec<bool> {
        c.neighbours()
            .iter()
            .map(|c| self.get(c))
            .map(|c| match c {
                Some(b) => b,
                None => false,
            })
            .collect()
    }

    pub fn decompress(&self, alg: &Algorithm, even_step: bool) -> Self {
        let mut new_img = Vec::new();
        // todo: check if outside is full or not
        
        for r in -1..=self.rows {
            for c in -1..=self.cols {
                let coord = Coord::new(r, c);
                let pixel = self.index_in_alg(alg, &coord, even_step);
                
                new_img.push(pixel);
            }
        }

        Self {
            pixels: new_img,
            rows: self.rows + 2,
            cols: self.cols + 2,
        }
    }

    pub fn count(&self) -> usize {
        self.coords()
            .map(|c| self.get(&c).unwrap_or(false))
            .filter(|c| *c)
            .count()
    }

    pub fn neighbours_as_img(&self, alg: &Algorithm, c: &Coord) -> Self {
        let mut pixels = Vec::new();


        for neighbor in c.neighbours().iter() {
            pixels.push(self.get(neighbor).unwrap_or(false));
        }

        Self {
            pixels,
            rows: 3,
            cols: 3,
        }
    }

    pub fn index_in_alg(&self, alg: &Algorithm, c: &Coord, even_step: bool) -> bool {
        let pixels: Vec<bool> = vec![c.neighbours()]
            .iter()
            .map(|c| self.get(&c))
            .map(|cell| match cell {
                Some(c) => c,
                None => even_step,
            })
            .collect();


        let mut idx = 0;
        for p in pixels {
            idx = idx << 1;
            if p {
                idx += 1;
            }
        }

        alg.0[idx]
    }


    pub fn coords(&self) -> impl Iterator<Item = Coord> {
        CoordIter::new(self.rows, self.cols)
    } 
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, pixel) in self.pixels.iter().enumerate() {
            if i % self.cols as usize == 0 && i != 0 {
                write!(f, "\n")?;
            }

            let ch = match *pixel {
                true => ONE,
                false => ZERO,
            };

            
            write!(f, "{}", ch)?;
        }

        Ok(())
    }
}

fn bools_to_num(input: &[bool]) -> usize {
    let mut out = 0;

    for b in input {
        out = out << 1;

        if *b {
            out += 1;
        }
    }

    out
}