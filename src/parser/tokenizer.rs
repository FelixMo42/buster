#[allow(non_upper_case_globals)]
const whitespace  : &'static [char]= &[' ', '\n', '\t'];

#[allow(non_upper_case_globals)]
const punctuation : &'static [char]= &[
    '{', '}', '(', ')', '[', ']', ',',
    ' ', '\n', '\t'
];

#[derive(Debug)]
pub enum TokenKind {
    Word,
    Punctuation,
    StrValue
}

pub struct Token {
    pub kind   : TokenKind,
    pub start  : usize,
    pub length : usize,
    pub text   : String
}

pub struct Tokens <'a> {
    file   : &'a str,
    index  : usize,
    // buffer : Vec<Token>
}

impl <'a> Tokens <'a> {
    fn done(&mut self) -> bool {
        return self.file.len() == self.index;
    }

    fn eat(&mut self, chr: char) -> bool {
        if !self.done() && self.file.as_bytes()[self.index] as char == chr {
            // self.index += 1;
            return true;
        }

        return false;
    }

    fn eat_not(&mut self, chr: char) -> bool {
        if !self.done() && self.file.as_bytes()[self.index] as char != chr {
            // self.index += 1;
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
                // self.index += 1;
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

        // self.index += 1;

        return true;
    }

    fn read(&mut self) -> TokenKind {
        if self.eat_arr(punctuation) {
            self.index += 1;
            return TokenKind::Punctuation;
        }

        if self.eat('\"') {
            self.index += 1;

            while self.eat_not('\"') { self.index += 1; }

            if self.eat('\"') {
                self.index += 1;
                return TokenKind::StrValue;
            }
        }

        while self.eat_arr_not(punctuation) {
            self.index += 1;
        }

        return TokenKind::Word;
    }
}

impl Iterator for Tokens <'_> {
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

        return Some(Token {
            kind   : kind,
            text   : self.file[start..self.index].to_string(),
            start  : start,
            length : self.index - start
        })
    }
}

pub fn tokenize <'a> (file : &'a str) -> Tokens {
    return Tokens {
        file   : file,
        index  : 0,
        // buffer : Vec::new()
    };
}