#[cfg(test)]
mod tests {
    use crate::ty::*;
    use crate::unification_error::*;
    use crate::unifier::*;

    #[test]
    fn unify_var_with_concrete_type() {
        let mut unifier = Unifier::new();
        unifier.add_constraint(Constraint::new(
            Ty::Var("a".to_string()),
            Ty::Data(TypeId(1), vec![]),
        ));
        _ = unifier.solve();
        assert_eq!(unifier.get_solved("a"), Some(&Ty::Data(TypeId(1), vec![])));
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

    #[test]
    fn unify_vars_in_succession() {
        let mut unifier = Unifier::new();
        unifier.add_constraint(Constraint::new(
            Ty::Var("a".to_string()),
            Ty::Var("b".to_string()),
        ));
        unifier.add_constraint(Constraint::new(
            Ty::Var("b".to_string()),
            Ty::Var("c".to_string()),
        ));
        unifier.add_constraint(Constraint::new(
            Ty::Var("c".to_string()),
            Ty::Data(TypeId(1), vec![]),
        ));
        _ = unifier.solve();
        assert_eq!(unifier.get_solved("a"), Some(&Ty::Data(TypeId(1), vec![])));
    }

    #[test]
    fn fail_to_unify_data_with_func() {
        let mut unifier = Unifier::new();
        unifier.add_constraint(Constraint::new(
            Ty::Data(TypeId(0), vec![]),
            Ty::Func(
                Box::new(Ty::Var(String::from("a"))),
                Box::new(Ty::Var(String::from("b"))),
            ),
        ));
        let result = unifier.solve();
        assert!(matches!(result, Err(UnificationError::CannotUnify(..))));
    }
}
