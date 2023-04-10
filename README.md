# Mathematician - Math parsing library

Simple parser combinator library for parsing mathematical expression from a string. It doesn't require any dependency.

It supports basic operations (`+`, `-`, `*`, `/` and `^`) and integer numbers for now.

## Example

```rust
use mathematician::MathExpr;

fn main() {
    let expr = "2^3^2/512+50*(2-5)".parse::<MathExpr>().unwrap();
    println!("{}", expr.eval());
    // Should print -149
}
```

## Running

```bash
cargo run
```
