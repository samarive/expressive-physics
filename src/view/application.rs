//! Encapsule tout le code nécessaire au lancement et à la gestion de l'application.

use std::collections::HashMap;
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

	forces: HashMap::<String, Force>,

	rl_handle: RaylibHandle,
	rl_thread: RaylibThread,
	
	inspector: WidgetTree,
	point_menu: WidgetTree,
	contextual_menu: WidgetTree,
}

impl Application {
	pub fn realize() -> Self {
		let (mut rl_handle, rl_thread) = raylib::init()
										.size(800, 450)
										.title("Expressive Physics")
										.build();
		rl_handle.set_target_fps(60);
		
		Application {
			world: World::new(),
			selected_point: -1i32,

			forces: HashMap::<String, Force>::new(),

			rl_handle,
			rl_thread,
			
			inspector: WidgetTree::new(
			  	Self::build_default_inspector(),
			  	Layout::new(Vector2::new(100f32, 225f32), Vector2::new(200f32, 400f32))
			),
			contextual_menu: WidgetTree::new(
				Self::build_default_contextual_menu(),
				Layout::new(Vector2::new(600f32, 150f32), Vector2::new(100f32, 200f32))
			),
			point_menu: WidgetTree::new(
				Self::build_default_point_menu(),
				Layout::new(Vector2::new(600f32, 100f32), Vector2::new(400f32, 200f32))
			)

		}
	}

	fn build_default_inspector() -> Widget {
		Widget::new(
			Layout::new(
				Vector2::new(0f32,0f32),
				Vector2::new(1f32, 1f32)
			),
			WidgetVariant::Frame { outline_thickness: 1f32}
		)
		.style(Style::default()
		.background(Color::BLACK)
		.foreground(Color::new(255, 255, 255, 100)))
		.add_child(
			Widget::new(
				Layout::new(
					Vector2::new(0f32, 0f32),
					Vector2::new(1f32, 1f32)
				),
				WidgetVariant::Scroll {offset: 0f32}
			)
			.id("point scroll".to_string())
		)
	}

	fn build_default_contextual_menu() -> Widget {
		Widget::new(
			Layout::new(
				Vector2::new(0f32, 0f32),
				Vector2::new(1f32, 1f32)),
			WidgetVariant::Frame {
				outline_thickness: 0f32
			}
		)
		.hidden()
		.add_child(
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

		)
	}

	fn build_default_point_menu() -> Widget {
		Widget::new(
			Layout::new(
				Vector2::new(0f32, 0f32),
				Vector2::new(1f32, 1f32)),
			WidgetVariant::Frame {
				outline_thickness: 1f32
			}
		)
		.style(Style::default()
		.background(Color::BLACK)
		.foreground(Color::new(255, 255, 255, 100)))
		.hidden()
		.add_child(
			Widget::new(
				Layout::new(Vector2::new(0f32, -0.4f32), Vector2::new(0.8f32, 0.2f32)),
				WidgetVariant::Label {text: format!("Point properties"), font_size: 24i32}
			).id("title".to_string())
		)
		.add_child(
			Widget::new(
				Layout::new(Vector2::new(0f32, -0.15f32), Vector2::new(0.8f32, 0.15f32)),
				WidgetVariant::TextInput {
					selected: false,
					text: String::new(),
					placeholder: "Acceleration X".to_string(),
					cursor: 0u32,
					registered: true
				}
			).id("set ax".to_string())
		)
		.add_child(
			Widget::new(
				Layout::new(Vector2::new(0f32, 0.05f32), Vector2::new(0.8f32, 0.15f32)),
				WidgetVariant::TextInput {
					selected: false,
					text: String::new(),
					placeholder: "Acceleration Y".to_string(),
					cursor: 0u32,
					registered: true
				}
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
		self.inspector.check_event(&mut self.rl_handle);
		self.point_menu.check_event(&mut self.rl_handle);
		self.contextual_menu.check_event(&mut self.rl_handle);

		// Special behaviours
		self.contextual_menu_events();
		self.inspector_events();
		self.point_menu_events();
				
	}

	fn draw(&mut self) {
		let mut d = self.rl_handle.begin_drawing(&self.rl_thread);

		d.clear_background(Color::WHITE);

		for (i, point) in self.world.iter_mut().enumerate() {
			point.draw(
				if i == self.selected_point as usize {
					PointStyle::Cross
				} else {
					PointStyle::Circle
				},
				&mut d
			);
		}

		self.inspector.draw(&mut d);
		self.point_menu.draw(&mut d);
		self.contextual_menu.draw(&mut d);
	}


	fn show_point_menu(&mut self, id: i32) {
		if let Some(title) = self.point_menu.seek("title") {
			if let WidgetVariant::Label {text, ..} = title.get_variant() {
				*text = format!("Point properties {id}");
			}
		}
		self.point_menu.root.set_visible(true);
	}

	fn contextual_menu_events(&mut self) {
		
		let contextual_activations = self.contextual_menu.root.get_all_activations();

		if contextual_activations.contains(&"add point".to_string()) {
			
			// Adding point in world
			let mut new_point = Point::new(Vector2::new(self.rl_handle.get_mouse_position().x, self.rl_handle.get_mouse_position().y));
			new_point.set_trail_visibility(true);
			self.world.push(new_point);

			// Adding point handle in inspector
			match self.inspector.seek("point scroll") {
				Some(s) => {
					let children_count = s.get_children_count(1u32);
					let h =  children_count as f32 * 0.1f32 - 0.4f32;
					s.add_child_inplace(
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
					)
				},
				None => println!("Error: No scroll menu in inspector, what happened ?")
			}
			
		}

		if self.rl_handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
			self.contextual_menu.bounds.center = self.rl_handle.get_mouse_position() + self.contextual_menu.bounds.size/2f32;
			self.contextual_menu.root.set_visible(true);
		}
		if self.rl_handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && !self.contextual_menu.bounds.contains(self.rl_handle.get_mouse_position()) {
			self.contextual_menu.root.set_visible(false);
		}
	}

	fn inspector_events(&mut self) {
		let inspector_activations = self.inspector.root.get_all_activations();

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
			!self.point_menu.bounds.contains(self.rl_handle.get_mouse_position()) &&
			self.rl_handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) &&
			!handle_activated 
		{
			self.point_menu.root.set_visible(false);
			self.selected_point = -1i32;
		}
	}

	fn point_menu_events(&mut self) {

		let point_menu_activations = self.point_menu.root.get_all_activations();

		if point_menu_activations.contains(&"apply forces".to_string()) {
			match (
				Tokenizer::tokenize(&self.point_menu.root.get_entry_in_tree("set ax").unwrap_or("0".to_string())),
				Tokenizer::tokenize(&self.point_menu.root.get_entry_in_tree("set ay").unwrap_or("0".to_string()))
			) {
				(Ok(tx), Ok(ty)) => {
					match self.world.get_mut(self.selected_point as usize) {
						Some(point) => {
							// TODO: ADD FORCE
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