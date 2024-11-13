#[derive(Debug, Clone)]
pub enum Ty {
    Var(String),
    Func(Vec<Ty>, Box<Ty>), // TODO: support type variable constraints
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

pub enum Kind {
    Type,
    Arrow(Box<Kind>, Box<Kind>),
    // Type variable constraint goes here?!
}

/// The consumer of the library is free to map their concrete types
/// (e.g. 'f32', or 'String') to any Id they see fit - or they
/// can use the tyr::TypeMapper for convenience.
#[derive(Debug, Clone)]
pub struct TypeId(pub usize);
