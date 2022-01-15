use std::{collections::BinaryHeap, cmp::{Reverse, Ordering}};

use smallvec::SmallVec;
use rustc_hash::FxHashSet;

use crate::{coord::Coord, field::Field};

#[derive(Debug, Clone)]
pub struct Distances<'a> {
    nodes: FxHashSet<Coord>,
    orphans: BinaryHeap<Reverse<NodeDistance>>,
    field: &'a Field<'a>,
}


impl<'a> Distances<'a> {
    pub fn new(field: &'a Field) -> Self {
        let mut heap = BinaryHeap::new();
        heap.push(Reverse(
            NodeDistance::new(
                Node::new(&field.start(), field), 0)
            )
        );
        
        Self {
            nodes: FxHashSet::default(),
            orphans: heap,
            field,
        }
    }

    pub fn next(&mut self) -> Option<usize> {
        // dbg!(&self);

        if let Some(node) = self.orphans.pop() {
            // dbg!(&node);

            for neighbor in node.0.node.neighbors {
                if self.nodes.contains(&neighbor) {
                    continue;
                }

                let extra_distance = self.field.get(&neighbor).unwrap();
                let distance = node.0.distance + extra_distance as usize;

                // dbg!((extra_distance, distance));

                if neighbor == self.field.end() {
                    return Some(distance);
                }

                let new_node = Node::new(&neighbor, self.field);
                let new_node_distance = NodeDistance::new(new_node, distance);

                self.orphans.push(Reverse(new_node_distance));
                self.nodes.insert(neighbor);
            }
        } else {
            panic!("no orphans");
        }

        None
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    // location: Coord,
    neighbors: SmallVec<[Coord; 4]>,
}

impl Node {
    pub fn new(location: &Coord, field: &Field) -> Self {
        Self {
            // location: *location,
            neighbors: field.neighbors(location),
        }
    }
}

#[derive(Clone, Debug)]
pub struct NodeDistance {
    node: Node,
    distance: usize,
}

impl NodeDistance {
    pub fn new(node: Node, distance: usize) -> Self {
        Self {
            node,
            distance,
        }
    }
}


impl PartialEq for NodeDistance {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl PartialOrd for NodeDistance {
    fn gt(&self, other: &Self) -> bool {
        self.distance > other.distance
    }

    fn ge(&self, other: &Self) -> bool {
        self.distance >= other.distance
    }
    
    fn lt(&self, other: &Self) -> bool {
        self.distance < other.distance
    }

    fn le(&self, other: &Self) -> bool {
        self.distance <= other.distance
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl Eq for NodeDistance {}
impl Ord for NodeDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}