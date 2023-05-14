pub mod core;
pub mod block;
pub mod expr;
pub mod bool_expr;
pub mod assign;
pub mod conditional;
pub mod operation;

use std::boxed::Box;

use self::core::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseErr {
    msg: String
}
impl ParseErr {
    pub fn new (s: &str) -> Self {
        Self { msg: s.to_string() }
    }
}
impl std::fmt::Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Parser Err: {}", self.msg)
    }
}
impl std::error::Error for ParseErr {}
pub fn par_err<T> (s: &str) -> ParseRes<T> {
    Err( ParseErr::new(s) )
}

pub type ParseRes<'a, T> = Result<(&'a str, T), ParseErr>;
pub trait Parser<'a, T> {
    fn parse (&self, s: &'a str) -> ParseRes<'a, T>;
}
impl<'a, F, T> Parser<'a, T> for F 
where
    F: Fn (&'a str) -> ParseRes<T>
{
    fn parse (&self, input: &'a str) -> ParseRes<'a, T> {
        let start = {
            let mut iter = input.chars();
            let mut counter = 0;
            while let Some(c) = iter.next() {
                if c != ' ' && c != '\n' && c != '\r' {
                    break;
                }
                counter += 1;
            }
            counter
        };
        self(&input[start..])
    }
}

pub struct BoxedParser<'a, T> {
    parser: Box<dyn Parser<'a, T> + 'a>
}
impl<'a, T> BoxedParser<'a, T> 
where
    T: 'a
{
    pub fn new<P> (p: P) -> Self
    where
        P: Parser<'a, T> + 'a
    {
        Self{ parser: Box::new(p) }
    }

    pub fn option (self) -> BoxedParser<'a, Option<T>> {
        BoxedParser::new(option(self))
    }

    pub fn map<O, F> (self, f: F) -> BoxedParser<'a, O> 
    where
        O: 'a,
        F: 'a + Fn(T) -> O
    {
        BoxedParser::new( map(self, f) ) 
    }

    pub fn and<B, P> (self, p: P) -> BoxedParser<'a, (T, B)> 
    where
        B: 'a,
        P: 'a + Parser<'a, B>
    {
        BoxedParser::new( and(self, p) ) 
    }
    pub fn or<P> (self, p: P) -> BoxedParser<'a, T> 
    where
        P: 'a + Parser<'a, T>
    {
        BoxedParser::new( or(self, p) ) 
    }

    pub fn one_or_more (self) -> BoxedParser<'a, Vec<T>> {
        BoxedParser::new( one_or_more(self) )
    }

    pub fn zero_or_more (self) -> BoxedParser<'a, Vec<T>> {
        BoxedParser::new( zero_or_more(self) )
    }
}
impl<'a, T> Parser<'a, T> for BoxedParser<'a, T> {
    fn parse (&self, buf: &'a str) -> ParseRes<'a, T> {
        self.parser.parse(buf)
    }
}
