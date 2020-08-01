use crate::scope;
use crate::ast;
use crate::parser::tokenizer::{ Tokens, Token, TokenKind, tokenize };

// macro_rules! eat {
//     ($toks:ident, $bla:ident) => {
//         if $toks.source[$toks.index] {
//             $toks.index += 1;
//         }
//     }
// }

#[inline]
pub fn eat_kind(toks: &mut Tokens, kind: TokenKind) -> bool {
    if toks.peek().kind == kind {
        toks.next();
        return true;
    }

    return false;
}

#[inline]
pub fn eat_text(toks: &mut Tokens, text: &str) -> bool {
    if toks.peek().text == text {
        toks.next();
        return true;
    }

    return false;
}

type ParserRule <T> = fn(&mut Tokens) -> Option<T>;

#[inline]
pub fn eat_func<'a, T>(toks: &'a mut Tokens, func: fn(&'a mut Tokens) -> Option<T>) -> bool {
    let save = toks.save();
    let node = func(toks);

    match node {
        Some (_) => {
            // toks.load(save);
            return true;
        }
        None    => {
            return false;
        }
    }
}

pub fn parse_node <'a> (toks: &'a mut Tokens) -> Option<ast::Node<'a>> {
    if eat_text(toks, "(") {
        if eat_text(toks, "[") {
            if eat_text(toks, "]") {
                if eat_text(toks, ")") {
                    if eat_func(toks, parse_node) {
                        return Some( ast::Variable::make("hi", toks.scope) );
                    }
                }
            }
        }
    }

    return None;
}

pub fn parse <'a> (file: &'a str, scope: &'a mut scope::Scope<'a>) -> ast::Node<'a> {
    // make the token iterator
    let mut tokens = tokenize(file, scope);

    return parse_node(&mut tokens).expect("hi");
}