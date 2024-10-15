//! Expressive Physics vise a être un logiciel simple permettant de faire des
//! simulations physiques. La principale fonctionalité est de pouvoir entrer des
//! expressions de force (ex: "frottements = -lambda * v") à calculer en temps
//! réel sur un ensemble de points.
//! Ce projet est en phase très expérimentale et de multiples changement de but
//! et d'implémentations sont à prévoir.
//! Le logiciel n'est d'ailleurs pas encore utilisable.

mod model;
mod view;

use model::physics::*;
use view::application::Application;
use raylib::math::Vector2;

use view::widgets::*;
use raylib::prelude::*;
fn main() {



    let mut app = Application::realize();

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
