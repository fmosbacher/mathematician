#[derive(Debug)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub type ParseResult<T> = Result<(T, String), String>;

pub trait Parser<T> {
    fn run(&self, input: &str) -> ParseResult<T>;
}

impl<P, T> Parser<T> for P
where
    P: Fn(&str) -> ParseResult<T>,
{
    fn run(&self, input: &str) -> ParseResult<T> {
        self(input)
    }
}

pub fn p_char(search_char: char) -> impl Parser<char> {
    move |input: &str| match input.chars().nth(0) {
        Some(first_char) => {
            let remaining = input.get(1..).expect("tail of string with length > 0");
            if first_char == search_char {
                Ok((first_char, remaining.into()))
            } else {
                Err(format!(
                    "char `{}` not found at start of input",
                    search_char
                ))
            }
        }
        None => Err("trying to parse empty string".into()),
    }
}

pub fn choice<T, U>(p1: impl Parser<T>, p2: impl Parser<U>) -> impl Parser<Either<T, U>> {
    move |input: &str| {
        p1.run(input).map_or_else(
            |_| {
                p2.run(input)
                    .map(|(parsed, remaining)| (Either::Right(parsed), remaining))
            },
            |(parsed, remaining)| Ok((Either::Left(parsed), remaining)),
        )
    }
}

pub fn seq<T, U>(p1: impl Parser<T>, p2: impl Parser<U>) -> impl Parser<(T, U)> {
    move |input: &str| {
        p1.run(input).and_then(|(parsed1, remaining1)| {
            p2.run(remaining1.as_str())
                .and_then(|(parsed2, remaining2)| Ok(((parsed1, parsed2), remaining2)))
        })
    }
}

pub fn map<T, U>(map_fn: impl Fn(T) -> U, p: impl Parser<T>) -> impl Parser<U> {
    move |input: &str| match p.run(input) {
        Ok((parsed, remaining)) => Ok((map_fn(parsed), remaining)),
        Err(err) => Err(err),
    }
}

pub fn many<T>(p: impl Parser<T>) -> impl Parser<Vec<T>> {
    move |input: &str| {
        let mut acc = vec![];
        let mut input = input.to_string();
        while let Ok((parsed, remaining)) = p.run(input.as_str()) {
            acc.push(parsed);
            input = remaining;
        }
        Ok((acc, input))
    }
}

pub fn p_digit(input: &str) -> ParseResult<char> {
    for p in ('0'..='9').map(p_char) {
        let result = p.run(input);
        if result.is_ok() {
            return result;
        }
    }
    Err("digit not found at start of input".into())
}

pub fn p_natural_number(input: &str) -> ParseResult<u32> {
    let mut acc = String::new();
    let mut input = input.to_string();
    while let Ok((parsed, remaining)) = p_digit.run(input.as_str()) {
        acc.push(parsed);
        input = remaining;
    }
    match acc.parse::<u32>() {
        Ok(ok) => Ok((ok, input.into())),
        Err(..) => Err("cannot find any integer".into()),
    }
}
