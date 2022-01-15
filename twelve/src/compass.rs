use std::collections::HashSet;
use crate::cave::Cave;

#[derive(Debug, Clone, Default)]
pub struct Compass<'a> {
    visited: HashSet<&'a str>,
    twice: bool,
}

impl<'a> Compass<'a> {
    pub fn new() -> Self {
        Self {
            visited: HashSet::new(),
            twice: false,
        }
    }

    pub fn can_visit(&self, cave: &str) -> bool {
        if Cave::new(cave).big() {
            true
        } else {
            if self.visited.contains(cave) {
                if cave == "start" || cave == "end" {
                    false
                } else {
                    !self.twice
                }
            } else {
                true
            }
        }
    }

    pub fn visit(&mut self, cave: &'a str) -> bool {
        if Cave::new(cave).big() {
            return true;
        }


        let first_time = self.visited.insert(cave);
        if !first_time {
            if !self.twice {
                self.twice = true;
                true
            } else {
                false
            }
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twice() {
        let mut compass = Compass::new();

        assert!(compass.can_visit("start"));
        compass.visit("start");
        assert!(!compass.can_visit("start"));
        
        compass.visit("hamburger");
        assert!(compass.can_visit("hamburger"));
        
        compass.visit("hamburger");
        assert!(!compass.can_visit("hamburger"));
        
        compass.visit("mamma mia");
        assert!(!compass.can_visit("mamma mia"));
        

        assert!(compass.can_visit("HELLO"));
        compass.visit("HELLO");
        assert!(compass.can_visit("HELLO"));
        compass.visit("HELLO");
        assert!(compass.can_visit("HELLO"));
        compass.visit("HELLO");
        assert!(compass.can_visit("HELLO"));
        
        assert!(!compass.clone().can_visit("hamburger"));
    }

    #[test]
    fn it_wont_work() {
        let mut compass = Compass::new();
        let caves = ["start", "A", "b", "A", "c", "A"];
        for cave in caves {
            compass.visit(cave);
        }

        dbg!(&compass);
        assert!(compass.can_visit("b"));
        assert!(compass.can_visit("c"));
        assert!(compass.can_visit("end"));
    }
}