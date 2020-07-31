use std::collections::HashMap;

pub struct Kind {
    pub base : String,
    pub args : Vec<Kind>,

    pub lang : Option<String>
}

impl Kind {
    pub fn new(base: &str) -> Self {
        return Kind {
            base : String::from(base),
            args : vec! [],
            lang : None
        };
    }
}

//

pub struct Scope <'a> {
    hash : HashMap<String, Kind>,
    prev : Option<&'a Scope<'a>>
}

impl Scope <'_> {
    pub fn new() -> Self {
        return Scope {
            hash : HashMap::new(),
            prev : None
        };
    }

    pub fn set(&mut self, name: String, kind: Kind) {
        self.hash.insert(name, kind);
    }

    pub fn get(&self, name: &str) -> Option<&Kind> {
        return match self.hash.get(name) {
            Some (value) => Some(value),
            None => {
                match self.prev {
                    Some (prev) => prev.get(name),
                    None => None
                }
            }
        };
    }

    pub fn add_lang_defined_func(&mut self, name: &str, args: Vec<Kind>) {
        self.set( String::from(name), Kind {
            base : String::from("func"),
            args : args,
            lang : Some(String::from(name))
        });
    }
}