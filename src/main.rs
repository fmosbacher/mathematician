use mathematician::Lexer;

fn main() {
    let mut lexer = Lexer::default();
    assert_eq!(lexer.tokenize("12 + 3"), Ok(()));
    lexer.tokens().for_each(|token| println!("{:?}", token));
    // Will print:
    // Integer(12)
    // Plus
    // Integer(3)
}
