#![allow(dead_code)]

use std::collections::HashMap;

use crate::ty::Ty;

#[derive(Debug, Clone)]
pub struct Unifier {
    constraints: Vec<Constraint>,
    solved_variables: HashMap<String, Ty>,
}

impl Unifier {
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
            solved_variables: HashMap::new(),
        }
    }

    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }

    pub fn solve(&mut self) -> Result<(), ()> {
        // TODO: fix cloning
        for constraint in self.constraints.clone() {
            self.solve_one(&constraint.a, &constraint.b)?;
            self.solve_one(&constraint.b, &constraint.a)?;
        }

        Ok(())
    }

    fn solve_one(&mut self, a: &Ty, b: &Ty) -> Result<(), ()> {
        match a {
            Ty::Var(name) => _ = self.solved_variables.insert(name.to_string(), b.clone()),
            Ty::Func(_, _) => (),
            Ty::Data(_, _) => (),
        }

        Ok(())
    }

    pub fn get_solved(&self, variable_name: &str) -> Option<&Ty> {
        self.solved_variables.get(variable_name)
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

#[cfg(test)]
mod tests {
    use crate::ty::TypeId;

    use super::*;

    #[test]
    fn unify_var_with_concrete_type() {
        let mut unifier = Unifier::new();
        unifier.add_constraint(Constraint::new(
            Ty::Var("a".to_string()),
            Ty::Data(TypeId(0), vec![]),
        ));
        _ = unifier.solve();
        assert_eq!(unifier.get_solved("a"), Some(&Ty::Data(TypeId(0), vec![])));
    }

    #[test]
    fn unify_var_with_concrete_type_other_direction() {
        let mut unifier = Unifier::new();
        unifier.add_constraint(Constraint::new(
            Ty::Data(TypeId(0), vec![]),
            Ty::Var("a".to_string()),
        ));
        _ = unifier.solve();
        assert_eq!(unifier.get_solved("a"), Some(&Ty::Data(TypeId(0), vec![])));
    }
}
