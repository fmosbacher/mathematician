use mathematician::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::default();
    assert_eq!(lexer.tokenize("2+2+-365*/"), Ok(()));
    lexer.tokens().for_each(|token| println!("{:?}", token));
}
