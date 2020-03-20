use crate::lang::*;

#[derive(Debug, Copy, Clone)]
pub struct Span {
	start: usize,
	end: usize,
}

impl Span {
	fn new(range: Range<usize>) -> Self {
		let Range {
			start,
			end,
		} = range;
		Span {
			start,
			end,
		}
	}

	pub fn merge(&self, other: Span) -> Self {
		let start = self.start.min(other.start);
		let end = self.end.max(other.end);
		Span {
			start,
			end,
		}
	}

	pub fn range(&self) -> Range<usize> {
		self.start..self.end
	}
}

pub type Token<'source> = (TokenKind, Span, &'source str);

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

	pub fn peek_token(&mut self) -> Token<'source> {
		let current = self.lexer.token;
		let range = Span::new(self.lexer.range());
		let slice = self.lexer.slice();
		(current, range, slice)
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

	pub fn take_token(&mut self) -> Token<'source> {
		let token = self.peek_token();
		self.lexer.advance();
		token
	}

	pub fn expect(&mut self, token: TokenKind) -> Result<Token<'source>, Token<'source>> {
		let result = self.peek_token();
		if result.0 == token {
			self.lexer.advance();
			Ok(result)
		} else {
			Err(result)
		}
	}
}