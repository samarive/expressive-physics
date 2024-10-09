//! Contient le nécessaire pour construire des interfaces graphiques simplement.
//! [ATTENTION]
//! Ce fichier est actuellement invalide !

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
	background: Color,
	foreground: Color
}


/// Représente un élément de l'interface graphique utilisateur (bouton, text, champ d'entrée, etc.)
///
/// [Example]
/// ```
///	use raylib::prelude::*;
///
///	fn main() {
///		let mut frame = Widget::new(
///			Layout::new(
///				Vector2::new(0f32, 0f32),
///				Vector2::new(0.9f32, 0.9f32)
///			),
///			&Frame {outline_width = 16f32}
///		);
///		frame.add_child(
///			Widget::new(
///				Layout::new(
///					Vector2::new(0f32, 0f32),
///					Vector2::new(0.8f32, 0.3f32)
///				)
///			),
///			Label {text: "Hello World!".to_string()}
///		);
///
///		let (rl, thread) = raylib::init().size(800, 450).build();
///		rl.set_target_fps(60);
///
///		while !rl.window_should_close() {
///			let mut d = rl.begin_drawing(&thread);
///			frame.draw_tree(
///				&Layout::new(Vector2::new(0f32, 0f32), Vector2::new(800f32, 450f32)),
///				&d
///			);
///		}
///	}
/// ```
pub struct Widget {
	layout: Layout,
	children: Vec::<Box::<Widget>>
}

impl Widget {
	pub fn new(layout: Layout) -> Widget{
		Widget {
			layout,
			children: Vec::<Box::<Widget>>::new()
		}
	}


	pub fn draw_tree(&self, parent_layout: &Layout, rl_handle: &mut RaylibDrawHandle) {
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

		rl_handle.draw_rectangle_v(true_coords.center - true_coords.size / 2f32, true_coords.size, Color::BLUE);

		for child in self.children.iter() {
			child.draw_tree(&true_coords, rl_handle);
		}
	}

	pub fn add_child(&mut self, w: Widget) {
		self.children.push(Box::new(w));
	}
}