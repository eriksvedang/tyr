#![allow(dead_code)]

use crate::ty::Ty;

#[derive(Debug, Clone)]
pub struct Unifier {
    constraints: Vec<Constraint>,
}

impl Unifier {
    pub fn new(constraints: Vec<Constraint>) -> Self {
        Self { constraints }
    }
}

#[derive(Debug, Clone)]
pub struct Constraint {
    pub a: Ty,
    pub b: Ty,
}

impl Constraint {
    pub fn new(a: Ty, b: Ty) -> Self {
        Self { a, b }
    }
}
