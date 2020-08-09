use crate::scope;

pub enum Node <'a> {
    FuncMake (FuncMake<'a>),
    FuncCall (FuncCall<'a>),
    // StmBlock (StmBlock<'a>),
    Variable (Variable<'a>),
    NumValue (NumValue),
    // Sequence (Sequence) 
}

impl <'a> Node <'a> {
    pub fn kind(&self) -> &scope::Kind {
        match self {
            Node::FuncMake (node) => node.kind(),
            Node::FuncCall (node) => node.kind(),
            // Node::StmBlock (node) => &node.kind(),
            Node::Variable (node) => node.kind(),
            Node::NumValue (node) => node.kind(),
        }
    }
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

impl <'a> StmBlock <'a> {
    pub fn new(statments: Vec<Statment<'a>>) -> Self {
        return StmBlock { statments }
    }

    pub fn kind(&self) -> &scope::Kind {
        for statment in self.statments.iter() {
            if let Statment::Output(node) = statment  {
                return node.kind();
            };
        };

        unreachable!();
    }
}

//

pub struct FuncMakeParam {
    pub name : String,
    pub kind : scope::Kind
}

pub struct FuncMake <'a> {
    pub params : Vec<FuncMakeParam>,
    pub body   : StmBlock<'a>
}

impl <'a> FuncMake <'a> {
    pub fn new(params: Vec<FuncMakeParam>, body: StmBlock<'a>) -> Self {
        return FuncMake {
            params : params,
            body   : body
        }
    }

    pub fn make(params: Vec<FuncMakeParam>, body: StmBlock<'a>) -> Node<'a> {
        return Node::FuncMake( FuncMake::new(params, body) );
    }

    pub fn kind(&self) -> &scope::Kind {
        return self.body.kind()
    }
}

//

pub struct FuncCall <'a> {
    pub func : &'a Node<'a>,
    pub args : Vec<Node<'a>>
}

impl <'a> FuncCall <'a> {
    pub fn kind(&self) -> &scope::Kind {
        return self.func.kind().args.last().expect("func has no return value");
    }
}

//

pub struct Variable <'a> {
    pub name : &'a str,
    pub scope : &'a scope::Scope<'a>
}

impl <'a> Variable <'a> {
    pub fn new(name: &'a str, scope: &'a scope::Scope) -> Self {
        return Variable { name , scope };
    }

    pub fn make(name: &'a str, scope: &'a scope::Scope) -> Node<'a> {
        return Node::Variable( Variable::new(name, scope) );
    }

    pub fn kind(&self) -> &scope::Kind {
        return self.scope.get(self.name).expect("variable is undefined");
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

    pub fn kind(&self) -> &scope::Kind {
        // TODO: return a number kind
        unreachable!();
    }
}

//

pub struct Sequence {
    pub legth : usize,
}

impl Sequence {
    pub fn kind(&self) -> &scope::Kind {
        // TODO: return a number kind
        unreachable!();
    }
}