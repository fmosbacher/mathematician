# Mathematician - Math parsing library

Simple parser combinator library for parsing mathematical expression from a string. It doesn't require any dependency.

It supports basic operations `(+, -, *, /)` and integer numbers for now.

## Example

```rust
use mathematician::MathExpr;

fn main() {
    let expr = "(3+5)/4*(27/3/3)".parse::<MathExpr>().unwrap();
    println!("{}", expr.eval());
    // Should print 6
}
```

## Running

```bash
cargo run
```
