//! Expressive Physics vise a être un logiciel simple permettant de faire des
//! simulations physiques. La principale fonctionalité est de pouvoir entrer des
//! expressions de force (ex: "frotements = -lambda * v") à calculer en temps
//! réel sur un ensemble de points.
//! Ce projet est en phase très expérimentale et de multiples changement de but
//! et d'implémentations sont à prévoir.
//! Le logiciel n'est d'ailleurs pas encore utilisable.

mod model;
mod view;

use view::application::Application;

fn main() {
    
    let mut app = Application::realize();

    app.mainloop();
}
