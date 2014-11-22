use ord::InvOrd;
use std::collections::{BinaryHeap, HashMap, Bitv};

pub type Weight = uint;

#[deriving(Show, PartialEq, Eq, Hash)]
pub enum Node<T: Ord + Copy> {
    Leaf(T, Weight),
    Branch(Box<Node<T>>, Box<Node<T>>),
}

impl<T: Ord + Copy> Node<T> {
    pub fn weight(&self) -> Weight {
        match *self {
            Node::Leaf(_, weight) => weight,
            Node::Branch(ref left, ref right) => left.weight() + right.weight(),
        }
    }
}

impl<T: Ord + Copy> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Node<T>) -> Option<::std::cmp::Ordering> {
        self.weight().partial_cmp(&other.weight())
    }
}

impl<T: Ord + Copy> Ord for Node<T> {
    fn cmp(&self, other: &Node<T>) -> ::std::cmp::Ordering {
        self.weight().cmp(&other.weight())
    }
}

pub fn make_code<T: Ord + Copy>(weights: &[(T, Weight)]) -> Option<Node<T>> {
    let mut queue = BinaryHeap::new();

    for &(value, weight) in weights.iter() {
        queue.push(InvOrd(Node::Leaf(value, weight)));
    }

    while queue.len() > 1 {
        let left = queue.pop();
        let right = queue.pop();

        match (left, right) {
            (Some(left), Some(right)) => {
                let branch = Node::Branch(box left.get(), box right.get());
                queue.push(InvOrd(branch));
            }
            _ => panic!(),
        }
    }

    queue.pop().map(|node| node.get())
}

pub fn calculate_weights<T: Eq + ::std::hash::Hash + Copy>(items: &[T]) -> Vec<(T, Weight)> {
    let mut weights = HashMap::new();

    for &item in items.iter() {
        let weight = match weights.get(&item) {
            Some(&weight) => weight,
            None => 0,
        };
        weights.insert(item, weight + 1);
    }

    weights.into_iter().collect()
}
