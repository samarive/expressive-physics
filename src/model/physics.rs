use raylib::math::Vector2;
use std::collections::HashMap;
use common_macros::hash_map;
use super::tokening::Token;
use super::parsing::Parser;

pub type World = Vec::<Point>;

pub struct Force {
	pub x: Vec::<Token>,
	pub y: Vec::<Token>
}

pub struct Point {
	position: Vector2,
	speed: Vector2,
	acceleration: Vector2,
	forces: HashMap::<String, Force>
}

impl Point {
	pub fn new(position: Vector2) -> Point{
		Point {
			position,
			speed: Vector2::zero(),
			acceleration: Vector2::zero(),
			forces: HashMap::<String, Force>::new()
		}
	}

	pub fn position(&self) -> Vector2 {
		self.position
	}

	pub fn simulate(&mut self) {
		self.position += self.speed;
		
		let context = hash_map!{
			"px".to_string() => self.position.x,
			"py".to_string() => self.position.y,
			"vx".to_string() => self.speed.x,
			"vy".to_string() => self.speed.y,
			"ax".to_string() => self.acceleration.x,
			"ay".to_string() => self.acceleration.y
		};

		// Summing into this variable in order to be able to access current acceleration
		// rather than access a mid-sumation temporary acceleration 
		let mut new_acceleration = Vector2::zero();
		
		// Summing forces
		for force in self.forces.iter() {
			new_acceleration += Vector2::new(
				match Parser::parse(&force.1.x, &context) {
					Ok(v) => v,
					Err(e) => {
						println!("Error while summing x forces : {e:?}.");
						0f32
					}
				},
				match Parser::parse(&force.1.y, &context) {
					Ok(v) => v,
					Err(e) => {
						println!("Error while summing y forces : {e:?}.");
						0f32
					}
				}
			);
		}
		self.acceleration = new_acceleration;
		
		self.speed += self.acceleration;
	}

	pub fn add_force(&mut self, name: &str, force: Force) -> Result<(), String> {
		if Self::only_contains_valid_variables(&force.x) && Self::only_contains_valid_variables(&force.y) {
			self.forces.insert(name.to_string(), force);
			Ok(())
		}
		else {
			Err("Invalid variable in force expression.".to_string())
		}
	}

	fn only_contains_valid_variables(tokens: &Vec::<Token>) -> bool {
		const ACCEPTED_VARIABLES:[&str; 6]  = ["px", "py", "vx", "vy", "ax", "ay"];
		for token in tokens {
			if let Token::Variable(name) = token {
				if !ACCEPTED_VARIABLES.contains(&name.as_str()) {
					return false;
				}
			}
		}
		true
	}
}