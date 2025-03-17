#[cfg(test)]
mod tests {
    use sysyust_visitor_template::Transformable;

    #[derive(Transformable)]
    pub enum Expr {
        Add(Box<Expr>, Box<Expr>),
        AtomExpr(i32),
    }

    struct ExprFoldTransformer {

    }

    impl ExprTransformer for ExprFoldTransformer {
        fn transform_add(&mut self, lhs: Box<Expr>, rhs: Box<Expr>) -> Expr {
            let fold_lhs = lhs.transform(self);
            let fold_rhs = rhs.transform(self);

            match (fold_lhs, fold_rhs) {
                (Expr::AtomExpr(lhs_val), Expr::AtomExpr(rhs_val)) => Expr::AtomExpr(lhs_val + rhs_val),
                _ => panic!("Unexpected transform"),
            }
        }
    }

    #[test]
    fn test_apply() {
        let example_expr = Expr::Add(Box::new(Expr::AtomExpr(1)), Box::new(Expr::AtomExpr(2)));
        let mut folder = ExprFoldTransformer{};
        let ans_expr = example_expr.transform(&mut folder);
        let ans_value = match ans_expr {
            Expr::AtomExpr(lhs) => lhs,
            _ => panic!("Unexpected transform"),
        };

        assert_eq!(ans_value, 3);

    }
}