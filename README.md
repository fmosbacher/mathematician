# Mathematician

Math library that uses [kombi](https://github.com/fmosbacher/mathematician/tree/main/kombi), a parser combinator library, to parse and evaluate math expressions.

It supports basic operations (`+`, `-`, `*`, `/` and `^`) with float numbers.

## Example

```rust
use mathematician::eval;

fn main() {
    let input = "2^3^2/512+50*-(2-5)";
    let result = eval(input).unwrap();
    println!("{}", result);
    // Should print 151
}
```

## Usage

There is a binary crate with an example. You can just run it.

```bash
cargo run
```
