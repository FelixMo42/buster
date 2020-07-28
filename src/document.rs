// https://gimletmedia.com/shows/reply-all/76h63r

use std::cmp::Ordering;

use crate::stream::Stream;

// Spot

#[derive(PartialEq, Eq, PartialOrd)]
pub struct Spot {
    pub line : usize,
    pub colm : usize,
}

impl Ord for Spot {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.line > other.line {
            return Ordering::Greater;
        }

        if self.line < other.line {
            return Ordering::Less;
        }

        return self.colm.cmp(&other.colm);
    }
}

impl Spot {
    fn next_line(&mut self) {
        self.line = self.line + 1;
        self.colm = 0;
    }

    fn next_char(&mut self) {
        self.line = self.line;
        self.colm = self.colm + 1;
    }
}

// Area

pub struct Area {
    pub start : Spot,
    pub end   : Spot,
}

impl Area {
    pub fn includes(&self, spot: &Spot) -> bool {
        return spot >= &self.start && spot <= &self.end
    }
}

// Symbol

pub enum SymbolKind {
    // KeyWord,
    // Variable,
    // StringValue,
    // NumberValue,
    // Error
}

pub struct Symbol {
    pub kind : SymbolKind,
    pub area : Area,
    pub children : Vec<Symbol>
}

impl Symbol {
    pub fn search(&self, target: &Spot) -> &Symbol {
        for symbol in self.children.iter() {
            if symbol.area.includes(target) {
                return symbol.search(target)
            }
        }

        return self
    }
}

// Document

pub struct Document {
    pub text : String,
    pub line : Vec<usize>,
    pub symbols : Vec<Symbol>
}

impl Document {
    pub fn get_area(&self, area: &Area) -> &str {
        let start = self.line[area.start.line] + area.start.colm;
        let end   = self.line[area.end.line]   + area.end.colm;

        return &self.text[start..end];
    }

    pub fn find(&self, target: &Spot) -> Option<&Symbol> {
        for symbol in self.symbols.iter() {
            if symbol.area.includes(target) {
                return Some( symbol.search( target ) );
            }
        }

        return None
    }
}

pub fn index_of_lines(text: &str) -> Vec<usize> {
    let mut lines = vec! [ 0 ];

    let mut i = 0;

    for chr in text.chars() {
        i += 1;

        if chr == '\n' {
            lines.push(i);
        }
    }

    return lines;
}

pub fn tokenize(text: &str) -> Vec<Symbol> {
    let mut tokens = vec! [];
    let mut stream = Stream::new( text.as_bytes() );
    let mut spot = Spot { line : 0 , colm : 1 };

    loop {
        // weve reached the end of the file
        if stream.done() { break }

        // go to next line when we reach end of line
        if stream.peek() == &('\n' as u8) {
            spot.next_line();
            stream.skip();
            continue;
        }

        // skip over white spaces
        if stream.peek().is_ascii_whitespace() {
            spot.next_char();
            stream.skip();
            continue;
        }

        stream.skip();
    }

    return tokens;
}

pub fn parse(text: &str) -> Document {
    return Document {
        text : String::from(text),
        line : index_of_lines(text),
        symbols : tokenize(text)
    }
}