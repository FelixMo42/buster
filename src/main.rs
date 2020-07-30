// mod document;

mod ast;

fn encode_list<'a, T>(arr: &Vec<T>, func: fn(&T) -> String) -> String {
    arr.iter().map(func).collect::<Vec<String>>().join(", ")
}

fn encode<'a>(node: &'a ast::Node<'a>) -> String {
    match node {
        ast::Node::FuncMake (a) => format!("([{}]) {}",
            encode_list(&a.params, |arg| format!("{} {}", encode(&arg.kind), arg.name)),
            encode(a.output),
        ),
        ast::Node::FuncCall (a) => format!("{}({})",
            encode(a.func),
            encode_list(&a.args, |arg| encode(&arg))
        ),
        ast::Node::Variable (a) => format!("{}", a.name),
    }
}

fn main() {
    let func = ast::Variable::make("+");

    let root = ast::FuncMake {
        params : vec! [
            ast::FuncMakeParam {
                name : String::from("a"),
                kind : ast::Variable::make("i64")
            },
            ast::FuncMakeParam {
                name : String::from("b"),
                kind : ast::Variable::make("i64")
            }
        ],
        output : & ast::Node::FuncCall( ast::FuncCall {
            func : &func,
            args : vec! [
                ast::Variable::make("a"),
                ast::Variable::make("b"),
            ]
        } )
    };

    println!("{}", encode( & ast::Node::FuncMake( root ) ));
}