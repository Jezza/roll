use logos::Logos;

#[derive(Logos, Debug, PartialEq, Copy, Clone, Ord, PartialOrd, Eq)]
pub enum TokenKind {
	#[end]
	End,

	// Keywords
	#[token = "d"]
	Dice,

	// Constants
	#[regex = "[0-9]+"]
	Number,

	// Symbols
	#[token = "{"]
	LeftBrace,
	#[token = "}"]
	RightBrace,
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
	Percentage,
	#[token = "^"]
	Caret,
	#[token = ","]
	Comma,

	#[error]
	UnexpectedToken,
}