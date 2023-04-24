#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Num,
    Boolean,
    Operator,
    Identifier,
    End
}

#[derive(Debug)]
pub struct Token {
    pub typ: TokenType,
    pub str: String,
}
impl Token {
    pub fn str (&self) -> &str { self.str.as_str() }
}


