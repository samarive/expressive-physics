//! Expressive Physics is (will be) a welcoming sandbox app for aspiring physicists !

mod model;
mod view;

/*
use model::tokening::*;
use model::parsing::*;
*/

use model::physics::*;
use view::application::Application;
use view::widgets::*;
use std::rc::Rc;
use std::cell::RefCell;
use raylib::math::Vector2;


fn main() {
    
    let world = Rc::new(RefCell::new(World::new()));
    world.borrow_mut().push(Point::new(Vector2::new(200f32, 200f32)));

    let mut app = Application::realize(&world);

    app.mainloop();

    /*
    println!("___________Expressive Physics______________");


    let test = String::from("3.23 * 4 + (10 / 4 - 5)");

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
    */
}
