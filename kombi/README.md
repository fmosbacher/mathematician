# Kombi

Parser combinator library. Implements multiple methods to build complex parsers.

## Example with simple HTML tag

```rust
use kombi::*;

#[derive(Debug)]
struct Tag {
    name: String,
    unique_attr: (String, String),
}

fn tag() -> Parser<'static, Tag> {
    let tag_name = right(
        character('<'),
        many1(lower()).map(|letters| letters.iter().collect::<String>()),
    );
    let attr_name = right(
        whitespaces1(),
        many1(lower()).map(|letters| letters.iter().collect::<String>()),
    );
    let not_dquotes = many1(satisfies(any_character(), |first_char| *first_char != '"'))
        .map(|chars| chars.iter().collect::<String>());
    let attr_data = right(
        literal("=\""),
        left(
            not_dquotes,
            sequence(sequence(character('"'), whitespaces()), literal("/>")),
        ),
    );
    sequence(sequence(tag_name, attr_name), attr_data).map(|((tag_name, attr_name), attr_data)| {
        Tag {
            name: tag_name,
            unique_attr: (attr_name, attr_data),
        }
    })
}

fn main() {
    let input = r#"<tag attr="something here" />"#;
    let result = tag().parse(input).unwrap();
    println!("{:?}", result);
    // Should print (Tag { name: "tag", unique_attr: ("attr", "something here") }, "")
}

```

## Usage

```bash
cargo build
```
