use structopt::StructOpt;

pub mod document;
pub mod generator;
pub mod ast;
pub mod scope;
pub mod parser;

#[derive(StructOpt, Debug)]
#[structopt(name="buster")]
enum CliOpts {
	To {
		format: String
	}
}

const FILE : &str = "([i64 a i64 b]) (+ a b)";

fn main() {
	let mut scope = scope::Scope::new();

	// add number operators
	for name in [ "+", "-", "*", "/" ].iter() {
		scope.add_lang_defined_func( name , vec! [
			scope::Kind::new( String::from("i64") ),
			scope::Kind::new( String::from("i64") ),
		] );
	};

	// parse the command line arguments
	let opt = CliOpts::from_args();

	// parse the file
	let root = parser::parser::parse(FILE, &mut scope);
	
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