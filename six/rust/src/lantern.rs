pub struct Lantern {
    pub days_till_kid: usize,
}

const DAYS_FOR_KID: usize = 7;
const DAYS_FOR_PUBERTY: usize = 2;


impl Lantern {
    pub fn new(days: usize) -> Self {
        Self { days_till_kid: days }
    }

    pub fn tick(&mut self, sea: &mut Sea) {
        if self.days_till_kid == 0 {
            sea.fishes.push(
                Rc::new(
                    RefCell::new(
                        Lantern::new(DAYS_FOR_KID + DAYS_FOR_PUBERTY - 1)
                    )
                )
            );

            self.days_till_kid = DAYS_FOR_KID - 1;
        } else {
            self.days_till_kid -= 1;
        }
    }
}


use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

pub struct Sea {
    // fishes: Vec<Lantern>,
    fishes: Vec<Rc<RefCell<Lantern>>>,
}

impl Sea {
    pub fn from_str(input: &str) -> Self {
        todo!();
    }

    pub fn tick(&mut self) {
        for i in 0..self.fishes.len() {
            self.fishes[0].borrow_mut().tick(&mut self);
        }
    }
}