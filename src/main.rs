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
	let mut scope = scope::Scope::new();

	// add number operators
	for name in [ "+", "-", "*", "/" ].iter() {
		scope.add_lang_defined_func( name , vec! [
			scope::Kind::new("i64"),
			scope::Kind::new("i64"),
		] );
	};

	// parse the command line arguments
	let opt = CliOpts::from_args();

	let add_func : ast::Node = ast::Variable::make("+", &scope);
    let mul_func : ast::Node = ast::Variable::make("*", &scope);

    let root = ast::Node::FuncMake(ast::FuncMake {
        params : vec! [
            ast::FuncMakeParam {
                name : String::from("a"),
                kind : ast::Variable::make("i64", &scope)
            },
            ast::FuncMakeParam {
                name : String::from("b"),
                kind : ast::Variable::make("i64", &scope)
            }
        ],
        body : ast::StmBlock {
            statments : vec! [
                ast::Statment::Assign(String::from("c"), ast::Node::FuncCall( ast::FuncCall {
                    func : &mul_func,
                    args : vec! [
                        ast::Variable::make("a", &scope),
                        ast::NumValue::make(42),
                    ]
                } ) ),

                ast::Statment::Output( ast::Node::FuncCall( ast::FuncCall {
                    func : &add_func,
                    args : vec! [
                        ast::Variable::make("b", &scope),
                        ast::Variable::make("c", &scope),
                    ]
                } ) )
            ]
		}
	});

	// give are selves some padding
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

	// give are selves some padding
	println!("");
}