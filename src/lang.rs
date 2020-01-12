

pub mod lexer {
	pub use logos::{Lexer, Logos, Slice, Source};

	pub fn from_source<'source, S: Source<'source>>(source: S) -> Lexer<TokenKind, S> {
		TokenKind::lexer(source)
	}

	#[derive(Logos, Debug, PartialEq, Copy, Clone, Ord, PartialOrd, Eq)]
	pub enum TokenKind {
		#[end]
		End,

		// Keywords
		#[token = "d"]
		Dice,

		// Literals
		#[regex = "[0-9]+"]
		Number,  // _

		// Symbols
		#[token = "["]
		LeftBracket,
		#[token = "]"]
		RightBracket,
		#[token = "("]
		LeftParenthesis,
		#[token = ")"]
		RightParenthesis,
		#[token = "-"]
		Dash,
		#[token = "+"]
		Plus,
		#[token = "*"]
		Asterisk,
		#[token = "/"]
		ForwardSlash,
		#[token = "%"]
		Percent,
		#[token = "^"]
		Caret,

//		#[regex = "//[^\n]*"]
		#[error]
		UnexpectedToken,
	}
}

pub mod parser {
	use std::ops::Range;

	use crate::lang::lexer::*;

	pub type Token = (TokenKind, Range<usize>);

	pub struct Parser<Source> {
		lexer: Lexer<TokenKind, Source>,
	}

	impl<'source> Parser<&'source str> {

		pub fn new(source: &str) -> Parser<&str> {
			let lexer = crate::lang::lexer::from_source(source);
			Parser {
				lexer,
			}
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
}
