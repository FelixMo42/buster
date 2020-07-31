use structopt::StructOpt;

// mod document;
pub mod generator;
pub mod ast;
pub mod scope;
pub mod parser;

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

	// parse the file
	let root = parser::parser::parser("");
	
	// give are selves some padding
	println!("");

	match opt {
		CliOpts::To { format } => {
			match format.as_ref() {
				"tus" => println!( "{}", generator::gen_tus::generate(&root) ),
				"js"  => println!( "{}", generator::gen_js::generate(&root)  ),
				_     => println!( "unrecognised output format '{}'", format )
			}
		}
	}

	// give are selves some padding
	println!("");
}