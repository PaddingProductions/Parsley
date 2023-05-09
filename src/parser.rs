pub mod core;
pub mod block;
pub mod expr;
pub mod bool_expr;
pub mod assign;
pub mod conditional;
pub mod operation;

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
        self(input)
    }
}
