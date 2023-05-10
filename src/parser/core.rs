use super::*;

pub fn and<'a, A, B, PA, PB> (a: PA, b: PB) -> impl Parser<'a, (A, B)> 
where
    PA: Parser<'a, A>,
    PB: Parser<'a, B>,
{
    move |buf: &'a str| -> ParseRes<'a, (A, B)> {
        a.parse(buf)
            .and_then(|(buf, res_a)| 
                b.parse(buf)
                    .map(|(buf, res_b)| (buf, (res_a, res_b) ))
            )
    }
}

pub fn zero_or_more<'a, A, P> (p: P) -> impl Parser<'a, Vec<A>> 
where
    P: Parser<'a, A>
{
    move |buf: &'a str| {
        let mut v = vec![];
        let mut buf_out = buf;
        while let Ok((buf, out)) = p.parse(buf_out) {
            v.push(out);

            buf_out = buf;
        }
        Ok((buf_out, v))
    }
}

pub fn one_or_more<'a, A, P> (p: P) -> impl Parser<'a, Vec<A>> 
where
    P: Parser<'a, A>
{
    move |buf: &'a str| {
        let mut v = vec![];
        let mut buf_out = buf;
        while let Ok((buf, out)) = p.parse(buf_out) {
            v.push(out);

            buf_out = buf;
        }
        if v.is_empty() {
            par_err("none of pattern found in 'one_or_more'")
        } else {
            Ok((buf_out, v))
        }
    }
}

pub fn prefix<'a, T, P> (s: &'a str, p: P) -> impl Parser<'a, T> 
where
    P: Parser<'a, T>
{
    map( 
        and(
            parse_literal(s),
            p
        ),
        |(_, a)| a
    )
}

pub fn suffix<'a, T, P> (s: &'a str, p: P) -> impl Parser<'a, T> 
where
    P: Parser<'a, T>
{
    map( 
        and(
            p,
            parse_literal(s) 
        ),
        |(a, _)| a
    )
}

pub fn surround<'a, T, P> (a: &'a str, b: &'a str, p: P) -> impl Parser<'a, T> 
where
    P: Parser<'a, T>
{
    prefix(
        a, 
        suffix(b, p) 
    )
}

pub fn parse_literal<'a> (lit: &'a str) -> impl Parser<'a, &str> {
    move |input: &'a str| match input.get(0..lit.len()) {
        Some(s) if s == lit => Ok((&input[lit.len()..], lit)),
        _ => {
            println!("{input}");
            par_err("Literal not found")
        }
    } 
}


pub fn parse_literals<'a> (lits: Vec<&'a str>) -> impl Parser<'a, &str> {
    move |buf: &'a str| -> ParseRes<'a, &str> {
        for lit in lits.iter() {
            match buf.get(0..lit.len()) {
                Some(s) if &s == lit => return Ok((&buf[lit.len()..], &buf[0..lit.len()])),
                _ => continue
            }
        }
        par_err("Literal Not Found")
    }
}

pub fn parse_tok_with_rule<'a, R> (rule: R) -> impl Parser<'a, String> 
where
    R: Fn (char) -> bool
{
    move |input: &'a str| {
        let mut tok = String::new();
        let mut iter = input.chars();

        match iter.next() {
            Some(c) if rule(c) => tok.push(c),
            _ => return par_err("First character does not satisfy rule")
        }
        while let Some(c) = iter.next() {
            if rule(c) {
                tok.push(c);
            } else { break }
        }
        if tok.is_empty() {
            par_err("Empty Token.")
        } else {
            Ok((&input[tok.len()..], tok))
        }
    }
}


pub fn parse_number<'a> () -> impl Parser<'a, f64> {
    move |input: &'a str| {
        let num_rule = |c: char| {
            c.is_ascii_digit() || c == '.'
        };
        let (buf, tok) = parse_tok_with_rule(num_rule).parse(input)?;
        if let Ok(num) = tok.parse::<f64>() {
            Ok((buf, num))
        } else {
            par_err("could not parse into number")
        }
    }
}

pub fn parse_identifier<'a> () -> impl Parser<'a, String> {
    move |input: &'a str| {
        let rule = |c: char| {
            c.is_alphanumeric() || c == '_'
        };
        let (buf, tok) = parse_tok_with_rule(rule).parse(input)?;

        if !tok.chars().next().unwrap().is_ascii_digit() {
            Ok((buf, tok))
        } else {
            par_err("identifier cannot start with digit")
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
