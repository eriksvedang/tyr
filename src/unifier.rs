#![allow(dead_code)]

use std::collections::HashMap;

use crate::{ty::Ty, unification_error::UnificationError};

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

        Ok(())
    }

    fn solve_one(vars: &mut HashMap<String, Ty>, a: &Ty, b: &Ty) -> Result<(), UnificationError> {
        match a {
            Ty::Var(a_name) => _ = vars.insert(a_name.to_string(), b.clone()),
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
