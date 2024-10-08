//! Expressive Physics
//! Expressive Physics is (will be) a welcoming sandbox app for aspiring physicists !

mod model;

use model::tokening::*;
use model::parsing::*;


fn main() {
    println!("___________Expressive Physics______________");


    let test = String::from("x0 * (10 / (x1+2))");

    match Tokenizer::tokenize(&test) {
        Ok (tokens) => {
            println!("_______________Tokens_____________");
            println!("{:?}", tokens);
            println!("_______________Parsed______________");
            let mut context = VariableContext::new();
            context.insert("x0".to_string(), 6f32);
            context.insert("x1".to_string(), 10f32);

            println!("{:?}", Parser::parse(&tokens, &context));
        },
        Err (e) => eprintln!("{:?}", e)
    }
}
