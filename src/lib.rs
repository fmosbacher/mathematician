use std::str::FromStr;

#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

type ParseResult<T> = Result<(T, String), String>;

trait Parser<T> {
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

fn p_char(search_char: char) -> impl Parser<char> {
    move |input: &str| {
        if let Some(head) = input.chars().nth(0) {
            if head == search_char {
                Ok((
                    head,
                    input
                        .get(1..)
                        .expect("tail of string with length > 0")
                        .into(),
                ))
            } else {
                Err(format!(
                    "char `{}` not found at start of input",
                    search_char
                ))
            }
        } else {
            Err("trying to parse empty string".into())
        }
    }
}

fn choice<T, U>(p1: impl Parser<T>, p2: impl Parser<U>) -> impl Parser<Either<T, U>> {
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

fn seq<T, U>(p1: impl Parser<T>, p2: impl Parser<U>) -> impl Parser<(T, U)> {
    move |input: &str| {
        p1.run(input).and_then(|(parsed1, remaining1)| {
            p2.run(remaining1.as_str())
                .and_then(|(parsed2, remaining2)| Ok(((parsed1, parsed2), remaining2)))
        })
    }
}

fn map<T, U>(map_fn: impl Fn(T) -> U, parser: impl Parser<T>) -> impl Parser<U> {
    move |input: &str| match parser.run(input) {
        Ok((parsed, remaining)) => Ok((map_fn(parsed), remaining)),
        Err(err) => Err(err),
    }
}

fn many<T>(parser: impl Parser<T>) -> impl Parser<Vec<T>> {
    move |input: &str| {
        let mut acc = vec![];
        let mut input = input.to_string();
        while let Ok((parsed, remaining)) = parser.run(input.as_str()) {
            acc.push(parsed);
            input = remaining;
        }
        Ok((acc, input))
    }
}

fn p_digit(input: &str) -> ParseResult<char> {
    for parser in ('0'..='9').map(p_char) {
        let result = parser.run(input);
        if result.is_ok() {
            return result;
        }
    }
    Err("digit not found at start of input".into())
}

fn p_positive_integer(input: &str) -> ParseResult<u32> {
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

fn p_expr(input: &str) -> ParseResult<i64> {
    // expr: term { ("+" | "-") term }*
    map(
        |(a, pairs)| {
            pairs.iter().fold(a, |acc, (op, b)| match op {
                Either::Left(_add) => acc + b,
                Either::Right(_sub) => acc - b,
            })
        },
        seq(p_term, many(seq(choice(p_char('+'), p_char('-')), p_term))),
    )
    .run(input)
}

fn p_term(input: &str) -> ParseResult<i64> {
    // term: factor { ("*" | "/") factor }*
    map(
        |(a, pairs)| {
            pairs.iter().fold(a, |acc, (op, b)| match op {
                Either::Left(_mul) => acc * b,
                Either::Right(_div) => acc / b,
            })
        },
        seq(
            p_factor,
            many(seq(choice(p_char('*'), p_char('/')), p_factor)),
        ),
    )
    .run(input)
}

fn p_factor(input: &str) -> ParseResult<i64> {
    // factor: power { "^" power }*
    map(
        |(a, pairs)| {
            let partial = pairs.iter().rfold(1, |acc, (_, b)| (*b as u32).pow(acc));
            a.pow(partial)
        },
        seq(p_power, many(seq(p_char('^'), p_power))),
    )
    .run(input)
}

fn p_power(input: &str) -> ParseResult<i64> {
    // power: INTEGER | "(" expr ")" | "-" factor
    map(
        |result| match result {
            Either::Left(num) => num as i64,
            Either::Right(result) => match result {
                Either::Left(num) => num,
                Either::Right(num) => num,
            },
        },
        choice(
            p_positive_integer,
            choice(
                map(
                    |(_, (expr, _))| expr,
                    seq(p_char('('), seq(p_expr, p_char(')'))),
                ),
                map(|(_, factor)| -factor, seq(p_char('-'), p_factor)),
            ),
        ),
    )
    .run(input)
}

#[derive(Debug)]
pub struct MathExpr(i64);

impl MathExpr {
    pub fn eval(self) -> i64 {
        self.0
    }
}

impl FromStr for MathExpr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = p_expr.run(s);
        match res {
            Ok((integer, remaining)) => {
                if remaining.len() > 0 {
                    Err("could not parse to the end of the input".into())
                } else {
                    Ok(MathExpr(integer))
                }
            }
            Err(err) => Err(err),
        }
    }
}
