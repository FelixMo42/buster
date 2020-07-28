// publish the modules
pub mod document;
pub mod stream;

fn main() {
    let doc = document::parse("([int x, int y] int)\n(+ 1 1)");

    let symbol = doc
        .find( & document::Spot { line : 1 , colm : 1 } )
        .expect("couldnt find the requested symbol");

    println!( "{}", doc.get_area(&symbol.area) )
}