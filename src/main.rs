// mod document;

use std::iter::Iterator;

pub enum Node <'a> {
    FuncMake (FuncMake<'a>),
    FuncCall (FuncCall<'a>),
    Variable (Variable)
}

pub struct FuncMakeParam <'a> {
    name : String,
    kind : Node<'a>
}

pub struct FuncMake <'a> {
    params : Vec<FuncMakeParam<'a>>,
    output : &'a Node<'a>
}

// pub struct FuncCallArg <'a> {
//     name  : Option<String>,
//     value : Node<'a>
// }

pub struct FuncCall <'a> {
    func : &'a Node<'a>,
    args : Vec<Node<'a>>
}

pub struct Variable {
    name : String
}

fn encode_list<'a, T>(arr: &Vec<T>, func: fn(&T) -> String) -> String {
    arr.iter().map(func).collect::<Vec<String>>().join(", ")
}

fn encode<'a>(node: &'a Node<'a>) -> String {
    match node {
        Node::FuncMake (a) => format!("([{}]) {}",
            encode_list(&a.params, |arg| format!("{} {}", encode(&arg.kind), arg.name)),
            encode(a.output),
        ),
        Node::FuncCall (a) => format!("{}({})",
                encode(a.func),
                encode_list(&a.args, |arg| encode(&arg))
            ),
        Node::Variable (a) => format!("{}", a.name),
    }
}

fn var<'a>(name: &'a str) -> Node {
    return Node::Variable( Variable { name : String::from(name) } )
}

fn main() {
    let func = var("+");

    let root = FuncMake {
        params : vec! [
            FuncMakeParam {
                name : String::from("a"),
                kind : var("i64")
            },
            FuncMakeParam {
                name : String::from("b"),
                kind : var("i64")
            }
        ],
        output : & Node::FuncCall( FuncCall {
            func : &func,
            args : vec! [
                var("a"),
                var("b"),
            ]
        } )
        
        // Variable { name : String::from("hi") } )
    };

    println!("{}", encode( & Node::FuncMake( root ) ));
}