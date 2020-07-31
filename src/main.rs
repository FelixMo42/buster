use structopt::StructOpt;


// mod document;
pub mod gen;
pub mod ast;
pub mod scope;

#[derive(StructOpt, Debug)]
#[structopt(name = "buster")]
enum CliOpts {
	To {
		format: String
	}
}

fn main() {
	// parse the command line arguments
	let opt = CliOpts::from_args();

	let add_func : ast::Node = ast::Variable::make("+");
    let mul_func : ast::Node = ast::Variable::make("*");

    let root = ast::Node::FuncMake(ast::FuncMake {
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
        body : ast::StmBlock {
            statments : vec! [
                ast::Statment::Assign(String::from("c"), ast::Node::FuncCall( ast::FuncCall {
                    func : &mul_func,
                    args : vec! [
                        ast::Variable::make("a"),
                        ast::NumValue::make(42),
                    ]
                } ) ),

                ast::Statment::Output( ast::Node::FuncCall( ast::FuncCall {
                    func : &add_func,
                    args : vec! [
                        ast::Variable::make("b"),
                        ast::Variable::make("c"),
                    ]
                } ) )
            ]
		}
	});
	
	// gives are selves some padding
	println!("");

	match opt {
		CliOpts::To { format } => {
			if format == "tus" {
				println!( "{}",  gen::gen_tus::generate(&root) )
			}

			else if format == "js" {
				println!( "{}",  gen::gen_js::generate(&root) )
			}

			else {
				println!( "unrecognised output format '{}'", format );
			}
		}
	}

	// println!( "{}",  gen::gen_tus::generate(&root) );

	// println!("{}", encode( & ast::Node::FuncMake( root ) , "" ));
}