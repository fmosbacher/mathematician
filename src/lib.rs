use kombi::{character, digit, either, lazy, left, many, many1, right, sequence, Either, Parser};

fn integer() -> Parser<'static, i32> {
    let string_to_integer =
        |digits: Vec<char>| digits.iter().collect::<String>().parse::<i32>().unwrap();
    let negative = right(character('-'), many1(digit())).map(string_to_integer);
    let positive = many1(digit()).map(string_to_integer);
    either(negative, positive).map(|integer| match integer {
        Either::Left(negative) => -negative,
        Either::Right(positive) => positive,
    })
}

fn expr() -> Parser<'static, i32> {
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

fn term() -> Parser<'static, i32> {
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

fn factor() -> Parser<'static, i32> {
    lazy(|| {
        either(
            integer(),
            right(character('('), left(expr(), character(')'))),
        )
    })
    .map(|factor| match factor {
        Either::Left(integer) => integer,
        Either::Right(expr) => expr,
    })
}

pub fn eval(input: &'static str) -> Result<i32, String> {
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
