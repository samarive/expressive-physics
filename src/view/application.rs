use raylib::prelude::*;
use super::super::model::physics::*;
use super::widgets::*;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Application {
	world: Rc::<RefCell::<World>>,
	rl_handle: RaylibHandle,
	rl_thread: RaylibThread,
	widget_root: Widget<Frame>
}

impl Application {
	pub fn realize(w: &Rc::<RefCell::<World>>) -> Self {
		let (mut rl_handle, rl_thread) = raylib::init()
										.size(800, 450)
										.title("Expressive Physics")
										.build();
		rl_handle.set_target_fps(60);
		let mut r = Application {
			world: Rc::clone(w),
			rl_handle,
			rl_thread,
			widget_root: Widget::new(Layout::new(Vector2::new(0f32, 0f32), Vector2::new(1f32, 1f32)), Frame {outline_width: 0f32})
		};

		r.widget_root.add_child(Widget::new(Layout::new(Vector2::new(0f32, 0f32), Vector2::new(0.8f32, 0.3f32)), &Label {text: "Default".to_string()}));

		r
	}

	pub fn mainloop(&mut self) {
		while !self.rl_handle.window_should_close() {
			let mut d = self.rl_handle.begin_drawing(&self.rl_thread);
			
			d.clear_background(Color::WHITE);
			for point in self.world.borrow().iter() {
				d.draw_circle_v(point.position(), 5f32, Color::BLACK);
			}

			self.widget_root.draw_tree(&Layout::new(Vector2::new(100f32, 100f32), Vector2::new(200f32, 200f32)), &mut d);
		}
	}

}