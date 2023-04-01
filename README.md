# Mathematician - Math parsing library

Simple library for parsing mathematical expression from a string. It doesn't require any dependency.

It supports only tokenization of integer values and the plus operator for now.

## Example

```rust
let mut lexer = Lexer::default();
assert_eq!(lexer.tokenize("12 + 3"), Ok(()));
lexer.tokens().for_each(|token| println!("{:?}", token));
// Will print:
// Integer(12)
// Plus
// Integer(3)
```

## Running

```bash
cargo run
```
