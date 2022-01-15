use core::fmt;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::utils::*;
pub type CaveRef<'a> = Rc<RefCell<Cave<'a>>>;

#[derive(Debug, PartialEq, Clone)]
pub struct Cave<'a> {
    pub name: &'a str,
    // connections: Vec<Rc<Cave<'a>>>
    pub connections: Vec<CaveRef<'a>>,
}

impl<'a> Cave<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name: name,
            connections: Vec::new(),
        }
    }

    pub fn connect(&mut self, to: CaveRef<'a>) {
        self.connections.push(to);
    }

    pub fn big(&self) -> bool {
        is_uppercase(self.name)
    }

    pub fn small(&self) -> bool {
        is_lowercase(self.name)
    }
}

impl<'a> fmt::Display for Cave<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.name)?;
        
        for cave in &self.connections {
            write!(f, "  {}\n", cave.borrow().name)?;
        }

        Ok(())
    }   
}

