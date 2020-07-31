pub enum Node <'a> {
    FuncMake (FuncMake<'a>),
    FuncCall (FuncCall<'a>),
    StmBlock (StmBlock<'a>),
    Variable (Variable),
    NumValue (NumValue),
    // Sequence (Sequence) 
}

//

pub enum Statment <'a> {
    Assign (String, Node<'a>),
    Effect (Node<'a>),
    Output (Node<'a>),
}

pub struct StmBlock <'a> {
    pub statments : Vec<Statment<'a>>
}

// impl StmBlock <'a> {
//     pub fn new(statments: Vec<Statment<'a>>) -> Self {
//         return StmBlock { statments };
//     }
// }

//

pub struct FuncMakeParam <'a> {
    pub name : String,
    pub kind : Node<'a>
}

pub struct FuncMake <'a> {
    pub params : Vec<FuncMakeParam<'a>>,
    pub body   : StmBlock<'a>
}

//

pub struct FuncCall <'a> {
    pub func : &'a Node<'a>,
    pub args : Vec<Node<'a>>
}

//

pub struct Variable {
    pub name : String
}

impl Variable {
    pub fn new<'a>(name: &'a str) -> Self {
        return Variable { name : String::from(name) };
    }

    pub fn make<'a>(name: &'a str) -> Node {
        return Node::Variable( Variable::new(name) );
    }
}

//

pub struct NumValue {
    pub value : i64,
    pub size : usize
}

impl NumValue {
    pub fn new<'a>(value: i64) -> Self {
        return NumValue { value, size : 8 };
    }

    pub fn make<'a>(value: i64) -> Node<'a> {
        return Node::NumValue( NumValue::new(value) );
    }
}

//
pub struct Sequence {
    pub legth : usize,
}