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
///    let mut app = Application::realize();
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
		let r = Application {
			world: World::new(),
			rl_handle,
			rl_thread,
			inspector: Widget::new(Layout::new(Vector2::new(0f32, 0f32), Vector2::new(1f32, 1f32)), WidgetVariant::Frame { outline_thickness: 1f32}).style(Style::default().background(Color::BLACK).foreground(Color::WHITE)),
			contextual_menu: Widget::new(Layout::new(Vector2::new(0f32, 0f32), Vector2::new(1f32, 1f32)), WidgetVariant::Frame {outline_thickness: 0f32}).hidden(),
			contextual_menu_layout: Layout::new(Vector2::new(600f32, 150f32), Vector2::new(100f32, 200f32))
		};

		// Temporary UI

		r
		.build_inspector()
		.build_contextual_menu()
	}


	pub fn mainloop(&mut self) {
		while !self.rl_handle.window_should_close() {

			for p in self.world.iter_mut() {
				p.simulate();
			}

			self.handle_events();
			self.draw();
		}

		println!("Application closed successfuly :)");
	}

	fn handle_events(&mut self) {

		// Make widget trees hear events
		self.inspector.check_event_in_tree(&Layout::new(Vector2::new(100f32, 225f32), Vector2::new(200f32, 400f32)), &mut self.rl_handle);
		self.contextual_menu.check_event_in_tree(&self.contextual_menu_layout, &mut self.rl_handle);

		// Special behaviours

		// Add point button
		if self.contextual_menu.check_activation_in_tree("add point") {
			self.world.push(Point::new(Vector2::new(self.rl_handle.get_mouse_position().x, self.rl_handle.get_mouse_position().y)));
			self.world.last_mut().unwrap().set_trail_visibility(true);
		}

		// Apply forces button
		if self.inspector.check_activation_in_tree("apply forces") {
			match (Tokenizer::tokenize(&self.inspector.get_entry_in_tree("set ax").unwrap()), Tokenizer::tokenize(&self.inspector.get_entry_in_tree("set ay").unwrap())) {
				(Ok(tx), Ok(ty)) => {
					if let Some(p) = self.world.last_mut() {
						match p.add_force("test", Force {x: tx, y: ty}) {
							Ok(_) => println!("Force addded !"),
							Err(e) => println!("Can't add force : {e:?}.")
						}
					}
					else {
						println!("No point in world !");
					}
				}
				(Err(e), _) => println!("Error on X expression : {e:?}"),
				(_, Err(e)) => println!("Error on Y expression : {e:?}")
			}
		}

		// Toggle contextual menu
		if self.rl_handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
			self.contextual_menu_layout.center = self.rl_handle.get_mouse_position() + self.contextual_menu_layout.size/2f32;
			self.contextual_menu.set_visible(true);
		}
		if self.rl_handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && !self.contextual_menu_layout.contains(self.rl_handle.get_mouse_position()) {
			self.contextual_menu.set_visible(false);
		}
	}

	fn draw(&mut self) {
		let mut d = self.rl_handle.begin_drawing(&self.rl_thread);

		d.clear_background(Color::WHITE);

		for point in self.world.iter_mut() {
			point.draw(&mut d);
		}

		self.inspector.draw_tree(&Layout::new(Vector2::new(100f32, 225f32), Vector2::new(200f32, 400f32)), &mut d);
		self.contextual_menu.draw_tree(&self.contextual_menu_layout, &mut d);
	}

	fn build_contextual_menu(mut self) -> Self {
		self.contextual_menu = self.contextual_menu.add_child(
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

		self
	}

	fn build_inspector(mut self) -> Self {
		self.inspector = self.inspector.add_child(
			Widget::new(
				Layout::new(Vector2::new(0f32, -0.4f32), Vector2::new(0.8f32, 0.05f32)),
				WidgetVariant::TextInput {selected: false, text: String::new(), placeholder: "Acceleration X".to_string(), registered: true}
			).id("set ax")
		);
		self.inspector = self.inspector.add_child(
			Widget::new(
				Layout::new(Vector2::new(0f32, -0.3f32), Vector2::new(0.8f32, 0.05f32)),
				WidgetVariant::TextInput {selected: false, text: String::new(), placeholder: "Acceleration Y".to_string(), registered: true}
			).id("set ay")
		);
		self.inspector = self.inspector.add_child(
			Widget::new(
				Layout::new(Vector2::new(0f32, -0.2f32), Vector2::new(0.8f32, 0.05f32)),
				WidgetVariant::Button {state: ButtonState::Rest}
			).id("apply forces").add_child(
				Widget::new(
					Layout::new(Vector2::new(0f32, 0f32), Vector2::new(1f32, 1f32)),
					WidgetVariant::Label {text: "Apply force !".to_string(), font_size: 16i32}
				).style(Style::default().background(Color::new(0, 0, 0, 0)))
			)
		);

		self
	}

}