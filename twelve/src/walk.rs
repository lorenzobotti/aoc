use core::fmt;
use std::collections::HashSet;

use crate::field::*;
use crate::cave::*;
use crate::utils::*;

pub type Path<'a> = Vec<CaveRef<'a>>;

impl<'a> Field<'a> {
    pub fn walk(&self) -> Vec<Path> {
        let start = self.start();
        
        





        todo!();
    }
}


pub fn walk<'a>(cave: CaveRef<'a>) -> Vec<Path<'a>> {
    let mut set =  HashSet::new(); 
    walk_rec(cave, &mut set)
}

fn walk_rec<'a>(cave: CaveRef<'a>, walked_already: &HashSet<&'a str>) -> Vec<Path<'a>> {
    let conns = &cave.borrow().connections;
    
    let available_paths: Vec<&CaveRef<'a>> = conns
    .iter()
    .filter(|c| {
        c.borrow().big() ||
        !walked_already.contains(c.borrow().name)
    })
    .collect();
    
    
    
    let mut out = Vec::new();
    if available_paths.len() == 0 || cave.borrow().name == "end" {
        out.push(vec![cave.clone()]);
        return out;
    }



    for conn in available_paths {
        let mut walked_already_clone = walked_already.clone();
        walked_already_clone.insert(cave.borrow().name);
        let mut paths = walk_rec(conn.clone(), &walked_already_clone);
        
        paths = paths
            .into_iter()
            .filter(|p| p.last().unwrap().clone().borrow().name == "end")
            .map(|pt| {
                let mut path = Vec::new();
                let mut pt_cloned = pt.clone();
                
                path.push(cave.clone());
                path.append(&mut pt_cloned);
                path
            })
            .collect();
    
        out.append(&mut paths);
    }


    out
}

use crate::compass::Compass;

pub fn walk_twice<'a>(cave: CaveRef<'a>) -> Vec<Path<'a>> {
    let mut compass =  Compass::new(); 
    walk_twice_rec(cave, &compass)
}

fn walk_twice_rec<'a>(cave: CaveRef<'a>, walked_already: &Compass) -> Vec<Path<'a>> {
    let conns = &cave.borrow().connections;
    
    let available_paths: Vec<&CaveRef<'a>> = conns
    .iter()
    .filter(|c| {
        c.borrow().big() ||
        walked_already.can_visit(c.borrow().name)
    })
    .collect();
    
    
    
    let mut out = Vec::new();
    if available_paths.len() == 0 || cave.borrow().name == "end" {
        out.push(vec![cave.clone()]);
        return out;
    }

    for conn in available_paths {
        let mut walked_already_clone = walked_already.clone();
        walked_already_clone.visit(cave.borrow().name);
        let mut paths = walk_twice_rec(conn.clone(), &walked_already_clone);
        
        paths = paths
            .into_iter()
            .filter(|p| p.last().unwrap().clone().borrow().name == "end")
            .map(|pt| {
                let mut path = Vec::new();
                let mut pt_cloned = pt.clone();
                
                path.push(cave.clone());
                path.append(&mut pt_cloned);
                path
            })
            .collect();
    
        out.append(&mut paths);
    }


    out
}

pub fn print_path(path: &Path) -> String {
    let mut out = String::new();
    
    for cave in path {
        let name = cave.borrow().name;
        out.push_str(name);
        if name != "end" {
            out.push_str("-");
        }
    }
    
    out
}