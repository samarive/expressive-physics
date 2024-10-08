pub mod expressive_physics {

	pub mod parser {

		#[derive(Debug)]
		pub enum Token {
			Value (f32),
			Variable (String),
			Parenthesis (bool), // true: Open, false: Closed
			Operator(char)
		}

		#[derive(Debug)]
		pub enum TokenizerError {
			UnmatchedParenthesis,
			UnexpectedParenthesis,
			UnexpectedDot,
			UnexpectedOperator,
			TooManyDots,
			InvalidCharacter,
			InternalError(String)
		}

		#[derive(Debug)]
		pub enum TokenizerState {
			Initial,
			NumberBeforeDot,
			NumberAfterDot,
			VariableName,
			Error (TokenizerError)
		}

		pub struct Parser;
		impl Parser {

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

			pub fn tokenize(raw: &str) -> Result::<Vec::<Token>, TokenizerError> {
				let mut r = Vec::new();
				let mut parenthesis_stack = 0u32;
				let mut state = TokenizerState::Initial;
				let mut buffer = String::new();

				for c in raw.chars() {
					if c == ' ' {
						continue;
					}
					match state {
						TokenizerState::Initial => {
							
							state = match c {
								'0' ..= '9' => {
									buffer.push(c);
									TokenizerState::NumberBeforeDot
								},
								'a' ..= 'z' => {
									buffer.push(c);
									TokenizerState::VariableName
								}
								'+' | '-' | '*' | '/' => {
									match r.last() {
										None | Some(Token::Operator(_)) => TokenizerState::Error(TokenizerError::UnexpectedOperator),
										Some(_) => {
											r.push(Token::Operator(c));
											TokenizerState::Initial
										}
									}
								},
								')' => {
									if parenthesis_stack == 0 {
										TokenizerState::Error(TokenizerError::UnmatchedParenthesis)
									}
									else {
										r.push(Token::Parenthesis(false));
										parenthesis_stack -= 1u32;
										TokenizerState::Initial
									}
								},
								'(' => {
									r.push(Token::Parenthesis(true));
									parenthesis_stack += 1u32;
									TokenizerState::Initial
								},
								'.' => TokenizerState::Error(TokenizerError::UnexpectedDot),
								 _  => TokenizerState::Error(TokenizerError::InvalidCharacter) 
							}
						},
						TokenizerState::NumberBeforeDot => {
							state = match c {
								'0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
									buffer.push(c);
									TokenizerState::NumberBeforeDot
								}
								'+' | '-' | '*' | '/' => {
									let next_state = match buffer.parse::<f32>() {
										Ok (value) => {
											r.push(Token::Value(value));
											TokenizerState::Initial
										},
										Err (_) => TokenizerState::Error(TokenizerError::InternalError("Invalid buffer in NumberBeforeDot state".to_string()))
									};
									buffer.clear();
									r.push(Token::Operator(c));
									next_state
								},
								')' => {
									if parenthesis_stack == 0 {
										TokenizerState::Error(TokenizerError::UnmatchedParenthesis)
									}
									else {
										parenthesis_stack -= 1u32;
										let next_state = match buffer.parse::<f32>() {
											Ok (value) => {
												r.push(Token::Value(value));
												TokenizerState::Initial
											},
											Err (_) => TokenizerState::Error(TokenizerError::InternalError("Invalid buffer in NumberBeforeDot state".to_string()))
										};
										buffer.clear();
										r.push(Token::Parenthesis(false));
										next_state
									}
								},
								'(' => {
									TokenizerState::Error(TokenizerError::UnexpectedParenthesis)
								},
								'.' => {
									buffer.push(c);
									TokenizerState::NumberAfterDot
								},
								 _  => TokenizerState::Error(TokenizerError::InvalidCharacter) 
							}
						},
						TokenizerState::NumberAfterDot => {
							state = match c {
								'0' ..= '9' => {
									buffer.push(c);
									TokenizerState::NumberAfterDot
								}
								'+' | '-' | '*' | '/' => {
									let next_state = match buffer.parse::<f32>() {
										Ok (value) => {
											r.push(Token::Value(value));
											TokenizerState::Initial
										},
										Err (_) => TokenizerState::Error(TokenizerError::InternalError("Invalid buffer in NumberAfterDot state".to_string()))
									};
									buffer.clear();
									r.push(Token::Operator(c));
									next_state
								},
								')' => {
									if parenthesis_stack == 0 {
										TokenizerState::Error(TokenizerError::UnmatchedParenthesis)
									}
									else {
										parenthesis_stack -= 1u32;
										let next_state = match buffer.parse::<f32>() {
											Ok (value) => {
												r.push(Token::Value(value));
												TokenizerState::Initial
											},
											Err (_) => TokenizerState::Error(TokenizerError::InternalError("Invalid buffer in NumberAfterDot state".to_string()))
										};
										buffer.clear();
										r.push(Token::Parenthesis(false));
										next_state
									}
								},
								'(' => {
									TokenizerState::Error(TokenizerError::UnexpectedParenthesis)
								},
								'.' => TokenizerState::Error(TokenizerError::TooManyDots),
								 _  => TokenizerState::Error(TokenizerError::InvalidCharacter) 
							}
						},
						TokenizerState::VariableName => {
							state = match c {
								'0' ..= '9' | 'a' ..= 'z' => {
									buffer.push(c);
									TokenizerState::VariableName
								}
								'+' | '-' | '*' | '/' => {
									r.push(Token::Variable(buffer.clone()));
									buffer.clear();
									r.push(Token::Operator(c));
									TokenizerState::Initial
								},
								')' => {
									if parenthesis_stack == 0 {
										TokenizerState::Error(TokenizerError::UnmatchedParenthesis)
									}
									else {
										parenthesis_stack -= 1u32;
										r.push(Token::Variable(buffer.clone()));
										buffer.clear();
										r.push(Token::Parenthesis(false));
										TokenizerState::Initial
									}
								},
								'(' => {
									TokenizerState::Error(TokenizerError::UnexpectedParenthesis)
								},
								'.' => TokenizerState::Error(TokenizerError::UnexpectedDot),
								 _  => TokenizerState::Error(TokenizerError::InvalidCharacter) 
							}
						},
						TokenizerState::Error(e) => {
							return Err(e);
						}
					}
				}

				if let Some(token) = Self::parse_buffer(&mut buffer) {
					r.push(token);
				}

				if parenthesis_stack != 0 {
					Err(TokenizerError::UnmatchedParenthesis)
				}
				else {
					Ok(r)
				}
			}
		}

	}

}