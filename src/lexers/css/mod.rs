// ---- DON'T EDIT THIS IS AUTO GENERATED CODE ---- //
pub mod token;
pub mod render;

pub struct Lexer {
	input: Vec<char>,
	pub position: usize,
	pub read_position: usize,
	pub ch: char
}

fn is_letter(ch: char) -> bool {
	'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
	'0' <= ch && ch <= '9'
}

impl Lexer {
	pub fn new(input: Vec<char>) -> Self {
		Self {
			input,
			position: 0,
			read_position: 0,
			ch: '0'
		}
	}

	pub fn read_char(&mut self) {
		if self.read_position >= self.input.len() {
			self.ch = '0';
		} else {
			self.ch = self.input[self.read_position];
		}
		self.position = self.read_position;
		self.read_position = self.read_position + 1;
	}

	pub fn next_token(&mut self) -> token::Token {
		let read_identifier = |l: &mut Lexer| -> Vec<char> {
			let position = l.position;
			while l.position < l.input.len() && is_letter(l.ch) {
				l.read_char();
			}
			l.input[position..l.position].to_vec()
		};

		let read_string = |l: &mut Lexer| -> Vec<char> {
			let position = l.position;
			l.read_char();
			while l.position < l.input.len() && l.ch != '"' {
				l.read_char();
			}
			l.read_char();
			l.input[position..l.position].to_vec()
		};

		let read_number = |l: &mut Lexer| -> Vec<char> {
			let position = l.position;
			while l.position < l.input.len() && is_digit(l.ch) {
				l.read_char();
			}
			l.input[position..l.position].to_vec()
		};

		let read_slash_comment = |l: &mut Lexer| -> Vec<char> {
			let position = l.position;
			while l.position < l.input.len() {
				l.read_char();
				if l.input[l.position+1] == '\n' {
					break;
				}
			}
			l.input[position..l.position+1].to_vec()
		};

		let read_slash_star_comment = |l: &mut Lexer| -> Vec<char> {
			let position = l.position;
			while l.position < l.input.len() {
				if l.position == l.input.len() {
					break;
				}
				if l.input[l.position+1] == '*' {
					if l.input[l.position+2] == '/' {
						l.read_char();
						l.read_char();
						break;
					}
				}
				l.read_char();
			}
			l.input[position..l.position+1].to_vec()
		};

		let tok: token::Token;
		match self.ch {
			'\n' => {
				tok = token::Token::ENDL(self.ch);
			}
			'0' => {
				if self.position < self.input.len() {
					tok = token::Token::INT(self.input[self.position..self.position+1].to_vec());
				} else {
					tok = token::Token::EOF;
				}
			}
			'/' => {
				if self.input[self.position+1] == '/' {
					tok = token::Token::COMMENT(read_slash_comment(self));
				} else if self.input[self.position+1] == '*' {
					tok = token::Token::COMMENT(read_slash_star_comment(self));
				} else {
					tok = token::Token::CH(self.ch);
				}
			}
			_ => {
				return if is_letter(self.ch) {
					let prev_pos = self.position;
					#[allow(unused_mut)]
					let mut identifier: Vec<char> = read_identifier(self);
					if is_digit(self.ch) {
						let position = self.position;
						while self.position < self.input.len() {
							if self.ch == ' ' || self.ch == ':' ||  self.ch == ',' || self.ch == '{' || self.ch == '\n' {
								break;
							}
							self.read_char();
						}
						identifier.append(&mut self.input[position..self.position].to_vec());
					}
					match token::get_keyword_token(&identifier) {
							Ok(keyword_token) => {
								keyword_token
							},
							Err(_err) => {
								if prev_pos != 0 && self.input[prev_pos-1] == '.' {
									let position = self.position;
									while self.position < self.input.len() {
										if self.ch == ' ' || self.ch == '{' || self.ch == ',' || self.ch == '\n' {
											break;
										}
										self.read_char();
									}
									identifier.append(&mut self.input[position..self.position].to_vec());
									return token::Token::ENTITY(identifier)
								}
								if self.ch == '(' {
									let position = self.position;
									while self.position < self.input.len() {
										if self.ch == ' ' || self.ch == ':' || self.ch == ';' || self.ch == '}' || self.ch == '\n' {
											break;
										}
										self.read_char();
									}
									identifier.append(&mut self.input[position..self.position].to_vec());
									return token::Token::IDENT(identifier)
								}
								if self.ch == '-' || self.ch == ':' {
									let position = self.position;
									while self.position < self.input.len() {
										if self.ch == ' ' || self.ch == ':' || self.ch == '}' || self.ch == '\n' {
											break;
										}
										self.read_char();
									}
									identifier.append(&mut self.input[position..self.position].to_vec());
									return token::Token::ENTITY(identifier)
								}
								if self.ch == '(' {
									return token::Token::ENTITY(identifier)
								}
								token::Token::IDENT(identifier)
							}
						}
					} else if is_digit(self.ch) {
						let identifier: Vec<char> = read_number(self);
						token::Token::INT(identifier)
					} else if self.ch == '"' {
						let str_value: Vec<char> = read_string(self);
						token::Token::STRING(str_value)
					} else {
						token::Token::ILLEGAL
					}
				}
			}
		self.read_char();
		tok
	}
}
