#![allow(dead_code)]

use crate::{ty::Ty, unification_error::UnificationError};
use log::*;
use std::collections::{HashMap, HashSet};

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

    pub fn solve(&mut self) -> Result<(), UnificationError> {
        for constraint in &self.constraints {
            Self::solve_one(&mut self.solved_variables, &constraint.a, &constraint.b)?;
            Self::solve_one(&mut self.solved_variables, &constraint.b, &constraint.a)?;
        }

        debug!("Done solving: {:#?}", self.solved_variables);

        Ok(())
    }

    fn solve_one(vars: &mut HashMap<String, Ty>, a: &Ty, b: &Ty) -> Result<(), UnificationError> {
        match a {
            Ty::Var(a_name) => {
                vars.insert(a_name.clone(), b.clone());
            }
            Ty::Func(a_in, a_out) => match b {
                Ty::Var(_) => (),
                Ty::Func(b_in, b_out) => {
                    Self::solve_one(vars, a_in, b_in)?;
                    Self::solve_one(vars, a_out, b_out)?;
                }
                Ty::Data(_, _) => return Err(UnificationError::CannotUnify(a.clone(), b.clone())),
            },
            Ty::Data(a_type_id, a_type_parameters) => match b {
                Ty::Var(_) => (),
                Ty::Func(_, _) => return Err(UnificationError::CannotUnify(a.clone(), b.clone())),
                Ty::Data(b_type_id, b_type_parameters) => {
                    if a_type_id == b_type_id {
                        for (a_parameter, b_parameter) in
                            a_type_parameters.iter().zip(b_type_parameters.iter())
                        {
                            Self::solve_one(vars, a_parameter, b_parameter)?;
                            Self::solve_one(vars, b_parameter, a_parameter)?;
                        }
                    } else {
                        return Err(UnificationError::CannotUnify(a.clone(), b.clone()));
                    }
                }
            },
        }

        Ok(())
    }

    /// Type variables are resolved by following their chain of associations in `vars`.
    /// Non-type variables just resolve to themselves.
    fn fully_resolve<'a>(vars: &'a HashMap<String, Ty>, type_variable: &'a Ty) -> &'a Ty {
        if !type_variable.is_var() {
            return type_variable;
        }

        let mut visited = HashSet::new();
        let mut resolved_to = type_variable;
        visited.insert(resolved_to);

        loop {
            match resolved_to {
                Ty::Var(var_name) => {
                    if let Some(found) = vars.get(var_name) {
                        if visited.contains(found) {
                            return found;
                        } else {
                            visited.insert(found);
                            resolved_to = found;
                        }
                    } else {
                        return resolved_to;
                    }
                }
                non_variable => return non_variable,
            }
        }
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
