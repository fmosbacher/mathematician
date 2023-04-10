use mathematician::MathExpr;

fn main() {
    let expr = "(3+5)/4*(27/3/3)".parse::<MathExpr>().unwrap();
    println!("{}", expr.eval());
    // Should print 6
}
