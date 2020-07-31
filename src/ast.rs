use crate::scope;

pub enum Node <'a> {
    FuncMake (FuncMake<'a>),
    FuncCall (FuncCall<'a>),
    StmBlock (StmBlock<'a>),
    Variable (Variable<'a>),
    NumValue (NumValue),
    // Sequence (Sequence) 
}

// pub impl Node {
//     pub fn resolve() -> scope::Kind {

//     }
// }

//

pub enum Statment <'a> {
    Assign (String, Node<'a>),
    Effect (Node<'a>),
    Output (Node<'a>),
}

pub struct StmBlock <'a> {
    pub statments : Vec<Statment<'a>>
}

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

pub struct Variable <'a> {
    pub name : String,
    pub scope : &'a scope::Scope<'a>
}

impl <'a> Variable <'a> {
    pub fn new(name: &'a str, scope: &'a scope::Scope) -> Self {
        return Variable {
            name : String::from(name),
            scope : scope
        };
    }

    pub fn make(name: &'a str, scope: &'a scope::Scope) -> Node<'a> {
        return Node::Variable( Variable::new(name, scope) );
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