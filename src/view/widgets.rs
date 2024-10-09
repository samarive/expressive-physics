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

trait WidgetVariant {
	fn handle_events(&mut self, bounding_box: &Layout, handle: &RaylibHandle) {}
	fn draw(&self, bounding_box: &Layout, style: &Style, handle: &mut RaylibDrawHandle);
}

pub struct Label {
	text: String
}
pub struct Frame {
	outline_width: f32
}
impl WidgetVariant for Label {
	fn draw(&self, bounding_box: &Layout, style: &Style, handle: &mut RaylibDrawHandle) {
		handle.draw_rectangle_v(bounding_box.center - bounding_box.size / 2f32, bounding_box.size, style.background);
		handle.draw_text(&self.text, bounding_box.center.x as i32 - bounding_box.size.x as i32/2i32, bounding_box.center.y as i32 + bounding_box.size.y as i32/2i32, 16i32, style.foreground);
	}
}
impl WidgetVariant for Frame {
	fn draw(&self, bounding_box: &Layout, style: &Style, handle: &mut RaylibDrawHandle) {
		handle.draw_rectangle_v(bounding_box.center - bounding_box.size / 2f32, bounding_box.size, style.background);
		handle.draw_rectangle_v(bounding_box.center - bounding_box.size / 2f32 + Vector2::new(self.outline_width, self.outline_width), bounding_box.size - Vector2::new(self.outline_width, self.outline_width) * 2f32, style.foreground);
	}
}


pub struct Widget<T: WidgetVariant> {
	layout: Layout,
	variant: Box::<T>,
	children: Vec::<Box::<T>>
}

impl <T: WidgetVariant>Widget<T> {
	pub fn new(layout: Layout, variant: T) -> Widget<T>{
		Widget {
			layout,
			variant: Box::new(variant),
			children: Vec::<Box::<T>>::new()
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

		// rl_handle.draw_rectangle_v(true_coords.center - true_coords.size / 2f32, true_coords.size, self.color);
		self.variant.draw(&true_coords, &Style {background: Color::GREEN, foreground: Color::BLUE}, &mut rl_handle);

		for child in self.children.iter() {
			child.draw_tree(&true_coords, rl_handle);
		}
	}

	pub fn add_child(&mut self, w: Widget<T>) {
		self.children.push(Box::new(w));
	}
}