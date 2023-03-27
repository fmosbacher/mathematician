# Mathematician - Math parsing library

Simple library for parsing mathematical expression from a string.

It supports only integers values for now.

## Example

```rust
let input = "3 * 3 ^ (2 + 1)";
let math_expr: MathExpr = input.parse().unwrap();
assert_eq!(math_expr.eval(), 81);
```

## Running

```bash
cargo run
```

## To do

- Add
- Subtract
- Parentheses
- Multiply
- Divide
- Decimal numbers
- Pow
- Improve error handling types
- Overflow handling
- Log
