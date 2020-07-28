mod document;

fn main() {
    let doc = document::parse("([int x, int y] int) (+ x y)");

    let symbol = doc
            .find( & document::Spot { line : 1 , colm : 1 } )
            .expect("couldnt find the requested symbol");

    println!( "{}", doc.get_area(&symbol.area) )
}