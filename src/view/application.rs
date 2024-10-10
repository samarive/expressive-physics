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
	inspector: Widget,
	contextual_menu: Widget,
	contextual_menu_layout: Layout
}

impl Application {
	pub fn realize(world: World) -> Self {
		let (mut rl_handle, rl_thread) = raylib::init()
										.size(800, 450)
										.title("Expressive Physics")
										.build();
		rl_handle.set_target_fps(60);
		let mut r = Application {
			world,
			rl_handle,
			rl_thread,
			inspector: Widget::new(Layout::new(Vector2::new(0f32, 0f32), Vector2::new(1f32, 1f32)), WidgetVariant::Frame { outline_thickness: 1f32}).style(Style {background: Color::BLACK, foreground: Color::GRAY}),
			contextual_menu: Widget::new(Layout::new(Vector2::new(0f32, 0f32), Vector2::new(1f32, 1f32)), WidgetVariant::Frame {outline_thickness: 0f32}).hidden(),
			contextual_menu_layout: Layout::new(Vector2::new(600f32, 150f32), Vector2::new(100f32, 200f32))
		};

		r.inspector.add_child(Widget::new(Layout::new(Vector2::new(0f32, 0f32), Vector2::new(0.8f32, 0.3f32)), WidgetVariant::Label{text: "Hello World!".to_string(), font_size: 24i32}).style(Style {background: Color::RED, foreground: Color::BLUE}));
		r.contextual_menu.add_child(Widget::new(Layout::new(Vector2::new(0f32, -0.45f32), Vector2::new(1f32, 0.1f32)), WidgetVariant::Label{text: "Ajouter un point".to_string(), font_size: 10i32}).style(Style::default().background(Color::GRAY)));

		r
	}

	pub fn mainloop(&mut self) {
		while !self.rl_handle.window_should_close() {
			
			self.inspector.check_event_in_tree(&Layout::new(Vector2::new(100f32, 225f32), Vector2::new(200f32, 400f32)), &mut self.rl_handle);
			self.contextual_menu.check_event_in_tree(&self.contextual_menu_layout, &mut self.rl_handle);

			if self.rl_handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
				self.contextual_menu_layout.center = self.rl_handle.get_mouse_position() + self.contextual_menu_layout.size/2f32;
				self.contextual_menu.set_visible(true);
			}
			if self.rl_handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && !self.contextual_menu_layout.contains(self.rl_handle.get_mouse_position()) {
				self.contextual_menu.set_visible(false);
			}

			let mut d = self.rl_handle.begin_drawing(&self.rl_thread);

			d.clear_background(Color::WHITE);
			for point in self.world.iter() {
				d.draw_circle_v(point.position(), 5f32, Color::BLACK);
			}

			self.inspector.draw_tree(&Layout::new(Vector2::new(100f32, 225f32), Vector2::new(200f32, 400f32)), &mut d);
			self.contextual_menu.draw_tree(&self.contextual_menu_layout, &mut d);
		}
	}

}