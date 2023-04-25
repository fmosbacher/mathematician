use mathematician::eval;

fn main() {
    let input = "2^3^2/512+50*-(2-5)";
    let result = eval(input).unwrap();
    println!("{}", result);
    // Should print 151
}
