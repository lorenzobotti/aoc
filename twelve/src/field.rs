use core::fmt;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::cave::*;

#[derive(Default, Debug)]
pub struct Field<'a> {
    caves: HashMap<&'a str, CaveRef<'a>>,
}

// use std::hash::{Hash, Hasher};
// use fasthash::RandomState;
// use fasthash::spooky::Hash128;

impl<'a> Field<'a> {
    pub fn from_str(input: &'a str) -> Self {
        // let hasher = RandomState::<Hash128>::new();
        // let mut caves: HashMap<&str, CaveRef<'a>> = HashMap::with_hasher(Hash128);
        let mut caves: HashMap<&str, CaveRef<'a>> = HashMap::new();

        for line in input.lines() {
            let mut parts = line.split('-');

            let one = parts.next().unwrap();
            let two = parts.next().unwrap();

            if caves.get(one) == None {
                let cave = Rc::new(RefCell::new(Cave::new(one)));
                caves.insert(one, cave);
            }
            if caves.get(two) == None {
                let cave = Rc::new(RefCell::new(Cave::new(two)));
                caves.insert(two, cave);
            }

            if let Some(cave_one) = caves.get(one) {
                if let Some(cave_two) = caves.get(two) {
                    cave_one.borrow_mut().connect(cave_two.clone());
                    cave_two.borrow_mut().connect(cave_one.clone());
                }
            }
        }

        Self { caves }
    }

    pub fn start(&self) -> CaveRef<'a> {
        match self.caves.get("start") {
            Some(c) => c.clone(),
            None => unreachable!(),
        }
    }
}

impl<'a> fmt::Display for Field<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for cave in self.caves.values() {
            cave.borrow().fmt(f)?;
            write!(f, "\n")?;
        }

        Ok(())
    }   
}
