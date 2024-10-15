//! Contient le nécessaire pour construire des interfaces graphiques simplement.


use raylib::prelude::*;

#[derive(Debug)]
pub struct Layout {
	pub center: Vector2,
	pub size: Vector2
}
impl Layout {
	pub fn new(center: Vector2, size: Vector2) -> Self {
		Layout {
			center,
			size
		}
	}

	pub fn contains(&self, point: Vector2) -> bool {
		return point.x >= self.center.x - self.size.x / 2f32
		    && point.x <= self.center.x + self.size.x / 2f32
		    && point.y >= self.center.y - self.size.y / 2f32
		    && point.y <= self.center.y + self.size.y / 2f32
	}
}



pub struct Style {
	pub background: Color,
	pub foreground: Color,
	pub action: Color
}
impl Style {
	pub fn default() -> Self {
		Style {
			background: Color::WHITE,
			foreground: Color::BLACK,
			action: Color::GRAY
		}
	}

	pub fn background(mut self, c: Color) -> Self {
		self.background = c;
		self
	}

	pub fn foreground(mut self, c: Color) -> Self {
		self.foreground = c;
		self
	}

	pub fn action(mut self, c: Color) -> Self {
		self.action = c;
		self
	}
}

#[derive(Debug)]
pub enum ButtonState {
	Rest,
	Hovered,
	Activated {countdown: i32, handled: bool},
}

pub enum WidgetVariant {
	Frame {outline_thickness: f32},
	Label {text: String, font_size: i32},
	Button {state: ButtonState},
	TextInput {selected: bool, placeholder: String, text: String, registered: bool}
}


/// Représente un élément de l'interface graphique utilisateur (bouton, text, champ d'entrée, etc.)
///
/// # Exemple
/// ```
/// use raylib::prelude::*;
/// use view::widgets::*;
///
/// fn main() {
///     let mut w = Widget::new(
///         Layout::new(
///             Vector2::new(0f32, 0f32),
///             Vector2::new(0.9f32, 0.9f32)
///         ),
///         WidgetVariant::Frame {outline_thickness: 1f32}
///     );
///     w.add_child(
///         Widget::new(
///             Layout::new(
///                 Vector2::new(0f32, 0f32),
///                 Vector2::new(0.8f32, 0.3f32)
///             ),
///             WidgetVariant::Label {text: "Hello World!".to_string(), font_size: 16i32}
///         )
///     );
///
///     let widget_layout = Layout::new(Vector2::new(400f32, 225f32), Vector2::new(800f32, 450f32));
///
///     let (mut rl, thread) = raylib::init().size(800, 450).build();
///     rl.set_target_fps(60);
///
///     while !rl.window_should_close() {
///         w.check_event_in_tree(&widget_layout, &mut rl);
///	        let mut d = rl.begin_drawing(&thread);
///         d.clear_background(Color::RED);
///         w.draw_tree(
///             &widget_layout,
///             &mut d
///         );
///     }
/// }
/// ```
pub struct Widget {
	layout: Layout,
	variant: WidgetVariant,
	style: Style,
	children: Vec::<Widget>,
	hidden: bool,
	id: &'static str
}

impl Widget {
	pub fn new(layout: Layout, variant: WidgetVariant) -> Widget{
		Widget {
			layout,
			variant,
			style: Style::default(),
			children: Vec::<Widget>::new(),
			hidden: false,
			id: "Unknown"
		}
	}
	pub fn style(mut self, style: Style) -> Self {
		self.style = style;
		self
	} 

	pub fn toggle(&mut self) {
		self.hidden = !self.hidden;
	}

	pub fn hidden(mut self) -> Self{
		self.set_visible(false);
		self
	}
	pub fn id(mut self, id: &'static str) -> Self {
		self.id = id;
		self
	}

	pub fn set_visible(&mut self, a: bool) {
		self.hidden = !a;
	}

	pub fn is_hidden(&self) -> bool {
		self.hidden
	}

	pub fn get_id(&self) -> &str  {
		self.id
	}

	pub fn get_entry_in_tree(&mut self, id: &'static str) -> Option<String> {
		if self.id == id {
			if let WidgetVariant::TextInput {text, ..} = &mut self.variant {
				return Some(String::clone(text)); 
			}
		}

		for c in &mut self.children {
			if let Some(s) = c.get_entry_in_tree(id) {
				return Some(s);
			}
		}

		return None;
	}

	pub fn check_entry_in_tree(&mut self, id: &'static str) -> Option<String> {
		if self.id == id {
			if let WidgetVariant::TextInput {text, registered, ..} = &mut self.variant {
				if !*registered {
					*registered = true;
					return Some(String::clone(text));
				} 
			}
		}

		for c in &mut self.children {
			if let Some(s) = c.check_entry_in_tree(id) {
				return Some(s);
			}
		}

		return None;
	}

	pub fn check_activation_in_tree(&mut self, id: &'static str) -> bool{
		if self.id == id {
			if let WidgetVariant::Button{state: ButtonState::Activated{handled, ..}} = &mut self.variant {
				if !*handled {
					*handled = true;
					return true;
				} 
			}
		}
		
		for c in &mut self.children {
			if c.check_activation_in_tree(id) {
				return true;
			}
		}

		return false;
	}

	pub fn check_event_in_tree(&mut self, parent_layout: &Layout, rl: &mut RaylibHandle) {
		if self.hidden {return;}

		let true_coords = Layout::new(
			Vector2::new(
				parent_layout.center.x + self.layout.center.x * parent_layout.size.x,
				parent_layout.center.y + self.layout.center.y * parent_layout.size.y
			),
			Vector2::new(
				parent_layout.size.x * self.layout.size.x,
				parent_layout.size.y * self.layout.size.y
			)
		);

		let mouse = rl.get_mouse_position();
		match &mut self.variant {
			WidgetVariant::Label {text, ..} => {
				if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && true_coords.contains(mouse){
					println!("Label '{text}' clicked !");
				}
			},
			WidgetVariant::Button {state} => {
				if true_coords.contains(mouse) {
					if let ButtonState::Rest = *state {
						*state = ButtonState::Hovered;
					}
					if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
						*state = ButtonState::Activated {countdown: 8i32, handled: false};
						println!("Button clicked");
					}
				}
				else {
					if let ButtonState::Hovered = *state {
						*state = ButtonState::Rest;
					}
				}

				if let ButtonState::Activated{countdown, ..} = state {
					*countdown -= 1;
					if *countdown <= 0 {
						*state = ButtonState::Hovered;
					}
				}

			},
			WidgetVariant::TextInput {selected, text, registered, ..} => {
				if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && true_coords.contains(mouse) {
					*selected = true;
				}
				else if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
					*selected = false;
					*registered = false;
				}

				if *selected {
					match rl.get_char_pressed() {
						Some(c) => text.push(c),
						None => if let Some(key) = rl.get_key_pressed() {
							match key {
								KeyboardKey::KEY_BACKSPACE => {text.pop();},
								KeyboardKey::KEY_ENTER => {
									*selected = false;
									*registered = false;
								}
								_ => {}
							}
						}
					}
				}
			}
			_ => {}
		}

		for child in self.children.iter_mut() {
			child.check_event_in_tree(&true_coords, rl);
		}
	}

	pub fn draw_tree(&self, parent_layout: &Layout, draw_handle: &mut RaylibDrawHandle) {
		if self.hidden {return;}
		let true_coords = Layout::new(
			Vector2::new(
				parent_layout.center.x + self.layout.center.x * parent_layout.size.x,
				parent_layout.center.y + self.layout.center.y * parent_layout.size.y
			),
			Vector2::new(
				parent_layout.size.x * self.layout.size.x,
				parent_layout.size.y * self.layout.size.y
			)
		);

		let coords_rect = Rectangle::new(
			true_coords.center.x - true_coords.size.x / 2f32,
			true_coords.center.y - true_coords.size.y / 2f32,
			true_coords.size.x,
			true_coords.size.y
		);

		match &self.variant {
			WidgetVariant::Frame {outline_thickness} => {
				let outline = Vector2::new(*outline_thickness, *outline_thickness);

				draw_handle.draw_rectangle_rec(coords_rect, self.style.background);
				draw_handle.draw_rectangle_v(
					Vector2::new(coords_rect.x, coords_rect.y) + outline, true_coords.size - outline * 2f32,
					self.style.foreground
				);
			},
			WidgetVariant::Label {text, font_size} => {
				draw_handle.draw_rectangle_rec(coords_rect, self.style.background);
				draw_handle.draw_text(
					text,
					true_coords.center.x as i32 - (true_coords.size.x*0.9f32) as i32/2i32,
					true_coords.center.y as i32 - font_size/2i32, *font_size,
					self.style.foreground
				);
			},
			WidgetVariant::Button {state, ..} => {
				draw_handle.draw_rectangle_v(true_coords.center - true_coords.size / 2f32, true_coords.size,
					match state {
						ButtonState::Activated{..} => self.style.foreground,
						ButtonState::Hovered => self.style.action,
						ButtonState::Rest => self.style.background
					}
				);
			}
			WidgetVariant::TextInput {selected, placeholder, text, ..} => {
				draw_handle.draw_rectangle_rec(coords_rect, self.style.background);
				draw_handle.draw_text(
					if text.is_empty() {placeholder} else {text},
					coords_rect.x as i32 + (0.05 * coords_rect.width as f32) as i32, coords_rect.y as i32,
					coords_rect.height as i32,
					if text.is_empty() {self.style.action} else {self.style.foreground}
				);
				
				draw_handle.draw_rectangle_lines(
					coords_rect.x as i32, coords_rect.y as i32,
					coords_rect.width as i32, coords_rect.height as i32,
					if *selected {self.style.action} else {self.style.foreground}
				);
				
			}
		}

		for child in self.children.iter() {
			child.draw_tree(&true_coords, draw_handle);
		}
	}

	pub fn add_child(mut self, w: Widget) -> Self{
		self.children.push(w);
		self
	}
}


/// Permet d'iterer simplement au travers d'un arbre de widget.
/// # Exemple
/// ```
/// let w = Widget::new(
/// Layout::new(
///        Vector2::new(0f32, 0f32),
///        Vector2::new(1f32, 1f32)
///    ),
///    WidgetVariant::Frame {outline_thickness: 1f32}
/// )
///.style(
///   Style::default()
///     .background(Color::WHITE)
///     .foreground(Color::BLACK)
/// )
/// .add_child(
///     Widget::new(
///         Layout::new(
///             Vector2::new(0f32, 0f32),
///             Vector2::new(0.8f32, 0.3f32)
///         ),
///         WidgetVariant::Label {text: "Hello World!".to_string(), font_size: 16i32}
///     )
///     .style(
///         Style::default()
///         .background(Color::YELLOW)
///         .foreground(Color::RED)
///     )
/// );
/// 
/// for widget in WidgetTreeIterator::new(&w) {
///     println!("Ce widget est caché : {}", widget.is_hidden());
/// }
/// ```

pub struct WidgetTreeIterator<'a> {
	stack: Vec::<&'a Widget>
}

impl <'a>WidgetTreeIterator<'a> {
	pub fn new(w: &'a Widget) -> Self {
		WidgetTreeIterator {
			stack: vec![w]
		}
	}
}

impl <'a> Iterator for WidgetTreeIterator<'a> {
	type Item = &'a Widget;

	fn next(&mut self) -> Option<Self::Item> {
		let p = self.stack.pop();
		if let Some(w) = p {
			for c in w.children.iter() {
				self.stack.push(c);
			}
		}
		p
	}
}
