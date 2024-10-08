//! Expressive Physics
//! Expressive Physics is (will be) a welcoming sandbox app for aspiring physicists !

mod model;

use model::tokening::*;

fn main() {
    println!("___________Expressive Physics______________");


    let test = String::from("(34.5 + (12)) / 51.725 - 3 * (lambda3 / 10 - offset0) + offset1");

    match Tokenizer::tokenize(&test) {
        Ok (tokens) => println!("{:?}", tokens),
        Err (e) => eprintln!("{:?}", e)
    }
}
