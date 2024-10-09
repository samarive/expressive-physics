//! Contient le nécessaire pour construire des interfaces graphiques simplement.

use raylib::prelude::*;

pub struct Layout {
	center: Vector2,
	size: Vector2
}
impl Layout {
	pub fn new(center: Vector2, size: Vector2) -> Self {
		Layout {
			center,
			size
		}
	}
}

pub struct Style {
	pub background: Color,
	pub foreground: Color
}
impl Style {
	pub fn default() -> Self {
		Style {
			background: Color::WHITE,
			foreground: Color::BLACK
		}
	}
}

pub enum WidgetVariant {
	Frame {outline_thickness: f32},
	Label {text: String, font_size: i32}
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
	children: Vec::<Widget>
}

impl Widget {
	pub fn new(layout: Layout, variant: WidgetVariant) -> Widget{
		Widget {
			layout,
			variant,
			style: Style::default(),
			children: Vec::<Widget>::new()
		}
	}
	pub fn style(mut self, style: Style) -> Self {
		self.style = style;
		self
	} 

	pub fn check_event_in_tree(&mut self, parent_layout: &Layout, rl: &mut RaylibHandle) {
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

		match &self.variant {
			WidgetVariant::Label {text, ..} => {
				let mouse = rl.get_mouse_position();
				if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && mouse.x > true_coords.center.x - true_coords.size.x/2f32 && mouse.x < true_coords.center.x + true_coords.size.x/2f32
				&& mouse.y > true_coords.center.y - true_coords.size.y/2f32 && mouse.y < true_coords.center.y + true_coords.size.y/2f32 {
					println!("Label '{text}' clicked !");
				}
			},
			_ => {}
		}

		for child in self.children.iter_mut() {
			child.check_event_in_tree(&true_coords, rl);
		}
	}

	pub fn draw_tree(&self, parent_layout: &Layout, draw_handle: &mut RaylibDrawHandle) {
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

		match &self.variant {
			WidgetVariant::Frame {outline_thickness} => {
				let outline = Vector2::new(*outline_thickness, *outline_thickness);

				draw_handle.draw_rectangle_v(true_coords.center - true_coords.size / 2f32, true_coords.size, self.style.background);
				draw_handle.draw_rectangle_v(true_coords.center - true_coords.size / 2f32 + outline, true_coords.size - outline * 2f32, self.style.foreground);
			},
			WidgetVariant::Label {text, font_size} => {
				draw_handle.draw_rectangle_v(true_coords.center - true_coords.size / 2f32, true_coords.size, self.style.background);
				draw_handle.draw_text(text, true_coords.center.x as i32 - (true_coords.size.x*0.9f32) as i32/2i32, true_coords.center.y as i32 - font_size/2i32, *font_size, self.style.foreground);
			}
		}

		for child in self.children.iter() {
			child.draw_tree(&true_coords, draw_handle);
		}
	}

	pub fn add_child(&mut self, w: Widget) {
		self.children.push(w);
	}
}