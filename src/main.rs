mod document;

pub enum NodeKind {
    MakeFunc,
    CallFunc,
    Variable,
    Sequence,
    Terminal,
}

pub enum Node <'a> {
    FuncMake (&'a FuncMake<'a>),
    FuncCall (&'a FuncCall<'a>),
    Variable (&'a Variable)
}

pub struct FuncMakeParam <'a> {
    name : String,
    expt : Node<'a>
}

pub struct FuncMake <'a> {
    params : Vec<FuncMakeParam<'a>>,
    output : Node<'a>
}

pub struct FuncCallArg <'a> {
    name  : Option<String>,
    value : Node<'a>
}

pub struct FuncCall <'a> {
    func : Node<'a>,
    args : Vec<FuncCallArg<'a>>
}

pub struct Variable {
    name : String
}

fn encode(node: &Node) -> String {
    
    return String::from("")
}

fn main() {
    let token = & Variable { name : String::from("hi") };

    let root = FuncMake {
        params : vec! [],
        output : Node::Variable( token )
    };

    println!("{}", encode( & Node::FuncMake( & root ) ));
}