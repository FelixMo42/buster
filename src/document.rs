// https://gimletmedia.com/shows/reply-all/76h63r

use std::cmp::Ordering;

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
    Module,
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
    pub fn find(&self, target: &Spot) -> &Symbol {
        for symbol in self.children.iter() {
            if symbol.area.includes(target) {
                return symbol.find(target)
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
                return Some( symbol.find( target ) );
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

pub fn parse(text: &str) -> Document {
    return Document {
        text : String::from(text),
        line : index_of_lines(text),
        symbols : vec! [
            Symbol {
                kind : SymbolKind::Module,
                area : Area {
                        start : Spot { line : 1 , colm : 0 },
                        end   : Spot { line : 1 , colm : 3 }
                    },
                children : Vec::new()
            }
        ]
    }
}