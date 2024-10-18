#[derive(Debug, PartialEq)]
pub enum Token {
	Value (f32),
	Variable (String),
	Parenthesis (bool), // true: Open, false: Closed
	Operator(char)
}

#[derive(Debug)]
pub enum TokenizerError {
	UnmatchedParenthesis(usize),
	UnexpectedParenthesis(usize),
	UnexpectedDot(usize),
	UnexpectedOperator(usize),
	TooManyDots(usize),
	InvalidCharacter(usize),
	InternalError(usize, String)
}

#[derive(Debug)]
pub enum TokenizerState {
	Initial,
	NumberBeforeDot,
	NumberAfterDot,
	VariableName,
	Error (TokenizerError)
}

pub struct Tokenizer;
impl Tokenizer {

	fn parse_buffer(s: &mut String) -> Option<Token>{
		if !(s.is_empty()) {
			let r = match s.parse::<f32>() {
				Ok (value) => Some(Token::Value(value)),
				Err(_)     => Some(Token::Variable(s.clone())) 
			};
			s.clear();
			r
		}
		else {
			None
		}
	}

	/// Tokenizes a raw &str and checks expression syntax
	pub fn tokenize(raw: &str) -> Result::<Vec::<Token>, TokenizerError> {
		let mut r = Vec::new();                  // Result vector
		let mut parenthesis_stack = 0u32;        // Checks if every opened parenthesis is closed
		let mut state = TokenizerState::Initial; // This function is a state machine
		let mut buffer = String::new();          // Contains a substring of raw, used for parsing

		for (i, c) in raw.chars().enumerate() {
			
			// Behaviours common to every state
			let mut should_push_parenthesis = false;
			if c == ' ' {
				continue;
			}
			else if c == ')' {
				if parenthesis_stack == 0 {
					state = TokenizerState::Error(TokenizerError::UnmatchedParenthesis(i))
				}
				else {
					// Delays the Token::Parenthesis registration to the end of the loop
					// to allow the state machine (i.e. the next match statement) to append
					// a token before the closing parenthesis one because a closing parenthesis
					// might mark the end of a parsing state.
					should_push_parenthesis = true;
					parenthesis_stack -= 1u32;
				}
			}

			// The state machine
			match state {
				TokenizerState::Initial => {
					state = match c {
						'0' ..= '9' => {
							buffer.push(c);
							TokenizerState::NumberBeforeDot
						},
						'a' ..= 'z' | 'A' ..= 'Z' => {
							buffer.push(c);
							TokenizerState::VariableName
						}
						'+' | '-' | '*' | '/' | '>' | '<' => {
							// Two operators in a row throws UnexpectedOperator.
							match r.last() {
								Some(Token::Operator(_)) => TokenizerState::Error(TokenizerError::UnexpectedOperator(i)),
								_ => {
									r.push(Token::Operator(c));
									TokenizerState::Initial
								}
							}
						},
						')' => TokenizerState::Initial, // The rest of this behaviour is already implemented in common behaviours at start of loop.
						'(' => {
							r.push(Token::Parenthesis(true));
							parenthesis_stack += 1u32;
							TokenizerState::Initial
						},
						'.' => TokenizerState::Error(TokenizerError::UnexpectedDot(i)),
						 _  => TokenizerState::Error(TokenizerError::InvalidCharacter(i)) 
					}
				},
				TokenizerState::NumberBeforeDot => {
					state = match c {
						'0' ..= '9' => {
							buffer.push(c);
							TokenizerState::NumberBeforeDot
						}
						'+' | '-' | '*' | '/' | '>' | '<' => {
							r.push(Self::parse_buffer(&mut buffer).unwrap());
							r.push(Token::Operator(c));
							TokenizerState::Initial
						},
						')' => {									
							r.push(Self::parse_buffer(&mut buffer).unwrap());
							TokenizerState::Initial								
						},
						'(' => {
							// TODO: Allow multiplication inference (ex: 3(1 + 2) instead of 3 * (1 + 2))
							// (If implemented here, multiplication inference should also be implemented
							//  in NumberAfterDot)
							TokenizerState::Error(TokenizerError::UnexpectedParenthesis(i))
						},
						'.' => {
							buffer.push(c);
							TokenizerState::NumberAfterDot
						},
						 _  => TokenizerState::Error(TokenizerError::InvalidCharacter(i)) 
					}
				},
				TokenizerState::NumberAfterDot => {
					state = match c {
						'0' ..= '9' => {
							buffer.push(c);
							TokenizerState::NumberAfterDot
						}
						'+' | '-' | '*' | '/' | '>' | '<' => {
							r.push(Self::parse_buffer(&mut buffer).unwrap());
							r.push(Token::Operator(c));
							TokenizerState::Initial
						},
						')' => {	
							r.push(Self::parse_buffer(&mut buffer).unwrap());
							TokenizerState::Initial
						},
						'(' => {
							// TODO: [See NumberBeforeDot arm of current match statement comment...]
							TokenizerState::Error(TokenizerError::UnexpectedParenthesis(i))
						},
						'.' => TokenizerState::Error(TokenizerError::TooManyDots(i)),
						 _  => TokenizerState::Error(TokenizerError::InvalidCharacter(i)) 
					}
				},
				TokenizerState::VariableName => {
					state = match c {
						'0' ..= '9' | 'a' ..= 'z' | 'A' ..= 'Z' => {
							buffer.push(c);
							TokenizerState::VariableName
						}
						'+' | '-' | '*' | '/' | '>' | '<' => {
							r.push(Self::parse_buffer(&mut buffer).unwrap());
							r.push(Token::Operator(c));
							TokenizerState::Initial
						},
						')' => {
							r.push(Self::parse_buffer(&mut buffer).unwrap());
							TokenizerState::Initial
						},
						'(' => {
							TokenizerState::Error(TokenizerError::UnexpectedParenthesis(i))
						},
						'.' => TokenizerState::Error(TokenizerError::UnexpectedDot(i)),
						 _  => TokenizerState::Error(TokenizerError::InvalidCharacter(i)) 
					}
				},
				TokenizerState::Error(e) => {
					return Err(e);
				}
			}

			if should_push_parenthesis {
				r.push(Token::Parenthesis(false));
			}
		}

		// If tokenizing ended by an error, we need to return it due to the 1 loop delay between
		// an error registration and its treatment in the automat.
		if let TokenizerState::Error(e) = state {
			return Err(e);
		}

		// If expression ends by a variable name, it is not parsed until now.
		if let Some(token) = Self::parse_buffer(&mut buffer) {
			r.push(token);
		}

		if parenthesis_stack != 0 {
			Err(TokenizerError::UnmatchedParenthesis(raw.len()))
		}
		else {
			Ok(r)
		}
	}
}
