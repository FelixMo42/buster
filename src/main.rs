pub struct Spot {
    line : usize,
    colm : usize,
}

pub struct Area {
    start : Spot,
    end   : Spot,
}

pub enum SymbolKind {
    Module,
    KeyWord,
    Variable,
    StringValue,
    NumberValue,
}

pub struct Symbol {
    kind : SymbolKind,
    area : Area,
    prop : Vec<Symbol>
}

pub struct Document {
    text : String,
    line : Vec<usize>,
    root : Vec<Symbol>
}

impl Document {
    fn get_area(&self, area: &Area) -> &str {
        let start = self.line[area.start.line] + area.start.colm;
        let end   = self.line[area.end.line]   + area.end.colm;

        return &self.text[start..end];
    }
}

fn index_of_lines(text: &str) -> Vec<usize> {
    let mut lines = vec! [ 0 ];

    let mut i = 0;
    
    for chr in text.chars() {
        i += 1;
        
        if chr == '\n' {
            lines.push(i);
        }
    }

    return lines
}

fn parse(text: &str) -> Document {
    return Document {
        text : String::from(text),
        line : index_of_lines(text),
        root : vec! [
            Symbol {
                kind : SymbolKind::Module,
                area : Area {
                        start : Spot { line : 1 , colm : 0 },
                        end   : Spot { line : 1 , colm : 3 }
                    },
                prop : Vec::new()
            }
        ],
    }
}

fn main() {
    let doc = parse("123\n456");

    println!( "{}", doc.get_area(&doc.root[0].area) )
}