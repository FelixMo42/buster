use crate::scope::{ Scope, Kind };
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
pub fn eat_kind<'a>(toks: &mut Tokens, kind: TokenKind) -> Option<&Token> {
    if toks.peek().kind == kind {
        return toks.read();
    }

    return None;
}

#[inline]
pub fn eat_text(toks: &mut Tokens, text: &str) -> bool {
    if toks.peek().text == text {
        toks.next();
        return true;
    }

    return false;
}

pub fn parse_node <'a> (toks: &mut Tokens, scope: &'a Scope) -> Option<ast::Node<'a>> {
    let save = toks.save();

    if eat_text(toks, "(") {
        if eat_text(toks, "[") {
            let mut params = Vec::new();
            while let Some(param) = parse_func_param(toks, scope) {
                params.push(param);
            }

            if eat_text(toks, "]") {
                if eat_text(toks, ")") {
                    let mut body = Vec::new();
                    while let Some(stm) = parse_statment(toks, scope) {
                        body.push(stm)
                    }

                    return Some( ast::FuncMake::make(params, ast::StmBlock::new(body)) )
                }
            }
        }
    }

    toks.load(save);

    if let Some(node) = eat_kind(toks, TokenKind::Word) {
        return Some( ast::Variable::make(node.text.clone(), scope) );                   
    }

    return None;
}

pub fn parse_func_param <'a> (toks: &mut Tokens, scope: &Scope) -> Option<ast::FuncMakeParam> {
    if let Some(name) = eat_kind(toks, TokenKind::Word) {
        if let Some(kind) = eat_kind(toks, TokenKind::Word) {
            return Some( ast::FuncMakeParam {
                name : name.text.clone(),
                kind : Kind::new( kind.text.clone() )
            } );
        }
    }

    return None;
}

pub fn parse_statment <'a> (toks: &mut Tokens, scope: &'a Scope) -> Option<ast::Statment<'a>> {
    let save = toks.save();

    if eat_text(toks, "let") {
        if let Some(name) = eat_kind(toks, TokenKind::Word) {
            if let Some(val) = parse_node(toks, scope) {
                return Some( ast::Statment::Assign(name.text.clone(), val) );
            }
        }
    }

    toks.load(save);

    if eat_text(toks, "return") {
        if let Some(val) = parse_node(toks, scope) {
            return Some( ast::Statment::Output(val) );
        }
    }

    return None;
}

pub fn parse <'a> (file: &'a str, scope: &'a Scope<'a>) -> ast::Node<'a> {
    // make the token iterator
    let mut tokens = tokenize(file);

    return parse_node(&mut tokens, scope).expect("hi");
}