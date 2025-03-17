#[cfg(test)]
mod tests {

    use sysyust_visitor_template::Transformable;

    #[derive(Transformable)]
    pub enum Expr {
        Add(AddExpr),
        Atom(AtomVal),
    }

    pub struct AtomVal {
        val: i64,
    }
    pub struct AddExpr {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    }

    struct FoldTransformer {

    }
    impl ExprTransformer for FoldTransformer {
        fn transform_add(&mut self, param: AddExpr) -> Expr {
            let AddExpr { lhs, rhs } = param;

            let mut ans = 0;
            if let Expr::Atom(lhs) = lhs.transform(self) {
                ans += lhs.val;
            } else {
                panic!("Unexpected");
            }

            if let Expr::Atom(rhs) = rhs.transform(self) {
                ans += rhs.val;
            } else {
                panic!("Unexpected");
            }

            return Expr::Atom(AtomVal { val: ans });
        }
    }

    #[test]
    fn test_transform() {
        let expr = Expr::Add(AddExpr{
            lhs: Box::new(Expr::Add(AddExpr{
                lhs: Box::new(Expr::Atom(AtomVal{val: 32})),
                rhs: Box::new(Expr::Atom(AtomVal{val: 32})),
            })),
            rhs: Box::new(Expr::Atom(AtomVal{val: 1})),
        });

        let mut tt = FoldTransformer{};
        if let Expr::Atom(ans) = expr.transform(&mut tt) {
            assert_eq!(ans.val, 65);
        } else {
            panic!("unpected value");
        }
    }
}