#[derive(Debug, Clone)]
pub enum Ty {
    Var(String),
    Function(Vec<Ty>, Box<Ty>), // TODO: support type variable constraints
    Parameterized(TypeId, Vec<Ty>),
    Concrete(TypeId),
}

impl Ty {
    pub fn kind(&self) -> Kind {
        todo!()
    }
}

pub enum Kind {
    Type,
    Arrow(Box<Kind>, Box<Kind>),
    // Type variable constraint goes here?!
}

/// The consumer of the library is free to map their concrete types
/// (e.g. 'i32', or 'String') to any Id they see fit - or they
/// can use the tyr::TypeMapper for convenience.
#[derive(Debug, Clone)]
pub struct TypeId(pub usize);
