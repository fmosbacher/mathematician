use std::str::FromStr;

use crate::parsec::{choice, many, map, p_char, p_natural, seq, Either, ParseResult, Parser};

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
            p_natural,
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
pub struct Expr(i64);

impl Expr {
    pub fn eval(self) -> i64 {
        self.0
    }
}

impl FromStr for Expr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match p_expr.run(s) {
            Ok((integer, remaining)) => {
                if remaining.len() > 0 {
                    Err(format!(
                        "parsing failed while parsing remaining input `{}`",
                        remaining
                    ))
                } else {
                    Ok(Expr(integer))
                }
            }
            Err(err) => Err(err),
        }
    }
}
