use std::collections::HashMap;

pub struct Kind {
    pub base : String,
    pub args : Vec<Kind>
}

//

pub struct Scope {
    hash : HashMap<String, Kind>
}

impl Scope {
    pub fn new() -> Self {
        return Scope {
            hash : HashMap::new()
        }
    }

    pub fn set(&mut self, name: String, kind: Kind) {
        self.hash.insert(name, kind);
    }

    pub fn get(&self, name: &str) -> Option<&Kind> {
        return self.hash.get(name);
    }
}

// let plus = 