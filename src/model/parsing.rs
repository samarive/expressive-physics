use super::tokening::*;

use std::collections::HashMap;

#[derive(Debug)]
pub enum ParsingError {
	DivisionByZero,
	UnknownOperator (char),
	NoOperator,
	NotAValue,
	MissingVariableInContext (String)
}

pub type VariableContext = HashMap::<String, f32>;

struct OperatorPosition (char, usize);

pub struct Parser;
impl Parser {
	
	/// Gate to recursive parsing
	pub fn parse(tokens: &Vec::<Token>, context: &VariableContext) -> Result<f32, ParsingError> {
		Self::recursive_parsing(tokens, context, 0usize, tokens.len())
	}

	/// Locates the last operation in order of priority and recursively resolves
	/// the left and right operands to perform a simple Operand Operator Operand
	/// calculation.
	fn recursive_parsing(tokens: &Vec::<Token>, context: &VariableContext, start: usize, end: usize) -> Result<f32, ParsingError> {

		// Should never happen when this method is called recursively.
		// Hence, this block will be executed if and only if the user
		// explicitly passes an empty vector of token as tokens.
		if start >= end {
			dbg!("Warning : Came accross an empty token vector (or filled with parenthesis) in recursive_parsing.");
			return Ok(0f32);
		}

		/*
		// Useful for debugging (delete only after rigorous testing)
		for i in start..end {
			print!("{:?} ", tokens[i]);
		}
		println!("");
		*/

		match Self::find_least_prior_operator(tokens, start, end) {
			Ok (op) => {
				// Split tokens in two at least prior operator position and calls this method recursively on each of those two parts. 
				match (Self::recursive_parsing(tokens, context, start, op.1), Self::recursive_parsing(tokens, context, op.1 + 1, end)) {
					(Ok(left_hand), Ok(right_hand)) => {
						match op.0 {
							'+' => Ok(left_hand + right_hand),
							'-' => Ok(left_hand - right_hand),
							'/' => Ok(left_hand / right_hand),
							'*' => Ok(left_hand * right_hand),
							 _  => Err(ParsingError::UnknownOperator(op.0)) 
						}
					}, 
					// If either of the recursive calls fails, make this method fail.
					(Err(e), Ok(_)) | (Ok(_), Err(e)) | (Err(e), Err(_)) => Err(e)
				}
			},
			Err(ParsingError::NoOperator) => {
				
				// I) The next piece of code suposes that at this point the tokens are either :
				//		1) (*[Variable | Value]
				//      2) [Variable | Value])*
				//		3) [Variable | Value]
				// If it is not 1), then it is 2) or 3) which are both parsed the same way.
				let mut token_then_only_prths = true; // which is in fact as well true when treating 2) as when treating 3).
				for i in start..end {
					if i != start && tokens[i] != Token::Parenthesis(false) {
						token_then_only_prths = false;
					}
				}
				
				let contains_value: &Token; // Points to the token that contains the value or the variable
				
				if token_then_only_prths {
					contains_value = &tokens[start];
				}
				else {
					contains_value = &tokens[end-1];
				}

				// Unboxing the value
				match contains_value {
					Token::Value(val) => Ok(*val),
					Token::Variable(name) => {
						match context.get(name) {
							Some(val) => Ok(*val),
							None      => Err(ParsingError::MissingVariableInContext(name.clone()))
						}
					}
					_ => Err(ParsingError::NotAValue) // This raises if the assumption I) is false (hopefully, never)
				}
			},
			Err(e) => Err(e)
		}
	}

	fn find_least_prior_operator(tokens: &Vec::<Token>, start: usize, end: usize) -> Result<OperatorPosition, ParsingError> {

		// The higher the number, the higher the priority
		let mut priorities = HashMap::<char, i32>::new();
		priorities.insert('+', 0);
		priorities.insert('-', 0);
		priorities.insert('*', 1);
		priorities.insert('/', 1);

		let mut r = OperatorPosition ('\0', 0usize); // Result variable
		let mut lowest_prio = 0i32;
		let mut parenthesis_level = 0i32;

		for i in start..end {

			match tokens[i] {
				Token::Operator(c) => {
					
					match priorities.get(&c) {
						Some (prio) => {
							
							// /!\ Dirty hack /!\
							// Works as long as no operator has a priority greater than 1000
							if *prio + parenthesis_level * 1000i32 <= lowest_prio || r.0 == '\0' {
								r.0 = c;
								r.1 = i;
								lowest_prio = *prio + parenthesis_level * 1000i32;
							}
						},
						None => {
							return Err(ParsingError::UnknownOperator (c));
						}
					}
				},
				Token::Parenthesis(true)  => parenthesis_level += 1i32,
				Token::Parenthesis(false) => parenthesis_level -= 1i32,
				_ => {}
			}
		}

		match r.0 {
			'\0' => Err(ParsingError::NoOperator),
			 _   => Ok(r)
		}
	}
} 