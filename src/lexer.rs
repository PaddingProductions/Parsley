use std::collections::VecDeque;

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
    
    pub fn lex (self) -> Result<VecDeque<Token>, String> {
        let mut v = VecDeque::new(); 
        for word in self.str.split([' ','\n','\r','\t']) {
            use TokenType::*;

            if word.parse::<usize>().is_ok() {
                v.push_back( Token {
                    typ: Num,
                    str: word.to_string()
                });
            }
            else if Self::is_operator(word) {
                v.push_back( Token {
                    typ: Operator,
                    str: word.to_string()
                });
            }
            else if word.len() > 0 {
                v.push_back( Token {
                    typ: Ident,
                    str: word.to_string()
                });
            };
        }
        v.push_back( Token { 
            typ:TokenType::EOF, 
            str: String::from("")
        });

        Ok(v)
    }
}
