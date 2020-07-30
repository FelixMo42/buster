pub struct Spot {
    pub start : usize,
    pub end   : usize,
}

impl Spot {
    pub fn new() -> Self {
        return Spot {
            start : 0,
            end   : 0,
        }
    }
}

pub struct Area {
    pub start : Spot,
    pub end   : Spot,
}

impl Area {
    pub fn new() -> Self {
        return Area {
            start : Spot::new(),
            end   : Spot::new(),
        }
    }
}