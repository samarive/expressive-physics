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
	selected_point: i32,

	rl_handle: RaylibHandle,
	rl_thread: RaylibThread,
	
	inspector: Widget,
	point_menu: Widget,
	contextual_menu: Widget,

	inspector_layout: Layout,
	point_menu_layout: Layout,
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
			selected_point: -1i32,

			rl_handle,
			rl_thread,
			
			inspector: Widget::new(Layout::new(Vector2::new(0f32, 0f32), Vector2::new(1f32, 1f32)), WidgetVariant::Frame { outline_thickness: 1f32}).style(Style::default().background(Color::BLACK).foreground(Color::new(255, 255, 255, 100))),
			contextual_menu: Widget::new(Layout::new(Vector2::new(0f32, 0f32), Vector2::new(1f32, 1f32)), WidgetVariant::Frame {outline_thickness: 0f32}).hidden(),
			point_menu: Widget::new(Layout::new(Vector2::new(0f32, 0f32), Vector2::new(1f32, 1f32)), WidgetVariant::Frame {outline_thickness: 1f32}).style(Style::default().background(Color::BLACK).foreground(Color::new(255, 255, 255, 100))),
			
			inspector_layout: Layout::new(Vector2::new(100f32, 225f32), Vector2::new(200f32, 400f32)),
			point_menu_layout: Layout::new(Vector2::new(600f32, 100f32), Vector2::new(400f32, 200f32)),
			contextual_menu_layout: Layout::new(Vector2::new(600f32, 150f32), Vector2::new(100f32, 200f32))
		};

		// Temporary UI

		r
		.build_contextual_menu()
		.build_point_menu()
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
		self.inspector.check_event_in_tree(&self.inspector_layout, &mut self.rl_handle);
		self.point_menu.check_event_in_tree(&self.point_menu_layout, &mut self.rl_handle);
		self.contextual_menu.check_event_in_tree(&self.contextual_menu_layout, &mut self.rl_handle);

		// Special behaviours

		let contextual_activations = self.contextual_menu.get_all_activations();
		let inspector_activations  = self.inspector.get_all_activations();
		let point_menu_activations = self.point_menu.get_all_activations();

		self.contextual_menu_events(contextual_activations);
		self.inspector_events(inspector_activations);
		self.point_menu_events(point_menu_activations);
				
	}

	fn draw(&mut self) {
		let mut d = self.rl_handle.begin_drawing(&self.rl_thread);

		d.clear_background(Color::WHITE);

		for point in self.world.iter_mut() {
			point.draw(&mut d);
		}

		self.inspector.draw_tree(&Layout::new(Vector2::new(100f32, 225f32), Vector2::new(200f32, 400f32)), &mut d);
		self.point_menu.draw_tree(&Layout::new(Vector2::new(600f32, 100f32), Vector2::new(400f32, 200f32)), &mut d);
		self.contextual_menu.draw_tree(&self.contextual_menu_layout, &mut d);
	}

	fn build_contextual_menu(mut self) -> Self {
		self.contextual_menu = self.contextual_menu.add_child(
			Widget::new(
				Layout::new(Vector2::new(0f32, -0.45f32), Vector2::new(1f32, 0.1f32)),
				WidgetVariant::Button {
					state: ButtonState::Rest
				}
			).style(Style::default().foreground(Color::GREEN).action(Color::GRAY)).id("add point".to_string())
			.add_child(
				Widget::new(
					Layout::new(Vector2::new(0f32, 0f32), Vector2::new(1f32, 1f32)),
					WidgetVariant::Label {text: "Add point".to_string(), font_size: 16i32}
				).style(Style::default().background(Color::new(0, 0, 0, 0)))
			)

		);

		self
	}

	fn show_point_menu(&mut self, id: i32) {
		if let Some(title) = self.point_menu.seek_in_tree("title") {
			if let WidgetVariant::Label {text, ..} = title.get_variant() {
				*text = format!("Point properties {id}");
			}
		}
		self.point_menu.set_visible(true);
	}

	fn build_point_menu(mut self) -> Self {

		self.point_menu = self.point_menu
			.add_child(
				Widget::new(
					Layout::new(Vector2::new(0f32, -0.4f32), Vector2::new(0.8f32, 0.2f32)),
					WidgetVariant::Label {text: format!("Point properties"), font_size: 24i32}
				).id("title".to_string())
			)
			.add_child(
				Widget::new(
					Layout::new(Vector2::new(0f32, -0.15f32), Vector2::new(0.8f32, 0.15f32)),
					WidgetVariant::TextInput {selected: false, text: String::new(), placeholder: "Acceleration X".to_string(), registered: true}
				).id("set ax".to_string())
			)
			.add_child(
				Widget::new(
					Layout::new(Vector2::new(0f32, 0.05f32), Vector2::new(0.8f32, 0.15f32)),
					WidgetVariant::TextInput {selected: false, text: String::new(), placeholder: "Acceleration Y".to_string(), registered: true}
				).id("set ay".to_string())
			)
			.add_child(
				Widget::new(
					Layout::new(Vector2::new(0f32, 0.3f32), Vector2::new(0.4f32, 0.2f32)),
					WidgetVariant::Button {state: ButtonState::Rest}
				).style(Style::default())
				.id("apply forces".to_string())
				.add_child(
					Widget::new(
						Layout::new(Vector2::new(0f32, 0f32), Vector2::new(0.9f32, 0.9f32)),
						WidgetVariant::Label {text: "Apply force !".to_string(), font_size: 16i32}
					).style(Style::default().background(Color::new(0, 0, 0, 0)))
				)
			)
			.hidden();

		self
	}

	fn contextual_menu_events(&mut self, contextual_activations: Vec::<String>) {
		if contextual_activations.contains(&"add point".to_string()) {
			
			// Adding point in world
			let mut new_point = Point::new(Vector2::new(self.rl_handle.get_mouse_position().x, self.rl_handle.get_mouse_position().y));
			new_point.set_trail_visibility(true);
			self.world.push(new_point);

			// Adding point handle in inspector
			let children_count = self.inspector.get_children_count(1u32);
			let h =  children_count as f32 * 0.1f32 - 0.4f32;
			self.inspector.add_child_inplace(
				Widget::new(
					Layout::new(
						Vector2::new(0f32, h),
						Vector2::new(0.8f32, 0.1f32)
					),
					WidgetVariant::Button {state: ButtonState::Rest}
				)
				.id(format!("point{children_count}"))
				.add_child(
					Widget::new(
						Layout::new(
							Vector2::new(0f32, 0f32),
							Vector2::new(1f32, 1f32)
						),
						WidgetVariant::Label {text: format!("point {children_count}").to_string(), font_size: 16i32}
					).style(Style::default().background(Color::new(0, 0, 0, 0)))
				)
			);
		}

		if self.rl_handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
			self.contextual_menu_layout.center = self.rl_handle.get_mouse_position() + self.contextual_menu_layout.size/2f32;
			self.contextual_menu.set_visible(true);
		}
		if self.rl_handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && !self.contextual_menu_layout.contains(self.rl_handle.get_mouse_position()) {
			self.contextual_menu.set_visible(false);
		}
	}

	fn inspector_events(&mut self, inspector_activations: Vec::<String>) {
		let mut handle_activated = false;
		for id in inspector_activations.iter() {
			if id.starts_with("point") {
				match id[5..].parse::<i32>() {
					Ok(v) =>  {
						self.selected_point = v;
						self.show_point_menu(v);
						handle_activated = true;
					}
					Err(_) => println!("Ill formated point name, expected i32 after column 5.")
				}
			}
		}

		if 
			!self.point_menu_layout.contains(self.rl_handle.get_mouse_position()) &&
			self.rl_handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) &&
			!handle_activated 
		{
			self.point_menu.set_visible(false);
			self.selected_point = -1i32;
		}
	}

	fn point_menu_events(&mut self, point_menu_activations: Vec::<String>) {
		if point_menu_activations.contains(&"apply forces".to_string()) {
			match (
				Tokenizer::tokenize(&self.point_menu.get_entry_in_tree("set ax").unwrap_or("0".to_string())),
				Tokenizer::tokenize(&self.point_menu.get_entry_in_tree("set ay").unwrap_or("0".to_string()))
			) {
				(Ok(tx), Ok(ty)) => {
					match self.world.get_mut(self.selected_point as usize) {
						Some(point) => {
							match point.add_force("test", Force {x: tx, y: ty}) {
								Ok(_) => println!("Force addded !"),
								Err(e) => println!("Can't add force : {e:?}.")
							}
						},
						None => println!("Error : Trying to set forces of point {} which doesn't exist.", self.selected_point)
					}
				}
				(Err(e), _) => println!("Error on X expression : {e:?}"),
				(_, Err(e)) => println!("Error on Y expression : {e:?}")
			}
					
		}
	}

}