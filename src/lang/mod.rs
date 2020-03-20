use std::fmt::{Debug, Formatter, Result as FResult, Write};
pub use std::ops::Range;

pub use logos::Lexer;
pub use logos::Logos;

pub use crate::lang::lexer::TokenKind;
pub use crate::lang::parser::Parser;
pub use crate::lang::parser::Span;

mod parser;
mod lexer;

pub fn parse_expression(input: &str) -> Expression {
	return Parser::new(input)
		.expr();
}

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
				TokenKind::Plus => (1, 1, BinaryOp::Add),
				TokenKind::Dash => (1, 1, BinaryOp::Minus),
				TokenKind::Asterisk => (2, 2, BinaryOp::Multiply),
				TokenKind::ForwardSlash => (2, 2, BinaryOp::Divide),
				TokenKind::Dice => (4, 3, BinaryOp::Dice),
				_ => break,
			};

			if limit <= precedence {
				break;
			}

			self.take_kind();

			let right = self.expr_with_precedence(new_precedence);

			let span = left.span.merge(right.span);

			left = Expression::new_binary(span, op, Box::new(left), Box::new(right));
		}

		left
	}

	fn prefix(&mut self) -> Expression {
		let kind = self.peek_kind();

		match kind {
			TokenKind::Dash => {
				self.take_kind();

				let expression = Box::new(self.expr_with_precedence(UNARY_PRIORITY));

				Expression::new_unary(expression.span, UnaryOp::Negative, expression)
			}
			TokenKind::Number => {
				let (_, span, slice) = self.take_token();
				let number = slice
					.parse()
					.unwrap();

				Expression::new_constant(span, number)
			}
			TokenKind::LeftParenthesis => {
				let (_, start_span, _) = self.take_token();

				let expression = Box::new(self.expr());

				let (_, end_span, _) = self.expect(TokenKind::RightParenthesis).unwrap();
				Expression::new_tree(start_span.merge(end_span), expression)
			}
			_ => {
				let (kind, range, _) = self.peek_token();
				panic!("Unexpected token \"{:?}\" @ {:?}", kind, range);
			}
		}
	}
}

#[derive(Debug)]
pub struct Expression {
	pub span: Span,
	pub kind: ExpressionKind,
}

pub enum ExpressionKind {
	Binary(BinaryOp, Box<Expression>, Box<Expression>),
	Unary(UnaryOp, Box<Expression>),
	Tree(Box<Expression>),
	Constant(i64),
}

impl Expression {
	pub fn new_binary(span: Span, binary_op: BinaryOp, left: Box<Expression>, right: Box<Expression>) -> Self {
		let kind = ExpressionKind::Binary(binary_op, left, right);
		Expression {
			span,
			kind,
		}
	}

	pub fn new_unary(span: Span, unary_op: UnaryOp, expr: Box<Expression>) -> Self {
		let kind = ExpressionKind::Unary(unary_op, expr);
		Expression {
			span,
			kind,
		}
	}

	pub fn new_constant(span: Span, constant: i64) -> Self {
		let kind = ExpressionKind::Constant(constant);
		Expression {
			span,
			kind,
		}
	}

	pub fn new_tree(span: Span, expr: Box<Expression>) -> Self {
		let kind = ExpressionKind::Tree(expr);
		Expression {
			span,
			kind,
		}
	}

	pub fn is_number(&self) -> bool {
		match &self.kind {
			ExpressionKind::Constant(..) => true,
			_ => false,
		}
	}
}

impl Debug for ExpressionKind {
	fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
		match &self {
			ExpressionKind::Binary(BinaryOp::Dice, left, right) => write!(f, "({:?}d{:?})", left, right),
			ExpressionKind::Binary(op, left, right) => write!(f, "({:?} {:?} {:?})", left, op, right),
			ExpressionKind::Unary(op, expr) => write!(f, "({:?}{:?})", op, expr),
			ExpressionKind::Tree(expr) => write!(f, "({:?})", expr),
			ExpressionKind::Constant(value) => write!(f, "{}", value),
		}
	}
}

pub enum BinaryOp {
	Dice,
	Add,
	Minus,
	Multiply,
	Divide,
}

impl Debug for BinaryOp {
	fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
		match self {
			BinaryOp::Dice => f.write_char('d'),
			BinaryOp::Add => f.write_char('+'),
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

	fn visit_tree(&mut self, expression: &Expression) -> Self::Result;

	fn visit_constant(&mut self, expression: &Expression) -> Self::Result;
}

pub trait VisitorMut {
	type Result;

	fn visit_pre(&mut self, expression: &mut Expression) {}
	fn visit_post(&mut self, expression: &mut Expression) {}

	fn visit_binary(&mut self, expression: &mut Expression) -> Self::Result;

	fn visit_unary(&mut self, expression: &mut Expression) -> Self::Result;

	fn visit_tree(&mut self, expression: &mut Expression) -> Self::Result;

	fn visit_constant(&mut self, expression: &mut Expression) -> Self::Result;
}

impl Expression {
	pub fn visit<V>(&self, visitor: &mut V) -> V::Result where V: Visitor {
		visitor.visit_pre(self);

		let result = match &self.kind {
			ExpressionKind::Binary(..) => visitor.visit_binary(self),
			ExpressionKind::Unary(..) => visitor.visit_unary(self),
			ExpressionKind::Tree(..) => visitor.visit_tree(self),
			ExpressionKind::Constant(..) => visitor.visit_constant(self),
		};

		visitor.visit_post(self);

		result
	}

	pub fn visit_mut<V>(&mut self, visitor: &mut V) -> V::Result where V: VisitorMut {
		visitor.visit_pre(self);

		let result = match &self.kind {
			ExpressionKind::Binary(..) => visitor.visit_binary(self),
			ExpressionKind::Unary(..) => visitor.visit_unary(self),
			ExpressionKind::Tree(..) => visitor.visit_tree(self),
			ExpressionKind::Constant(..) => visitor.visit_constant(self),
		};

		visitor.visit_post(self);

		result
	}
}
