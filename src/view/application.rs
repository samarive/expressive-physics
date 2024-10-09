//! Encapsule tout le code nécessaire au lancement et à la gestion de l'application.

use raylib::prelude::*;
use super::super::model::physics::*;
use super::widgets::*;


/// Gère les evenements, les visuels et les simulations
/// # Exemple
/// ```
///use model::physics::*;
///use raylib::math::Vector2;
///
///
///fn main() {
///
///    let mut world = World::new();
///    world.push(Point::new(Vector2::new(400f32, 225f32)));
///
///    let mut app = Application::realize(world);
///
///    app.mainloop();
///}
/// ```
pub struct Application {
	world: World,
	rl_handle: RaylibHandle,
	rl_thread: RaylibThread,
	widget_root: Widget
}

impl Application {
	pub fn realize(world: World/*w: &Rc::<RefCell::<World>>*/) -> Self {
		let (mut rl_handle, rl_thread) = raylib::init()
										.size(800, 450)
										.title("Expressive Physics")
										.build();
		rl_handle.set_target_fps(60);
		let mut r = Application {
			world,
			rl_handle,
			rl_thread,
			widget_root: Widget::new(Layout::new(Vector2::new(0f32, 0f32), Vector2::new(1f32, 1f32)), WidgetVariant::Frame { outline_thickness: 1f32}).style(Style {background: Color::BLACK, foreground: Color::GRAY})
		};

		r.widget_root.add_child(Widget::new(Layout::new(Vector2::new(0f32, 0f32), Vector2::new(0.8f32, 0.3f32)), WidgetVariant::Label{text: "Hello World!".to_string(), font_size: 24i32}).style(Style {background: Color::RED, foreground: Color::BLUE}));

		r
	}

	pub fn mainloop(&mut self) {
		while !self.rl_handle.window_should_close() {
			
			self.widget_root.check_event_in_tree(&Layout::new(Vector2::new(100f32, 225f32), Vector2::new(200f32, 400f32)), &mut self.rl_handle);

			let mut d = self.rl_handle.begin_drawing(&self.rl_thread);
			
			d.clear_background(Color::WHITE);
			for point in self.world.iter() {
				d.draw_circle_v(point.position(), 5f32, Color::BLACK);
			}

			self.widget_root.draw_tree(&Layout::new(Vector2::new(100f32, 225f32), Vector2::new(200f32, 400f32)), &mut d);
		}
	}

}