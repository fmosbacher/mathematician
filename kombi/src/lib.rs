#[derive(Debug)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub struct Parser<'a, T> {
    parse_fn: Box<dyn Fn(&'a str) -> Option<(T, &'a str)> + 'a>,
}

impl<'a, T: 'a> Parser<'a, T> {
    pub fn new(parse_fn: Box<dyn Fn(&'a str) -> Option<(T, &'a str)> + 'a>) -> Parser<'a, T> {
        Parser { parse_fn }
    }

    pub fn parse(&self, input: &'a str) -> Option<(T, &'a str)> {
        (self.parse_fn)(input)
    }

    pub fn map<U>(self, map_fn: impl Fn(T) -> U + 'a) -> Parser<'a, U> {
        Parser {
            parse_fn: Box::new(move |input| {
                self.parse(input)
                    .map(|(parsed, remaining)| (map_fn(parsed), remaining))
            }),
        }
    }
}

pub fn satisfies<'a, T: 'a>(
    parser: Parser<'a, T>,
    predicate: impl Fn(&T) -> bool + 'a,
) -> Parser<T> {
    Parser::new(Box::new(move |input| {
        parser.parse(input).and_then(|(parsed, remaining)| {
            if predicate(&parsed) {
                Some((parsed, remaining))
            } else {
                None
            }
        })
    }))
}

pub fn sequence<'a, T: 'a, U: 'a>(
    parser1: Parser<'a, T>,
    parser2: Parser<'a, U>,
) -> Parser<'a, (T, U)> {
    Parser::new(Box::new(move |input| {
        parser1.parse(input).and_then(|(parsed1, remaining1)| {
            parser2
                .parse(remaining1)
                .map(|(parsed2, remaining2)| ((parsed1, parsed2), remaining2))
        })
    }))
}

pub fn many<'a, T: 'a>(parser: Parser<'a, T>) -> Parser<'a, Vec<T>> {
    Parser::new(Box::new(move |mut input| {
        let mut results = vec![];
        while let Some((parsed, remaining)) = parser.parse(input) {
            input = remaining;
            results.push(parsed);
        }
        Some((results, input))
    }))
}

pub fn many1<'a, T: 'a>(parser: Parser<'a, T>) -> Parser<'a, Vec<T>> {
    Parser::new(Box::new(move |mut input| {
        let mut results = vec![];
        while let Some((parsed, remaining)) = parser.parse(input) {
            input = remaining;
            results.push(parsed);
        }
        if results.len() > 0 {
            Some((results, input))
        } else {
            None
        }
    }))
}

pub fn left<'a, T: 'a, U: 'a>(parser1: Parser<'a, T>, parser2: Parser<'a, U>) -> Parser<'a, T> {
    sequence(parser1, parser2).map(|(left, _)| left)
}

pub fn right<'a, T: 'a, U: 'a>(parser1: Parser<'a, T>, parser2: Parser<'a, U>) -> Parser<'a, U> {
    sequence(parser1, parser2).map(|(_, right)| right)
}

pub fn either<'a, T: 'a, U: 'a>(
    parser1: Parser<'a, T>,
    parser2: Parser<'a, U>,
) -> Parser<'a, Either<T, U>> {
    Parser::new(Box::new(move |input| match parser1.parse(input) {
        Some((parsed, remaining)) => Some((Either::Left(parsed), remaining)),
        _ => parser2
            .parse(input)
            .map(|(parsed, remaining)| (Either::Right(parsed), remaining)),
    }))
}

pub fn any_character() -> Parser<'static, char> {
    Parser::new(Box::new(|input| {
        input
            .chars()
            .next()
            .map(|first_char| (first_char, &input[first_char.len_utf8()..]))
    }))
}

pub fn lazy<'a, T: 'a>(parser_generator: impl Fn() -> Parser<'a, T> + 'a) -> Parser<'a, T> {
    Parser::new(Box::new(move |input| parser_generator().parse(input)))
}

pub fn character(search_char: char) -> Parser<'static, char> {
    satisfies(any_character(), move |first_char| {
        *first_char == search_char
    })
}

pub fn digit() -> Parser<'static, char> {
    satisfies(any_character(), move |first_char| {
        ('0'..='9').contains(first_char)
    })
}

pub fn whitespace() -> Parser<'static, char> {
    satisfies(any_character(), |first_char| *first_char == ' ')
}

pub fn whitespaces() -> Parser<'static, Vec<char>> {
    many(whitespace())
}

pub fn whitespaces1() -> Parser<'static, Vec<char>> {
    many1(whitespace())
}
