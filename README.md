# Mathematician - Math parsing library

Simple parser combinator library for parsing mathematical expression from a string. It doesn't require any dependency.

It suports basic operations (+, -, \*, /) and integer numbers for now.

## Example

```rust
use mathematician::MathExpr;

fn main() {
    let expr = "(1+9)*5".parse::<MathExpr>().unwrap();
    println!("{:?}", expr.eval());
    // Should print 50
}

```

## Running

```bash
cargo run
```
