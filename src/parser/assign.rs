use std::boxed::Box;

use super::ParserRes;
use crate::tokens::TokIter;
use crate::ast::*;

use super::core::*;

impl Extractable for Assignment {
    type T = Self;
    fn extract<'a, 'b> (tok: &'a TokIter<'b>) -> ParserRes<'b, Self> {
        let tok: TokIter<'b> = tok.clone();
        let (tok, ident) = ident(&tok)?;
        let (tok, _)     = operator(&tok, "=")?;
        let (n, expr): (TokIter<'b>, Expr)  = Expr::extract(&tok)?;

        Ok((
            n, 
            Self {
                ident, 
                expr: Box::new(expr)
            }
        ))
    }
}

impl Assignment {

}
