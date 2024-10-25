//! Encapsule tout le code nécessaire au lancement et à la gestion de l'application.

use std::collections::HashMap;
use std::rc::Rc;
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

	forces: HashMap::<String, Rc::<Force>>,
	selected_force: String,

	rl_handle: RaylibHandle,
	rl_thread: RaylibThread,
	
	// TODO: Put all of those in a struct with an iterator
	// so that we could just iterate when calling the same function
	// on every WidgetTree.
	inspector: WidgetTree,
	force_menu: WidgetTree,
	contextual_menu: WidgetTree,
	force_inspector: WidgetTree,
	force_naming: WidgetTree,

	force_menu_just_appeared: bool
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

			forces: HashMap::<String, Rc::<Force>>::new(),
			selected_force: String::new(),

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
			force_menu: WidgetTree::new(
				Self::build_default_force_menu(),
				Layout::new(Vector2::new(400f32, 225f32), Vector2::new(600f32, 200f32))
			),
			force_inspector: WidgetTree::new(
				Self::build_default_force_inspector(),
				Layout::new(Vector2::new(700f32, 225f32), Vector2::new(200f32, 400f32))
			),
			force_naming: WidgetTree::new(
				Self::build_default_force_naming(),
				Layout::new(Vector2::new(400f32, 225f32), Vector2::new(600f32, 100f32))
			),

			force_menu_just_appeared: false

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
		.add_child(
			Widget::new(
				Layout::new(Vector2::new(0f32, -0.34f32), Vector2::new(1f32, 0.1f32)),
				WidgetVariant::Button {
					state: ButtonState::Rest
				}
			).style(Style::default().foreground(Color::GREEN).action(Color::GRAY)).id("add force".to_string())
			.add_child(
				Widget::new(
					Layout::new(Vector2::new(0f32, 0f32), Vector2::new(1f32, 1f32)),
					WidgetVariant::Label {text: "Add force".to_string(), font_size: 16i32}
				).style(Style::default().background(Color::new(0, 0, 0, 0)))
			)

		)

	}

	fn build_default_force_menu() -> Widget {
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
				WidgetVariant::Label {
					text: String::new(),
					font_size: 32i32
				}
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
			.id("apply".to_string())
			.add_child(
				Widget::new(
					Layout::new(Vector2::new(0f32, 0f32), Vector2::new(0.9f32, 0.9f32)),
					WidgetVariant::Label {text: "Apply !".to_string(), font_size: 16i32}
				).style(Style::default().background(Color::new(0, 0, 0, 0)))
			)
		)
		
	}

	fn build_default_force_inspector() -> Widget {
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
			.id("force scroll".to_string())
		)
	}

	fn build_default_force_naming() -> Widget {
		Widget::new(
			Layout::default(),
			WidgetVariant::Frame {outline_thickness: 1f32}
		)
		.style(Style::default()
		.background(Color::BLACK)
		.foreground(Color::new(255, 255, 255, 100)))
		.hidden()
		.add_child(
			Widget::new(
				Layout::new(
					Vector2::new(0f32, 0f32),
					Vector2::new(0.8f32, 0.3f32)
				),
				WidgetVariant::TextInput {
					placeholder: String::from("Force name"),
					text: String::new(),
					registered: true,
					selected: false,
					cursor: 0u32
				}
			)
			.id(String::from("name"))
		)
		.add_child(
			Widget::new(
				Layout::new(
					Vector2::new(-0.25f32, 0.35f32),
					Vector2::new(0.2f32, 0.2f32)
				),
				WidgetVariant::Button {state: ButtonState::Rest}
			)
			.style(Style::default().action(Color::GREEN))
			.id(String::from("create"))
			.add_child(
				Widget::new(
					Layout::default(),
					WidgetVariant::Label {text: String::from("Create"), font_size: 16i32}
				)
			)
		)
		.add_child(
			Widget::new(
				Layout::new(
					Vector2::new(0.25f32, 0.35f32),
					Vector2::new(0.2f32, 0.2f32)
				),
				WidgetVariant::Button {state: ButtonState::Rest}
			)
			.id(String::from("cancel"))
			.style(Style::default().action(Color::RED))
			.add_child(
				Widget::new(
					Layout::default(),
					WidgetVariant::Label {text: String::from("Cancel"), font_size: 16i32}
				)
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
		self.force_inspector.check_event(&mut self.rl_handle);
		self.force_menu.check_event(&mut self.rl_handle);
		self.force_naming.check_event(&mut self.rl_handle);
		self.contextual_menu.check_event(&mut self.rl_handle);

		// Special behaviours
		self.contextual_menu_events();
		self.inspector_events();
		self.force_inspector_events();
		self.force_menu_events();
		self.force_naming_events();
				
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
		self.force_inspector.draw(&mut d);
		self.force_menu.draw(&mut d);
		self.force_naming.draw(&mut d);
		self.contextual_menu.draw(&mut d);
	}


	fn show_force_menu(&mut self, name: String) {
		if let Some(title) = self.force_menu.seek("title") {
			if let WidgetVariant::Label {text, ..} = title.get_variant() {
				*text = name.clone();
				self.selected_force = name.clone();
			}
		}

		match self.forces.get(&name) {
			Some(f) => {
				if let Some(ax) = self.force_menu.seek("set ax") {
					if let WidgetVariant::TextInput {text, cursor, ..} = ax.get_variant() {
						*text = Tokenizer::untokenize(&f.x);
						*cursor = text.len() as u32;
					}
				}
				if let Some(ay) = self.force_menu.seek("set ay") {
					if let WidgetVariant::TextInput {text, cursor, ..} = ay.get_variant() {
						*text = Tokenizer::untokenize(&f.y);
						*cursor = text.len() as u32;
					}
				}
			},
			None => println!("Error: force {} doen't exist in model.", name)
		}

		self.force_menu.root.set_visible(true);
		self.force_menu_just_appeared = true;
	}

	fn contextual_menu_events(&mut self) {
		
		let contextual_activations = self.contextual_menu.root.get_all_activations();

		if contextual_activations.contains(&String::from("add force")) {
			self.force_naming.root.set_visible(true);
			self.contextual_menu.root.set_visible(false);
		}
		if contextual_activations.contains(&String::from("add point")) {
			
			// Adding point in world
			let mut new_point = Point::new(Vector2::new(self.rl_handle.get_mouse_position().x, self.rl_handle.get_mouse_position().y));
			new_point.set_trail_visibility(true);
			for f in self.forces.iter() {
				if let Err(e) = new_point.add_force(Rc::clone(&f.1)) {
					println!("Error while adding force on newly created point : {e}.");
				}
			}
			self.world.push(new_point);

			// Adding point handle in inspector
			match self.inspector.seek("point scroll") {
				Some(s) => {
					Self::add_button_to_scroll(s, |x: u32| format!("point{x}"));
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

	fn force_inspector_events(&mut self) {
		let activations = self.force_inspector.root.get_all_activations();
		
		for id in activations.iter() {
			self.show_force_menu(id.clone());
		}
	}

	fn inspector_events(&mut self) {
		let inspector_activations = self.inspector.root.get_all_activations();

		for id in inspector_activations.iter() {
			if id.starts_with("point") {
				match id[5..].parse::<i32>() {
					Ok(v) =>  {
						self.selected_point = v;
						// TODO: Show point menu
					}
					Err(_) => println!("Ill formated point name, expected i32 after column 5.")
				}
			}
		}
	}

	fn force_menu_events(&mut self) {

		let force_menu_activations = self.force_menu.root.get_all_activations();

		if force_menu_activations.contains(&"apply".to_string()) {
			match (
				Tokenizer::tokenize(&self.force_menu.root.get_entry_in_tree("set ax").unwrap_or("0".to_string())),
				Tokenizer::tokenize(&self.force_menu.root.get_entry_in_tree("set ay").unwrap_or("0".to_string()))
			) {
				(Ok(tx), Ok(ty)) => {
					self.forces.insert(self.selected_force.clone(), Rc::new(Force {x:tx, y:ty}));
					self.selected_force.clear();
					self.force_menu.root.set_visible(false);
				}
				(Err(e), _) => println!("Error on X expression : {e:?}"),
				(_, Err(e)) => println!("Error on Y expression : {e:?}")
			}
					
		}

		if 
			!self.force_menu.bounds.contains(self.rl_handle.get_mouse_position()) &&
			self.rl_handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) &&
			!self.force_menu_just_appeared
		{
			self.force_menu.root.set_visible(false);
		} else {
			self.force_menu_just_appeared = false;
		}
	}

	fn force_naming_events(&mut self) {
		let activations = self.force_naming.root.get_all_activations();

		for id in activations {
			if id == "cancel" {
				self.force_naming.root.set_visible(false);
			} else if id == "create" {
				match self.force_inspector.seek("force scroll") {
					Some (s) => {
						let name = match self.force_naming.seek("name") {
							Some(n) => {
								if let WidgetVariant::TextInput{text, cursor, ..} = n.get_variant() {
									let t = text.clone();
									text.clear();
									*cursor = 0u32;
									t
								} else {
									println!("Error: Widget of ID 'name' is not a TextInput.");
									String::from("Unknown")
								}
							},
							None => String::from("Unknown")
						};
						self.forces.insert(name.clone(), Rc::new(Force::new()));
						Self::add_button_to_scroll(s, |_: u32| name.clone());
						self.force_naming.root.set_visible(false);
					},
					None => println!("Error: No scroll menu in force inspector, what happened ?")
				}
			}
		}
	}

	fn add_button_to_scroll(s: &mut Widget, t: impl Fn(u32) -> String) {
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
			.id(t(children_count))
			.add_child(
				Widget::new(
					Layout::new(
						Vector2::new(0f32, 0f32),
						Vector2::new(1f32, 1f32)
					),
					WidgetVariant::Label {text: t(children_count), font_size: 16i32}
				).style(Style::default().background(Color::new(0, 0, 0, 0)))
			)	
		)
	}

}