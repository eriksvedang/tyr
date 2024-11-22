use crate::ty::Ty;
use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum UnificationError {
    CannotUnify(Ty, Ty),
}

impl Error for UnificationError {}

impl Display for UnificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnificationError::CannotUnify(a, b) => {
                write!(f, "Can't unify '{a}' with '{b}'.")
            }
        }
    }
}
