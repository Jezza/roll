use std::fmt::{Debug, Formatter, Result as FResult, Write};
pub use std::ops::Range;

pub use logos::Lexer;
pub use logos::Logos;

pub use crate::lang::lexer::TokenKind;
pub use crate::lang::parser::Parser;

mod parser;
mod lexer;

pub fn parse_expression(input: &str) -> Expression {
	return Parser::new(input)
		.expr();
}

const ADD: (u32, u32, BinaryOp) = (1, 1, BinaryOp::Plus);
const MINUS: (u32, u32, BinaryOp) = (1, 1, BinaryOp::Minus);
const MULTIPLY: (u32, u32, BinaryOp) = (2, 2, BinaryOp::Multiply);
const DIVIDE: (u32, u32, BinaryOp) = (2, 2, BinaryOp::Divide);
const DICE: (u32, u32, BinaryOp) = (4, 3, BinaryOp::Dice);

const UNARY_PRIORITY: u32 = 3;

impl<'source> Parser<&'source str> {
	fn expr(&mut self) -> Expression {
		self.expr_with_precedence(0)
	}

	fn expr_with_precedence(&mut self, precedence: u32) -> Expression {
		let mut left = self.prefix();

		loop {
			let op = self.peek_kind();

			let (limit, new_precedence, op) = match op {
				TokenKind::Plus => ADD,
				TokenKind::Dash => MINUS,
				TokenKind::Asterisk => MULTIPLY,
				TokenKind::ForwardSlash => DIVIDE,
				TokenKind::Dice => DICE,
				_ => break,
			};

			if limit <= precedence {
				break;
			}

			self.take_kind();

			let mut right = self.expr_with_precedence(new_precedence);

			left = Expression::Binary(op, Box::new(left), Box::new(right));
		}

		left
	}

	fn prefix(&mut self) -> Expression {
		let kind = self.peek_kind();

		match kind {
			TokenKind::Dash => {
				self.take_kind();

				let expression = Box::new(self.expr_with_precedence(UNARY_PRIORITY));

				Expression::Unary(UnaryOp::Negative, expression)
			}
			TokenKind::Number => {
				let number = self.slice()
					.parse()
					.unwrap();

				self.take_kind();

				Expression::Constant(number)
			}
			TokenKind::LeftParenthesis => {
				self.take_kind();

				let expression = self.expr();

				self.expect(TokenKind::RightParenthesis).unwrap();
				expression
			}
			_ => {
				let (kind, range) = self.peek_token();
				panic!("Unexpected token \"{:?}\" @ {:?}", kind, range);
			}
		}
	}
}

pub enum Expression {
	Binary(BinaryOp, Box<Expression>, Box<Expression>),
	Unary(UnaryOp, Box<Expression>),
	Constant(i64),
}

impl Expression {
	pub fn is_number(&self) -> bool {
		match self {
			Expression::Constant(..) => true,
			_ => false,
		}
	}
}

impl Debug for Expression {
	fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
		match self {
			Expression::Binary(BinaryOp::Dice, left, right) => write!(f, "({:?}d{:?})", left, right),
			Expression::Binary(op, left, right) => write!(f, "({:?} {:?} {:?})", left, op, right),
			Expression::Unary(op, expr) => write!(f, "({:?}{:?})", op, expr),
			Expression::Constant(value) => write!(f, "{}", value),
		}
	}
}

pub enum BinaryOp {
	Dice,
	Plus,
	Minus,
	Multiply,
	Divide,
}

impl Debug for BinaryOp {
	fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
		match self {
			BinaryOp::Dice => f.write_char('d'),
			BinaryOp::Plus => f.write_char('+'),
			BinaryOp::Minus => f.write_char('-'),
			BinaryOp::Multiply => f.write_char('*'),
			BinaryOp::Divide => f.write_char('/'),
		}
	}
}

pub enum UnaryOp {
	Negative,
}

impl Debug for UnaryOp {
	fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
		match self {
			UnaryOp::Negative => f.write_char('-'),
		}
	}
}


pub trait Visitor {
	type Result;

	fn visit_pre(&mut self, expression: &Expression) {}
	fn visit_post(&mut self, expression: &Expression) {}

	fn visit_binary(&mut self, expression: &Expression) -> Self::Result;

	fn visit_unary(&mut self, expression: &Expression) -> Self::Result;

	fn visit_constant(&mut self, expression: &Expression) -> Self::Result;
}

pub trait VisitorMut {
	type Result;

	fn visit_pre(&mut self, expression: &mut Expression) {}
	fn visit_post(&mut self, expression: &mut Expression) {}

	fn visit_binary(&mut self, expression: &mut Expression) -> Self::Result;

	fn visit_unary(&mut self, expression: &mut Expression) -> Self::Result;

	fn visit_constant(&mut self, expression: &mut Expression) -> Self::Result;
}

impl Expression {
	pub fn visit<V>(&self, visitor: &mut V) -> V::Result where V: Visitor {
		visitor.visit_pre(self);

		let result = match self {
			Expression::Binary(..) => visitor.visit_binary(self),
			Expression::Unary(..) => visitor.visit_unary(self),
			Expression::Constant(..) => visitor.visit_constant(self),
		};

		visitor.visit_post(self);

		result
	}

	pub fn visit_mut<V>(&mut self, visitor: &mut V) -> V::Result where V: VisitorMut {
		visitor.visit_pre(self);

		let result = match self {
			Expression::Binary(..) => visitor.visit_binary(self),
			Expression::Unary(..) => visitor.visit_unary(self),
			Expression::Constant(..) => visitor.visit_constant(self),
		};

		visitor.visit_post(self);

		result
	}
}
