use super::*;

pub fn parse_literal<'a> (lit: &'a str) -> impl Parser<'a, &str> {
    move |input: &'a str| match input.get(0..lit.len()) {
        Some(s) => 
            if s == lit {
                Ok((&input[lit.len()..], lit))
            } else {
                Err(ParseErr::new("Literal not found"))
            }
        _ => Err(ParseErr::new("Literal not found"))
    } 
}

pub fn parse_number<'a> () -> impl Parser<'a, f64> {
    move |input: &'a str| {
        let mut tok = String::new();
        let mut iter = input.chars();


        match iter.next() {
            Some(c) if c.is_alphanumeric() => tok.push(c),
            _ => return Err(ParseErr::new("non-alphanumeric character found. expected number."))
        }
        while let Some(c) = iter.next() {
            if c.is_digit(10) || c == '.' {
                tok.push(c);
            } else { break }
        }
        if let Ok(num) = tok.parse::<f64>() {
            Ok((&input[tok.len()..], num))
        } else {
            Err(ParseErr::new("failed to parse number"))
        }
    }
}

pub fn map<'a, A, B, P, F> (parser: P, functor: F) -> impl Parser<'a, B> 
where 
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |buf: &'a str| -> ParseRes<'a, B> {
        parser.parse(buf)
            .map(|(b, out): (&str, A)| (b, functor(out)))
    }
}
