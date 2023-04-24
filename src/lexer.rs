use crate::tokens::*;

pub struct Lexer {
    str: String,
}

impl Lexer {
    pub fn from_str (str: String) -> Lexer {
        Self{ str }
    }

    fn is_operator (s: &str) -> bool {
        s == "+" || s == "-" || s == "*" || s == "/" || s == "=" || s == "?" || s == ":"
    }
    
    pub fn lex (self) -> Result<Vec<Token>, String> {
        let mut v = vec![];
        for word in self.str.split(' ') {
            if word.parse::<usize>().is_ok() {
                v.push( Token {
                    typ: TokenType::Num,
                    str: word.to_string()
                });
            }
            else if Self::is_operator(word) {
                v.push( Token {
                    typ: TokenType::Operator,
                    str: word.to_string()
                });
            }
            else {
                v.push( Token {
                    typ: TokenType::Identifier,
                    str: word.to_string()
                });
            };
        }
        v.push( Token { 
            typ:TokenType::End, 
            str: String::from("")
        });

        Ok(v)
    }
}
