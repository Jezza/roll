use crate::lang::*;

pub type Token = (TokenKind, Range<usize>);

pub struct Parser<Source> {
	lexer: Lexer<TokenKind, Source>,
}

impl<'source> Parser<&'source str> {

	pub fn new(source: &str) -> Parser<&str> {
		let lexer = TokenKind::lexer(source);
		Parser {
			lexer,
		}
	}

	pub fn slice(&self) -> &'source str {
		self.lexer.slice()
	}

	pub fn peek_token(&mut self) -> Token {
		let current = self.lexer.token;
		let range = self.lexer.range();
		(current, range)
	}

	pub fn peek_kind(&mut self) -> TokenKind {
		self.lexer.token
	}

	pub fn is(&self, token: TokenKind) -> bool {
		self.lexer.token == token
	}

	pub fn not(&self, token: TokenKind) -> bool {
		self.lexer.token != token
	}

	pub fn when(&mut self, token: TokenKind) -> bool {
		let expected = self.lexer.token == token;
		if expected {
			self.lexer.advance();
		}
		expected
	}

	pub fn take_kind(&mut self) -> TokenKind {
		let token = self.lexer.token;
		self.lexer.advance();
		token
	}

	pub fn take_token(&mut self) -> Token {
		let token = self.peek_token();
		self.lexer.advance();
		token
	}

	pub fn expect(&mut self, token: TokenKind) -> Result<Token, Token> {
		let result = self.peek_token();
		if result.0 == token {
			self.lexer.advance();
			Ok(result)
		} else {
			Err(result)
		}
	}
}