#![feature(box_patterns, box_syntax)]

use std::mem;

pub mod parse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    /// Note: `a Sub b` is implemented as `a Add (Neg b)`
    Add,
    Div,
    Mul,
    Eq,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Neg,
}

pub enum Type {
    Bool,
    Integer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Constant(String),
    BinOp(BinOp, Box<Node>, Box<Node>),
    UnaryOp(UnOp, Box<Node>),
    IntValue(u128),
    BoolValue(bool),
}

impl Node {
    pub fn reduce(&mut self) {
        let this = mem::replace(self, Node::IntValue(0));
        match self {
            Node::Constant(_) | Node::IntValue(_) | Node::BoolValue(_) => (),
            Node::BinOp(_, lhs, rhs) => {
                lhs.reduce();
                rhs.reduce();
            }
            Node::UnaryOp(_, val) => {
                val.reduce();
            }
        }

        *self = match this {
            Node::UnaryOp(UnOp::Neg, box Node::UnaryOp(UnOp::Neg, box val)) => val,
            Node::BinOp(BinOp::Add, a, b) if *a == *b => {
                Node::BinOp(BinOp::Mul, a, box Node::IntValue(2))
            }
            Node::BinOp(BinOp::Eq, a, b) if *a == *b => {
                Node::BoolValue(true)
            }
            this => this,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use parse::parse;
    #[track_caller]
    fn reduce_to(l: &str, r: &str) {
        let mut ln = parse(l).unwrap();
        let rn = parse(r).unwrap();
        ln.reduce();
        assert_eq!(ln, rn);
    }

    #[test]
    fn reduce_test() {
        reduce_to("pi--", "pi");
        reduce_to("a a+", "a 2*");
        reduce_to("a a=", "true");
    }
}
