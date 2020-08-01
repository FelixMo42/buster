use crate::scope;

#[allow(non_upper_case_globals)]
const whitespace  : &'static [char]= &[' ', '\n', '\t'];

#[allow(non_upper_case_globals)]
const punctuation : &'static [char]= &[
    '{', '}', '(', ')', '[', ']', ',',
    ' ', '\n', '\t'
];

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Word,
    Punctuation,
    StringLit
}

pub struct Token {
    pub kind   : TokenKind,
    pub start  : usize,
    pub length : usize,
    pub text   : String
}

pub struct Tokenizer <'a> {
    file   : &'a str,
    index  : usize,
}

impl <'a> Tokenizer <'a> {
    fn done(&mut self) -> bool {
        return self.file.len() == self.index;
    }

    fn eat(&mut self, chr: char) -> bool {
        if !self.done() && self.file.as_bytes()[self.index] as char == chr {
            self.index += 1;
            return true;
        }

        return false;
    }

    fn eat_not(&mut self, chr: char) -> bool {
        if !self.done() && self.file.as_bytes()[self.index] as char != chr {
            self.index += 1;
            return true;
        }

        return false;
    }

    fn eat_arr(&mut self, chrs: &[char]) -> bool {
        if self.done() {
            return false;
        }
    
        for chr in chrs.iter() {
            if self.file.as_bytes()[self.index] as char == *chr {
                self.index += 1;
                return true;
            }
        }

        return false;
    }

    fn eat_arr_not(&mut self, chrs: &[char]) -> bool {
        if self.done() {
            return false;
        }

        for chr in chrs {
            if self.file.as_bytes()[self.index] as char == *chr {
                return false;
            }
        }

        self.index += 1;

        return true;
    }

    fn read(&mut self) -> TokenKind {
        if self.eat_arr(punctuation) {
            return TokenKind::Punctuation;
        }

        if self.eat('\"') {
            while self.eat_not('\"') {}

            if self.eat('\"') {
                return TokenKind::StringLit;
            }
        }

        while self.eat_arr_not(punctuation) {}

        return TokenKind::Word;
    }
}

impl Iterator for Tokenizer <'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        // skip over the whitespace
        while self.eat_arr(whitespace) {
            self.index += 1;
        }

        // weve reached the end of the file
        if self.done() {
            return None;
        }

        // the start position of the token
        let start = self.index;

        let kind = self.read();

        let token = Token {
            kind   : kind,
            text   : self.file[start..self.index].to_string(),
            start  : start,
            length : self.index - start
        };

        return Some(token);
    }
}

pub struct Index {
    index : usize
}

impl Index {
    fn new() -> Self {
        return Index {
            index : 0
        }
    }
}

pub struct Tokens <'a> {
    index  : usize,
    source : Vec<Token>,
    pub scope  : &'a mut scope::Scope <'a>
}

impl <'a> Tokens <'a> {
    pub fn save(&self) -> usize {
        return self.index;
    }

    pub fn load(&mut self, index: usize) {
        self.index = index;
    }

    pub fn peek(&self) -> &Token {
        return &self.source[self.index];
    }

    pub fn next(&mut self) {
        self.index += 1;
    }
}

pub fn tokenize<'a>(file : &str, scope : &'a mut scope::Scope <'a>) -> Tokens<'a>  {
    let tokenizer = Tokenizer {
        file   : file,
        index  : 0,
    };

    return Tokens {
        index  : 0,
        source : tokenizer.collect(),
        scope  : scope
    };
}