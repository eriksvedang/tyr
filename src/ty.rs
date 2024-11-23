use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    Var(String),
    Func(Box<Ty>, Box<Ty>), // TODO: support type variable constraints
    Data(TypeId, Vec<Ty>),
}

impl Ty {
    pub fn is_var(&self) -> bool {
        matches!(self, Ty::Var(..))
    }

    pub fn kind(&self) -> Option<Kind> {
        match self {
            Ty::Var(_) => None,
            Ty::Func(_, _) => Some(Kind::Type), // ?
            Ty::Data(_, _) => todo!(),
        }
    }
}

impl Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ty::Var(name) => write!(f, "{}", name),
            Ty::Func(in_ty, out_ty) => write!(f, "{in_ty} -> {out_ty}"),
            Ty::Data(type_index, type_parameters) => {
                write!(f, "{type_index:?}")?;
                for ty_param in type_parameters {
                    write!(f, " {ty_param}")?;
                }
                Ok(())
            }
        }
    }
}

pub enum Kind {
    Type,
    Arrow(Box<Kind>, Box<Kind>),
    // Type variable constraint goes here?!
}

/// The consumer of the library is free to map their concrete types
/// (e.g. 'f32', or 'String') to any Id they see fit - or they
/// can use the tyr::TypeMapper for convenience.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeId(pub usize);
