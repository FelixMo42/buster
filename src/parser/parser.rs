use crate::scope;
use crate::ast;

pub fn parser(file: &str) -> ast::Node {
    return ast::NumValue::make(42);
}

// let add_func : ast::Node = ast::Variable::make("+", &scope);
// let mul_func : ast::Node = ast::Variable::make("*", &scope);

// ast::Node::FuncMake(ast::FuncMake {
//     params : vec! [
//         ast::FuncMakeParam {
//             name : String::from("a"),
//             kind : ast::Variable::make("i64", &scope)
//         },
//         ast::FuncMakeParam {
//             name : String::from("b"),
//             kind : ast::Variable::make("i64", &scope)
//         }
//     ],
//     body : ast::StmBlock {
//         statments : vec! [
//             ast::Statment::Assign(String::from("c"), ast::Node::FuncCall( ast::FuncCall {
//                 func : &mul_func,
//                 args : vec! [
//                     ast::Variable::make("a", &scope),
//                     ast::NumValue::make(42),
//                 ]
//             } ) ),

//             ast::Statment::Output( ast::Node::FuncCall( ast::FuncCall {
//                 func : &add_func,
//                 args : vec! [
//                     ast::Variable::make("b", &scope),
//                     ast::Variable::make("c", &scope),
//                 ]
//             } ) )
//         ]
//     }
// });