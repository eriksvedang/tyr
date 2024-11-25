#[cfg(test)]
mod tests {
    use crate::ty::*;
    use crate::unification_error::*;
    use crate::unifier::*;

    #[test]
    fn unify_var_with_concrete_type() {
        let mut unifier = Unifier::new();
        unifier.add_constraint(Constraint::new(Ty::var("a"), Ty::id(1)));
        assert!(unifier.solve().is_ok());
        assert_eq!(unifier.get_solved("a"), Some(&Ty::id(1)));
    }

    #[test]
    fn unify_var_with_concrete_type_other_direction() {
        let mut unifier = Unifier::new();
        unifier.add_constraint(Constraint::new(Ty::id(1), Ty::var("a")));
        assert!(unifier.solve().is_ok());
        assert_eq!(unifier.get_solved("a"), Some(&Ty::id(1)));
    }

    #[test]
    fn unify_vars_in_succession() {
        let mut unifier = Unifier::new();
        unifier.add_constraint(Constraint::new(Ty::var("a"), Ty::var("b")));
        unifier.add_constraint(Constraint::new(Ty::var("b"), Ty::var("c")));
        unifier.add_constraint(Constraint::new(Ty::var("c"), Ty::id(1)));
        assert!(unifier.solve().is_ok());
        assert_eq!(unifier.get_solved("a"), Some(&Ty::id(1)));
    }

    #[test]
    fn unify_vars_in_succession_reversed() {
        let mut unifier = Unifier::new();
        unifier.add_constraint(Constraint::new(Ty::var("c"), Ty::id(1)));
        unifier.add_constraint(Constraint::new(Ty::var("b"), Ty::var("c")));
        unifier.add_constraint(Constraint::new(Ty::var("a"), Ty::var("b")));
        assert!(unifier.solve().is_ok());
        assert_eq!(unifier.get_solved("a"), Some(&Ty::id(1)));
    }

    #[test]
    fn fail_to_unify_data_with_func() {
        let mut unifier = Unifier::new();
        unifier.add_constraint(Constraint::new(
            Ty::id(1),
            Ty::Func(Box::new(Ty::var("a")), Box::new(Ty::var("b"))),
        ));
        let result = unifier.solve();
        assert!(matches!(result, Err(UnificationError::CannotUnify(..))));
    }

    #[test]
    fn fail_to_unify_after_the_fact() {
        let mut unifier = Unifier::new();
        unifier.add_constraint(Constraint::new(Ty::var("a"), Ty::id(1)));
        unifier.add_constraint(Constraint::new(Ty::var("b"), Ty::id(2)));
        unifier.add_constraint(Constraint::new(Ty::var("a"), Ty::var("b")));
        let result = unifier.solve();
        assert!(matches!(result, Err(UnificationError::CannotUnify(..))));
    }

    #[test]
    fn fail_to_unify_before_the_fact() {
        let mut unifier = Unifier::new();
        unifier.add_constraint(Constraint::new(Ty::var("a"), Ty::var("b")));
        unifier.add_constraint(Constraint::new(Ty::var("a"), Ty::id(1)));
        unifier.add_constraint(Constraint::new(Ty::var("b"), Ty::id(2)));
        let result = unifier.solve();
        assert!(matches!(result, Err(UnificationError::CannotUnify(..))));
    }
}
