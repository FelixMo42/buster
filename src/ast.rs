pub enum Node <'a> {
    FuncMake (FuncMake<'a>),
    FuncCall (FuncCall<'a>),
    Variable (Variable)
}

pub struct FuncMakeParam <'a> {
    pub name : String,
    pub kind : Node<'a>
}

pub struct FuncMake <'a> {
    pub params : Vec<FuncMakeParam<'a>>,
    pub output : &'a Node<'a>
}

pub struct FuncCall <'a> {
    pub func : &'a Node<'a>,
    pub args : Vec<Node<'a>>
}

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