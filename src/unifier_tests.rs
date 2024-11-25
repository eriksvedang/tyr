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
    fn unify_really_long_chain() {
        let mut unifier = Unifier::new();
        unifier.add_constraint(Constraint::new(Ty::var("e"), Ty::var("f")));
        unifier.add_constraint(Constraint::new(Ty::var("a"), Ty::var("b")));
        unifier.add_constraint(Constraint::new(Ty::var("f"), Ty::var("g")));
        unifier.add_constraint(Constraint::new(Ty::var("c"), Ty::var("d")));
        unifier.add_constraint(Constraint::new(Ty::var("h"), Ty::id(1)));
        unifier.add_constraint(Constraint::new(Ty::var("b"), Ty::var("c")));
        unifier.add_constraint(Constraint::new(Ty::var("g"), Ty::var("h")));
        unifier.add_constraint(Constraint::new(Ty::var("d"), Ty::var("e")));
        assert!(unifier.solve().is_ok());
        assert_eq!(unifier.get_solved("a"), Some(&Ty::id(1)));
    }

    #[test]
    fn unify_binary_functions() {
        let mut unifier = Unifier::new();
        // Unify 'a -> #2 -> c' with '#1 -> b -> #3'
        unifier.add_constraint(Constraint::new(
            Ty::fun(Ty::var("a"), Ty::fun(Ty::id(2), Ty::var("c"))),
            Ty::fun(Ty::id(1), Ty::fun(Ty::var("b"), Ty::id(3))),
        ));
        assert!(unifier.solve().is_ok());
        assert_eq!(unifier.get_solved("a"), Some(&Ty::id(1)));
        assert_eq!(unifier.get_solved("b"), Some(&Ty::id(2)));
        assert_eq!(unifier.get_solved("c"), Some(&Ty::id(3)));
    }

    #[test]
    fn unify_data_type_args() {
        let mut unifier = Unifier::new();
        // Unify '(#1 a #3)' with '(#1 #2 b)'
        unifier.add_constraint(Constraint::new(
            Ty::Data(TypeId(1), vec![Ty::var("a"), Ty::id(3)]),
            Ty::Data(TypeId(1), vec![Ty::id(2), Ty::var("b")]),
        ));
        assert!(unifier.solve().is_ok());
        assert_eq!(unifier.get_solved("a"), Some(&Ty::id(2)));
        assert_eq!(unifier.get_solved("b"), Some(&Ty::id(3)));
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
