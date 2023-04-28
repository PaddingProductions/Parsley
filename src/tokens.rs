use std::collections::VecDeque;
use std::collections::vec_deque::Iter;
use std::iter::Peekable;

pub type Tokens = VecDeque<Token>;
pub type TokIter<'a> = Peekable<Iter<'a, Token>>;

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Num,
    Boolean,
    Operator,
    Ident,
    EOF 
}

#[derive(Debug)]
pub struct Token {
    pub typ: TokenType,
    pub str: String,
}
impl Token {
    pub fn str (&self) -> &str { self.str.as_str() }
}


