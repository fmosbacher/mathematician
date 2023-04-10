use mathematician::MathExpr;

fn main() {
    let expr = "(1+9)*5".parse::<MathExpr>().unwrap();
    println!("{:?}", expr.eval());
    // Should print 50
}
