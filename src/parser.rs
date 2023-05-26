pub mod core;
pub mod block;
pub mod expr;
pub mod assign;
pub mod conditional;
pub mod operation;
pub mod print;
pub mod _loop;

use std::boxed::Box;

use self::core::*;

const BLD: &str = "\x1b[1m";
const RST: &str = "\x1b[0m";

#[derive(Debug, PartialEq, Eq)]
pub struct ParseErr<'a> {
    msg: String,
    ptr: &'a str,
}
impl<'a> ParseErr<'a> {
    pub fn new (s: String, ptr: &'a str) -> Self {
        Self { msg: s, ptr }
    }

    pub fn print(&self, buf: &str) {

        // Calculate index
        let ptr = self.ptr.as_ptr() as usize;
        let index = ptr - buf.as_ptr() as usize;

        // Get relevant line
        let (line, line_index) = {
            let mut l = index;
            let mut r = index;
            while l != 0 && buf.get((l-1)..l).unwrap() != "\n" { l -= 1; }
            while buf.get(r..=r).is_some() && buf.get(r..=r).unwrap() != "\n" { r += 1; }
            (&buf[l..r], index - l)
        };
        println!("{BLD}[== Parser Err ==]{RST}  {}", self.msg);
        println!("--> at character #{BLD}{}{RST}", index);
        println!("|\n|\t{}", line);
        print!("|\t");
        for _ in 0..line_index {
            print!(" ");
        }
        println!("{BLD}^ Here{RST}");
    }
}

pub fn par_err_s<T> (ptr: &str, s: String) -> ParseRes<T> {
    Err( ParseErr::new(s, ptr) )
}
pub fn par_err<'a, T> (ptr: &'a str, s: &str) -> ParseRes<'a, T> {
    par_err_s(ptr, s.to_owned())
}

pub type ParseRes<'a, T> = Result<(&'a str, T), ParseErr<'a>>;
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
    pub fn option_with_default (self, default: &'a dyn Fn() -> T) -> BoxedParser<'a, T> {
        BoxedParser::new(
            map(
                option(self),
                move |o| o.unwrap_or_else(default)
            )
        )
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
    pub fn prefix (self, suf: &'a str) -> BoxedParser<'a, T> {
        BoxedParser::new( prefix(suf, self) )
    }
    pub fn suffix (self, suf: &'a str) -> BoxedParser<'a, T> {
        BoxedParser::new( suffix(suf, self) )
    }
}
impl<'a, T> Parser<'a, T> for BoxedParser<'a, T> {
    fn parse (&self, buf: &'a str) -> ParseRes<'a, T> {
        self.parser.parse(buf)
    }
}
