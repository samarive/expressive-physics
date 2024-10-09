use raylib::math::Vector2;
use std::collections::HashMap;
use common_macros::hash_map;
use super::tokening::{Token, Tokenizer};
use super::parsing::Parser;

pub type World = Vec::<Point>;

pub struct Force {
	x: Vec::<Token>,
	y: Vec::<Token>
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
		let mut new_acceleration = Vector2::zero();
		for force in self.forces.iter() {
			new_acceleration += Vector2::new(
				Parser::parse(&force.1.x, &context).unwrap(),
				Parser::parse(&force.1.y, &context).unwrap()
			);
		}
		self.acceleration = new_acceleration;
		
		self.speed += self.acceleration;
	}

	pub fn add_force(&mut self, name: &String, force: Force) -> Result<(), String> {
		if Self::only_contains_valid_variables(&force.x) && Self::only_contains_valid_variables(&force.y) {
			self.forces.insert(name.clone(), force);
			Ok(())
		}
		else {
			Err(format!("Invalid variable in force expression."))
		}
	}

	fn only_contains_valid_variables(tokens: &Vec::<Token>) -> bool {
		const accepted_variables:[&str; 6]  = ["px", "py", "vx", "vy", "ax", "ay"];
		for token in tokens {
			if let Token::Variable(name) = token {
				if !accepted_variables.contains(&name.as_str()) {
					return false;
				}
			}
		}
		return true;
	}
}