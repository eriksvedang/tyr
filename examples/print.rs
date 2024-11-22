use tyr::ty::*;
use tyr::unifier::*;

fn main() {
    let mut unifier = Unifier::new();
    unifier.add_constraint(Constraint::new(
        Ty::Data(TypeId(0), vec![]),
        Ty::Func(
            Box::new(Ty::Var(String::from("a"))),
            Box::new(Ty::Var(String::from("b"))),
        ),
    ));
    let result = unifier.solve();

    match result {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    }
}
