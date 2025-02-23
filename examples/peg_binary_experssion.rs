enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}


peg::parser!{
    grammar binary_experssion_parser() for str {
        pub rule expr() -> u32
            = lhs:term() o:expr_op() rhs:expr() {?
                match o {
                    BinaryOp::Add => Ok(lhs + rhs),
                    BinaryOp::Sub => Ok(lhs - rhs),
                    _ => Err("Not add expr"),
                }
            } 
            / n:term() {n}

        rule factor() -> u32
            = "(" n:expr() ")" {n}
            / n:number() {n}
        rule term() -> u32
            = lhs:factor() o:term_op() rhs:term() {?
                match o {
                    BinaryOp::Mul => Ok(lhs * rhs),
                    BinaryOp::Div => Ok(lhs / rhs),
                    BinaryOp::Mod => Ok(lhs % rhs),
                    _ => Err("Not mul expr"),
                }
            }
            / n:factor() {n}


        rule number() -> u32
            = n: $(['0'..='9']+) {? println!("There is {}", n);  n.parse().or(Err("Not U32"))}
        rule term_op() -> BinaryOp
            = "*" {BinaryOp::Mul} 
            / "/" {BinaryOp::Div} 
            / "%" {BinaryOp::Mod}
        rule expr_op() -> BinaryOp
            = "+" {BinaryOp::Add} 
            / "-" {BinaryOp::Sub}

    }
}


fn main() {
    assert_eq!(binary_experssion_parser::expr("5+(1+1)*2+3").unwrap(), 5+(1+1)*2+3);
}