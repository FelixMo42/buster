pub struct Stream<'a, R> {
    index : usize,
    array : &'a [R]
}

impl<'a, R> Stream<'a, R> {
    pub fn new(array: &'a [R]) -> Self {
        return Stream {
            index : 0,
            array : array
        }
    }

    // read the current character
    pub fn peek(&self) -> &'a R {
        return &self.array[self.index];
    }

    // read the next character
    pub fn next(&mut self) -> &'a R {
        let value = self.peek();

        self.skip();

        return value;
    }

    // skip over to the next character
    pub fn skip(&mut self) {
        self.index += 1;
    }

    // get the current position
    pub fn save(&self) -> usize {
        return self.index;
    }

    // set the position
    pub fn load(&mut self, index: usize) {
        self.index = index;
    }

    // is the stream done?
    pub fn done(&mut self) -> bool {
        return self.index == self.array.len()
    }
}