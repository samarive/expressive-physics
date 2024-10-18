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
			background: Color::new(255, 255, 255, 100),
			foreground: Color::BLACK,
			action: Color::new(100, 100, 100, 100)
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

#[derive(Debug)]
pub enum WidgetVariant {
	Frame {outline_thickness: f32},
	Label {text: String, font_size: i32},
	Button {state: ButtonState},
	TextInput {selected: bool, placeholder: String, text: String, cursor: u32, registered: bool},
	Scroll {offset: f32}
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
	id: String
}

impl Widget {

	// __________________________________Constructor______________________________________

	pub fn new(layout: Layout, variant: WidgetVariant) -> Widget{
		Widget {
			layout,
			variant,
			style: Style::default(),
			children: Vec::<Widget>::new(),
			hidden: false,
			id: String::from("Unknown")
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
	pub fn id(mut self, id: String) -> Self {
		self.id = id;
		self
	}

	// ___________________________________Setters__________________________________

	pub fn set_visible(&mut self, a: bool) {
		self.hidden = !a;
	}

	// __________________________________Getters______________________________________

	pub fn is_hidden(&self) -> bool {
		self.hidden
	}

	pub fn get_id(&self) -> &str  {
		&self.id
	}

	pub fn get_variant(&mut self) -> &mut WidgetVariant {
		&mut self.variant
	}

	// __________________________________Tree tools______________________________________

	pub fn seek_in_tree(&mut self, id: &'static str) -> Option<&mut Widget> {
		if self.id == id {
			return Some(self);
		}
		for c in self.children.iter_mut() {
			if let Some(w) = c.seek_in_tree(id) {
				return Some(w);
			}
		}
		return None;
	}

	/// Returns the content of the first TextInput of a given id encountered
	/// in depth-first iteration.
	/// # When to use ?
	/// This method is useful if you need the data of a text input only after another widget is activated or
	/// if you need the data of a text input in real time.
	pub fn get_entry_in_tree(&self, id: &'static str) -> Option<String> {
		if self.id == id {
			if let WidgetVariant::TextInput {text, ..} = &self.variant {
				return if text.is_empty() {Some("0".to_string())} else {Some(String::clone(text))}; 
			}
		}

		// Recursively repeats for each sub-tree (depth-first which is not ideal
		// in case of multiple widgets with the same id located at different heights
		// in tree)
		// TODO (very low priority) : Make it width first. (hint: an iterator would be OK !)
		for c in &self.children {
			if let Some(s) = c.get_entry_in_tree(id) {
				return Some(s);
			}
		}

		return None;
	}

	/// Returns the content of the first unregistered TextInput of a given id encountered
	/// in depth-first iteration and flag it as registered.
	/// # When to use ?
	/// Useful if you call this method multiple times and want it to truly execute only
	/// once per user validation.
	pub fn check_entry_in_tree(&mut self, id: &'static str) -> Option<String> {
		if self.id == id {
			if let WidgetVariant::TextInput {text, registered, ..} = &mut self.variant {
				if !*registered {
					*registered = true;
					return Some(String::clone(text));
				} 
			}
		}

		// Recursively repeats for each sub-tree (depth-first which is not ideal
		// in case of multiple widgets with the same id located at different heights
		// in tree)
		// TODO (very low priority) : Make it width first. (hint: an iterator would be OK !)
		for c in &mut self.children {
			if let Some(s) = c.check_entry_in_tree(id) {
				return Some(s);
			}
		}

		return None;
	}

	pub fn get_all_activations(&mut self) -> Vec::<String> {
		let mut r = Vec::<String>::new();
		
		if let WidgetVariant::Button{state: ButtonState::Activated{handled, ..}} = &mut self.variant {
			if !*handled {
				*handled = true;
				r.push(self.id.clone());
			} 
		}

		for c in self.children.iter_mut() {
			c.get_all_activations().iter().for_each(|x| r.push(x.clone()));
		}

		r
	}

	/// Check if a Button of a given id is not yet handled despite having been activated, if so, flag it
	/// as handled.
	/// # When to use ?
	/// Use it when implementing behaviour of a button of given id.
	pub fn check_activation_in_tree(&mut self, id: &'static str) -> bool{
		if self.id == id {
			if let WidgetVariant::Button{state: ButtonState::Activated{handled, ..}} = &mut self.variant {
				if !*handled {
					*handled = true;
					return true;
				} 
			}
		}
		
		// Recursively repeats for each sub-tree (depth-first which is not ideal
		// in case of multiple widgets with the same id located at different heights
		// in tree)
		// TODO (very low priority) : Make it width first. (hint: an iterator would be OK !)
		for c in &mut self.children {
			if c.check_activation_in_tree(id) {
				return true;
			}
		}

		return false;
	}

	/// Handles every event for every type of widget.
	/// Call this once per loop as it mutates frame-dependant data such as
	/// Buttons cooldown.
	pub fn check_event_in_tree(&mut self, parent_layout: &Layout, rl: &mut RaylibHandle) {
		if self.hidden {return;} // Disable events for hiddent widgets and their children

		let mut true_coords = self.get_true_coords(parent_layout);

		let mouse = rl.get_mouse_position();

		match &mut self.variant {
			WidgetVariant::Button {state} => {
				Self::handle_events_as_button(state, &true_coords, mouse, rl);
			},
			WidgetVariant::TextInput {selected, text, registered, cursor, ..} => {
				Self::handle_events_as_text_input(selected, text, registered, cursor, &true_coords, mouse, rl);
			},
			WidgetVariant::Scroll {offset} => {
				Self::handle_events_as_scroll(offset, &true_coords, mouse, rl);
				true_coords.center.y += *offset;
			}
			_ => {}
		}

		// Recursively check events for every child sub-tree (depth-first).
		for child in self.children.iter_mut() {
			child.check_event_in_tree(&true_coords, rl);
		}
	}

	/// Draws all widget and their children relative to their parents. 
	pub fn draw_tree(&self, parent_layout: &Layout, draw_handle: &mut RaylibDrawHandle) {
		if self.hidden {return;} // Hidden widgets and their children don't get drawn.
		
		let mut true_coords = self.get_true_coords(parent_layout);

		// true_coords as {left, top, width, height} format.
		let coords_rect = Rectangle::new(
			true_coords.center.x - true_coords.size.x / 2f32,
			true_coords.center.y - true_coords.size.y / 2f32,
			true_coords.size.x,
			true_coords.size.y
		);

		match &self.variant {
			WidgetVariant::Frame {outline_thickness} => {
				self.draw_as_frame(*outline_thickness, coords_rect, draw_handle);
			},
			WidgetVariant::Label {text, font_size} => {
				self.draw_as_label(text, *font_size, coords_rect, draw_handle);
			},
			WidgetVariant::Button {state, ..} => {
				self.draw_as_button(state, coords_rect, draw_handle);
			},
			WidgetVariant::TextInput {selected, placeholder, text, cursor, ..} => {
				self.draw_as_text_input(*selected, placeholder, text, cursor, coords_rect, draw_handle);
			},
			WidgetVariant::Scroll {offset} => {
				// TODO: Draw scrollbar on WidgetVariant::Scroll
				
				// Draw here, not after next line.
				// (or else scrollbar will be offseted too)
				true_coords.center.y += offset;
			}
		}

		// Recursively draw tree in depth first iteration.
		// TODO (very low priority) : Would be more sensible to draw the tree in width first order.
		for child in self.children.iter() {
			child.draw_tree(&true_coords, draw_handle);
		}
	}

	pub fn add_child(mut self, w: Widget) -> Self {
		self.children.push(w);
		self
	}

	pub fn add_child_inplace(&mut self, w: Widget) {
		self.children.push(w);
	}

	// Returns the number of children closer than depth.
	pub fn get_children_count(&self, depth: u32) -> u32{
		if depth == 0u32 {
			return 0u32;
		}

		let mut count = 0u32;

		for c in self.children.iter() {
			count += 1u32 + c.get_children_count(depth - 1);
		}

		count
	}

	// Private functions, mainly useful for organizing code complexity.

	/// true_coords are the on-screen coordinates as opposed to self.layout which are
	/// the coordinates relative to the parent coords.
	fn get_true_coords(&self, parent_layout: &Layout) -> Layout {
		Layout::new(
			Vector2::new(
				parent_layout.center.x + self.layout.center.x * parent_layout.size.x,
				parent_layout.center.y + self.layout.center.y * parent_layout.size.y
			),
			Vector2::new(
				parent_layout.size.x * self.layout.size.x,
				parent_layout.size.y * self.layout.size.y
			)
		)
	}

	// Drawing
	fn draw_as_frame(&self, outline_thickness: f32, coords_rect: Rectangle, draw_handle: &mut RaylibDrawHandle) {
		let outline = Vector2::new(outline_thickness, outline_thickness);

		// draw_handle.draw_rectangle_rec(coords_rect, self.style.background);
		draw_handle.draw_rectangle_lines_ex(coords_rect, outline_thickness, self.style.background);
		draw_handle.draw_rectangle_v(
			Vector2::new(coords_rect.x, coords_rect.y) + outline, Vector2::new(coords_rect.width, coords_rect.height) - outline * 2f32,
			self.style.foreground
		);
	}

	fn draw_as_label(&self, text: &str, font_size: i32, coords_rect: Rectangle, draw_handle: &mut RaylibDrawHandle) {
		draw_handle.draw_rectangle_rec(coords_rect, self.style.background);
		draw_handle.draw_text(
			text,
			(coords_rect.x + 0.1f32 * coords_rect.width) as i32,
			coords_rect.y as i32 + (coords_rect.height as i32 - font_size)/2i32,
			font_size,
			self.style.foreground
		);
	}

	fn draw_as_button(&self, state: &ButtonState, coords_rect: Rectangle, draw_handle: &mut RaylibDrawHandle) {
		draw_handle.draw_rectangle_rec(coords_rect,
			match state {
				ButtonState::Activated{..} => self.style.foreground,
				ButtonState::Hovered => self.style.action,
				ButtonState::Rest => self.style.background
			}
		);
	}

	fn draw_as_text_input(&self, selected: bool, placeholder: &str, text: &str, cursor: &u32, coords_rect: Rectangle, draw_handle: &mut RaylibDrawHandle) {
		draw_handle.draw_rectangle_rec(coords_rect, self.style.background);

		draw_handle.draw_text(
			if text.is_empty() {placeholder} else {text},
			coords_rect.x as i32 + (0.05 * coords_rect.width as f32) as i32, coords_rect.y as i32,
			coords_rect.height as i32,
			if text.is_empty() {self.style.action} else {self.style.foreground}
		);
		
		if selected {
			let cursor_offset = draw_handle.measure_text(
				&text[0..*cursor as usize],
				coords_rect.height as i32
			) as f32 + 0.05f32 * coords_rect.width as f32;

			draw_handle.draw_rectangle((coords_rect.x + cursor_offset) as i32, coords_rect.y as i32, 2i32, coords_rect.height as i32, Color::BLACK);
		}

		draw_handle.draw_rectangle_lines(
			coords_rect.x as i32, coords_rect.y as i32,
			coords_rect.width as i32, coords_rect.height as i32,
			if selected {self.style.action} else {self.style.foreground}
		);
	}

	// Events

	fn handle_events_as_scroll(offset: &mut f32, true_coords: &Layout, mouse: Vector2, rl: &mut RaylibHandle) {

		if true_coords.contains(mouse) {
			*offset += rl.get_mouse_wheel_move() * 10f32;
		}
	}

	fn handle_events_as_button(state: &mut ButtonState, true_coords: &Layout, mouse: Vector2, rl: &mut RaylibHandle) {
		if true_coords.contains(mouse) {
			if let ButtonState::Rest = *state {
				*state = ButtonState::Hovered;
			}
			if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
				*state = ButtonState::Activated {countdown: 8i32, handled: false};
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

	}
	fn handle_events_as_text_input(selected: &mut bool, text: &mut String, registered: &mut bool, cursor: &mut u32, true_coords: &Layout, mouse: Vector2, rl: &mut RaylibHandle) {
		if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && true_coords.contains(mouse) {
			if *selected {
				let left = true_coords.center.x - 9f32 * true_coords.size.x / 20f32;
				let character_size = rl.measure_text(text, true_coords.size.y as i32) as f32 / text.len() as f32;
				if left <= mouse.x {
					*cursor = ((mouse.x - left) / character_size) as u32;
					if *cursor as usize > text.len() {
						*cursor = text.len() as u32;
					}
				}
			} 
			else {
				*selected = true;
			}
		}
		else if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
			*selected = false;
			*registered = false;
		}

		if *selected {
			// Keyboard handling
			match rl.get_char_pressed() {
				Some(c) => {
					text.insert(*cursor as usize, c);
					*cursor += 1u32;
				},
				None => if let Some(key) = rl.get_key_pressed() {
					match key {
						KeyboardKey::KEY_BACKSPACE => {
							if *cursor != 0u32 {
								*cursor -= 1u32;
								text.remove(*cursor as usize);
							}
						},
						KeyboardKey::KEY_ENTER => {
							*selected = false;
							*registered = false;
						},
						KeyboardKey::KEY_LEFT => {
							if *cursor != 0u32 {
								*cursor -= 1u32;
							}
						},
						KeyboardKey::KEY_RIGHT => {
							if *cursor as usize != text.len() {
								*cursor += 1u32;
							}
						}
						_ => {}
					}
				}
			}
		}
	}
}


/// Permet d'iterer simplement au travers d'un arbre de widget.
/// L'ordre utilisé est celui du parcours par profondeur.
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
