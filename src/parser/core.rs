use super::*;


pub fn bind<F, T>(f: F) -> F
where
    F: for<'a> Fn(&'a str) -> ParseRes<'a, T>,
{
    f
}
fn bind2<'t, F, T>(f: F) -> F
where
    F: Fn(&'t str) -> ParseRes<'t, T>,
{
    f
}

pub fn option<T, P> (p: P) -> impl Parser<Option<T>>
where 
    P: Parser<T>
{
    bind(move |buf: & str| {
        if let Ok((buf, o)) = p.parse(buf) {
            Ok((buf, Some(o)))
        } else {
            Ok((buf, None))
        }
    })
}

pub fn and<'a, A, B, PA, PB> (a: PA, b: PB) -> impl Parser<(A, B)> 
where
    PA: Parser<A>,
    PB: Parser<B>,
{
    bind(move |buf| {
        a.parse(buf)
            .and_then(|(buf, res_a)| 
                b.parse(buf)
                    .map(|(buf, res_b)| (buf, (res_a, res_b) ))
            )
    })
}

pub fn or<'a, P1, P2, A>(parser1: P1, parser2: P2) -> impl Parser<A>
where
    P1: Parser<A>,
    P2: Parser<A>,
{
    bind(move |input| match parser1.parse(input) {
        ok @ Ok(_) => ok,
        Err(e) => {
            println!("option one failed with: {:?}", e);
            parser2.parse(input)
        },
    })
} 


pub fn zero_or_more<'a, A, P> (p: P) -> impl Parser<Vec<A>> 
where
    P: Parser<A>
{
    bind(move |buf| {
        let mut v = vec![];
        let mut buf_out = buf;
        while let Ok((buf, out)) = p.parse(buf_out) {
            v.push(out);

            buf_out = buf;
        }
        Ok((buf_out, v))
    })
}

pub fn one_or_more<'a, A, P> (p: P) -> impl Parser<Vec<A>> 
where
    P: Parser<A>
{
    bind(move |buf| {
        let mut v = vec![];
        let mut buf_out = buf;
        while let Ok((buf, out)) = p.parse(buf_out) {
            v.push(out);

            buf_out = buf;
        }
        if v.is_empty() {
            par_err(buf, "none of pattern found in 'one_or_more'")
        } else {
            Ok((buf_out, v))
        }
    })
}

pub fn prefix<'a, T, P> (s: &'a str, p: P) -> impl Parser<T>
where
    P: Parser<T>
{
    map( 
        and(
            parse_literal(s),
            p
        ),
        |(_, a)| a
    )
}

pub fn suffix<'a, T, P> (s: &'a str, p: P) -> impl Parser<T> 
where
    P: Parser<T>
{
    map( 
        and(
            p,
            parse_literal(s) 
        ),
        |(a, _)| a
    )
}

pub fn surround<'a, T, P> (a: &'a str, b: &'a str, p: P) -> impl Parser<T> 
where
    P: Parser<T>
{
    prefix(
        a, 
        suffix(b, p) 
    )
}

pub fn parse_literal (lit: &str) -> impl Parser<String> {
    let lit = lit.to_owned();
    bind(move |buf: & str| match buf.get(0..lit.len()) {
        Some(s) if s == lit => Ok((&buf[lit.len()..], lit.clone())),
        _ => par_err_s(buf, format!("Literal '{}' not found", lit))
    })
}


pub fn parse_literals (lits: Vec<&str>) -> impl Parser<String> {
    let lits: Vec<String> = lits.into_iter().map(|s| s.to_owned()).collect();
    bind(move |buf: &str| {
        for lit in lits.iter() {
            match buf.get(0..lit.len()) {
                Some(s) if &s == lit => return Ok((&buf[lit.len()..], lit.clone())),
                _ => continue
            }
        }
        par_err_s(buf, format!("Literal '{:?}' not found", lits))
    })
}

pub fn parse_tok_with_rule<R> (rule: R) -> impl Parser<String> 
where
    R: Fn (char) -> bool
{
    bind(move |buf: &str| {
        let mut tok = String::new();
        let mut iter = buf.chars();

        match iter.next() {
            Some(c) if rule(c) => tok.push(c),
            _ => return par_err(buf, "First character does not satisfy rule")
        }
        while let Some(c) = iter.next() {
            if rule(c) {
                tok.push(c);
            } else { break }
        }
        if tok.is_empty() {
            par_err(buf, "Empty Token.")
        } else {
            Ok((&buf[tok.len()..], tok))
        }
    })
}


pub fn parse_number<'a> (buf: &'a str) -> ParseRes<'a, f64> {
    let num_rule = |c: char| {
        c.is_ascii_digit() || c == '.'
    };
    let (buf, tok) = parse_tok_with_rule(num_rule).parse(buf)?;
    if let Ok(num) = tok.parse::<f64>() {
        Ok((buf, num))
    } else {
        par_err(buf, "could not parse into number")
    }
}


pub fn parse_identifier<'a> (buf: &'a str) -> ParseRes<'a, String> {
    let rule = |c: char| {
        c.is_alphanumeric() || c == '_'
    };
    let (buf, tok) = parse_tok_with_rule(rule).parse(buf)?;

    if tok.chars().next().unwrap().is_ascii_digit() { return par_err(buf, "identifier cannot start with digit") }
    if KEYWORDS.contains(&tok.as_str())             { return par_err(buf, "found keyword, cannot be used as identifier") }

    Ok((buf, tok))
}


pub fn map<'a, A, B, P, F> (parser: P, functor: F) -> impl Parser<B> 
where 
    P: Parser<A>,
    F: Fn(A) -> B,
{
    bind(move |buf: &str| -> ParseRes<B> {
        parser.parse(buf)
            .map(|(b, out): (&str, A)| (b, functor(out)))
    })
}

const KEYWORDS: [&str; 4] = [
    "true",
    "false",
    "if",
    "eval"
];
