use kombi::{character, digit, either, lazy, left, many, many1, right, sequence, Either, Parser};

fn positive_number() -> Parser<'static, f64> {
    let to_string = |chars: Vec<char>| chars.iter().collect::<String>();
    let integer = many1(digit())
        .map(to_string)
        .map(|digits| digits.parse::<f64>().unwrap());
    let float = sequence(
        many1(digit()).map(to_string),
        sequence(character('.'), many1(digit()).map(to_string)),
    )
    .map(|(left, (dot, right))| format!("{}{}{}", left, dot, right).parse::<f64>().unwrap());
    either(float, integer).map(|result| match result {
        Either::Left(float) => float,
        Either::Right(integer) => integer,
    })
}

fn expr() -> Parser<'static, f64> {
    either(
        sequence(
            term(),
            many(sequence(either(character('+'), character('-')), term())),
        ),
        term(),
    )
    .map(|term| match term {
        Either::Left((lhs, pairs)) => pairs.iter().fold(lhs, |acc, (op, rhs)| match op {
            Either::Left(_add) => acc + rhs,
            Either::Right(_sub) => acc - rhs,
        }),
        Either::Right(factor) => factor,
    })
}

fn term() -> Parser<'static, f64> {
    either(
        sequence(
            factor(),
            many(sequence(either(character('*'), character('/')), factor())),
        ),
        factor(),
    )
    .map(|term| match term {
        Either::Left((lhs, pairs)) => pairs.iter().fold(lhs, |acc, (op, rhs)| match op {
            Either::Left(_mul) => acc * rhs,
            Either::Right(_div) => acc / rhs,
        }),
        Either::Right(factor) => factor,
    })
}

fn factor() -> Parser<'static, f64> {
    either(sequence(exp(), many(right(character('^'), exp()))), exp()).map(|term| match term {
        Either::Left((head, tail)) => head.powf(tail.iter().rev().fold(1.0, |acc, x| x.powf(acc))),
        Either::Right(factor) => factor,
    })
}

fn exp() -> Parser<'static, f64> {
    lazy(|| {
        either(
            either(
                positive_number(),
                right(character('('), left(expr(), character(')'))),
            ),
            right(character('-'), expr()).map(|expr| -expr),
        )
        .map(|exp| match exp {
            Either::Left(pos_or_parentesis) => match pos_or_parentesis {
                Either::Left(positive_number) => positive_number,
                Either::Right(expr) => expr,
            },
            Either::Right(negative_expr) => negative_expr,
        })
    })
}

pub fn eval(input: &'static str) -> Result<f64, String> {
    let result = expr().parse(input);
    match result {
        Some((parsed, remaining)) => {
            if remaining.len() > 0 {
                Err("could not parse input".into())
            } else {
                Ok(parsed)
            }
        }
        None => Err("could not parse input".into()),
    }
}
