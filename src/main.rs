use mathematician::MathExpr;

fn main() {
    let expr = "2^3^2/512+50*(2-5)".parse::<MathExpr>().unwrap();
    println!("{}", expr.eval());
    // Should print -149
}
