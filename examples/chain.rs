use tyr::ty::*;
use tyr::unifier::*;

fn main() {
    env_logger::init();

    let mut unifier = Unifier::new();

    unifier.add_constraint(Constraint::new(
        Ty::Var("b".to_string()),
        Ty::Var("c".to_string()),
    ));
    unifier.add_constraint(Constraint::new(
        Ty::Var("b".to_string()),
        Ty::Data(TypeId(1), vec![]),
    ));
    unifier.add_constraint(Constraint::new(
        Ty::Var("c".to_string()),
        Ty::Data(TypeId(1), vec![]),
    ));
    unifier.add_constraint(Constraint::new(
        Ty::Var("a".to_string()),
        Ty::Var("b".to_string()),
    ));

    let result = unifier.solve();

    match result {
        Ok(_) => println!("OK."),
        Err(err) => println!("{}", err),
    }
}
