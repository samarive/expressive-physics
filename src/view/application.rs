//! Encapsule tout le code nécessaire au lancement et à la gestion de l'application.

use raylib::prelude::*;
use super::super::model::physics::*;
use super::widgets::*;
use super::super::model::tokening::*;

/// Gère les evenements, les visuels et les simulations
/// # Exemple
/// ```
///use model::physics::*;
///use raylib::math::Vector2;
///
///
///fn main() {
///
///    let mut world = World::new();
///    world.push(Point::new(Vector2::new(400f32, 225f32)));
///
///    let mut app = Application::realize(world);
///
///    app.mainloop();
///}
/// ```
pub struct Application {
	world: World,
	rl_handle: RaylibHandle,
	rl_thread: RaylibThread,
	
	inspector: Widget,
	contextual_menu: Widget,

	contextual_menu_layout: Layout
}

impl Application {
	pub fn realize() -> Self {
		let (mut rl_handle, rl_thread) = raylib::init()
										.size(800, 450)
										.title("Expressive Physics")
										.build();
		rl_handle.set_target_fps(60);
		let mut r = Application {
			world: World::new(),
			rl_handle,
			rl_thread,
			inspector: Widget::new(Layout::new(Vector2::new(0f32, 0f32), Vector2::new(1f32, 1f32)), WidgetVariant::Frame { outline_thickness: 1f32}).style(Style::default().background(Color::BLACK).foreground(Color::WHITE)),
			contextual_menu: Widget::new(Layout::new(Vector2::new(0f32, 0f32), Vector2::new(1f32, 1f32)), WidgetVariant::Frame {outline_thickness: 0f32}).hidden(),
			contextual_menu_layout: Layout::new(Vector2::new(600f32, 150f32), Vector2::new(100f32, 200f32))
		};

		// r.inspector = r.inspector.add_child(Widget::new(Layout::new(Vector2::new(0f32, 0f32), Vector2::new(0.8f32, 0.3f32)), WidgetVariant::Label{text: "Hello World!".to_string(), font_size: 24i32}).style(Style::default()));
		r.inspector = r.inspector.add_child(
			Widget::new(
				Layout::new(Vector2::new(0f32, 0f32), Vector2::new(0.8f32, 0.05f32)),
				WidgetVariant::TextInput {selected: false, text: String::new(), placeholder: "Type here".to_string(), registered: true}
			).id("set last point ax")
		);

		r.contextual_menu = r.contextual_menu.add_child(
			Widget::new(
				Layout::new(Vector2::new(0f32, -0.45f32), Vector2::new(1f32, 0.1f32)),
				WidgetVariant::Button {
					state: ButtonState::Rest
				}
			).style(Style::default().foreground(Color::GREEN)).id("add point")
			.add_child(
				Widget::new(
					Layout::new(Vector2::new(0f32, 0f32), Vector2::new(1f32, 1f32)),
					WidgetVariant::Label {text: "Add point".to_string(), font_size: 16i32}
				).style(Style::default().background(Color::new(0, 0, 0, 0)))
			)

		);


		r
	}


	pub fn mainloop(&mut self) {
		while !self.rl_handle.window_should_close() {

			for p in self.world.iter_mut() {
				p.simulate();
			}

			self.inspector.check_event_in_tree(&Layout::new(Vector2::new(100f32, 225f32), Vector2::new(200f32, 400f32)), &mut self.rl_handle);
			self.contextual_menu.check_event_in_tree(&self.contextual_menu_layout, &mut self.rl_handle);

			if self.contextual_menu.check_activation_in_tree("add point") {
				self.world.push(Point::new(Vector2::new(self.rl_handle.get_mouse_position().x, self.rl_handle.get_mouse_position().y)));
			}
			if let Some(s) = self.inspector.check_entry_in_tree("set last point ax") {
				if let Some(p) = self.world.last_mut() {
					match Tokenizer::tokenize(&s) {
						Ok(t) => {
							match p.add_force("test", Force {x: t, y: vec![Token::Value(0f32)]}) {
								Ok(_) => println!("Force ajoutée !"),
								Err(e) => println!("Impossible d'ajouter la force : {e}.")
							}
						},
						Err(e) => {
							println!("Ill formated expression : {:?}.", e);
						}
					}
					
				}
			}

			if self.rl_handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
				self.contextual_menu_layout.center = self.rl_handle.get_mouse_position() + self.contextual_menu_layout.size/2f32;
				self.contextual_menu.set_visible(true);
			}
			if self.rl_handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && !self.contextual_menu_layout.contains(self.rl_handle.get_mouse_position()) {
				self.contextual_menu.set_visible(false);
			}

			let mut d = self.rl_handle.begin_drawing(&self.rl_thread);

			d.clear_background(Color::WHITE);
			for point in self.world.iter() {
				d.draw_circle_v(point.position(), 5f32, Color::BLACK);
			}

			self.inspector.draw_tree(&Layout::new(Vector2::new(100f32, 225f32), Vector2::new(200f32, 400f32)), &mut d);
			self.contextual_menu.draw_tree(&self.contextual_menu_layout, &mut d);
		}
	}

}