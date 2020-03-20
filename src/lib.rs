#[warn(rust_2018_idioms)]

use lang::BinaryOp;
use lang::Expression;
use lang::ExpressionKind;
use lang::UnaryOp;

pub mod lang;

pub fn evaluate_str(input: &str) -> i64 {
	let expression = lang::parse_expression(input);
	evaluate_expression(input, &expression)
}

pub fn evaluate_expression(input: &str, expression: &Expression) -> i64 {
	use rand::distributions::Uniform;
	use rand::distributions::Distribution;

	struct Evaluator<'a>(&'a str);

	impl<'a> lang::Visitor for Evaluator<'a> {
		type Result = i64;

		fn visit_binary(&mut self, expression: &Expression) -> Self::Result {
			if let ExpressionKind::Binary(op, left, right) = &expression.kind {
				let left = left.visit(self);
				let right = right.visit(self);

				match op {
					BinaryOp::Dice => {
						if left == 0 {
							return 0;
						}
						if left < 0 {
							panic!("Cannot roll negative number of dice. (left: {}, right: {})", left, right);
						}
						if right < 1 {
							panic!("Cannot roll die with zero or negative faces. (left: {}, right: {})", left, right);
						}

						let between = Uniform::new_inclusive(1, right);
						let mut rng = rand::thread_rng();

						let mut rolls = Vec::with_capacity(left as usize);

						for _ in 0..left {
							let roll = between.sample(&mut rng);

							rolls.push(roll);
						}

						let result = rolls.iter().sum();

						println!("{}d{}: {:?} = {}", left, right, rolls, result);

						result
					}
					BinaryOp::Multiply => left * right,
					BinaryOp::Divide => left / right,
					BinaryOp::Minus => left - right,
					BinaryOp::Add => left + right,
				}
			} else {
				unreachable!()
			}
		}

		fn visit_unary(&mut self, expression: &Expression) -> Self::Result {
			if let ExpressionKind::Unary(op, expr) = &expression.kind {
				let result = expr.visit(self);

				match op {
					UnaryOp::Negative => -result
				}
			} else {
				unreachable!()
			}
		}

		fn visit_tree(&mut self, expression: &Expression) -> Self::Result {
			if let ExpressionKind::Tree(expr) = &expression.kind {
				expr.visit(self)
			} else {
				unreachable!()
			}
		}

		fn visit_constant(&mut self, expression: &Expression) -> Self::Result {
			if let ExpressionKind::Constant(value) = &expression.kind {
				*value
			} else {
				unreachable!()
			}
		}
	}

	expression.visit(&mut Evaluator(input))
}

pub fn fold_expression(expression: &mut Expression) {
	fn extract_number(expression: &Expression) -> Option<i64> {
		match &expression.kind {
			ExpressionKind::Constant(value) => Some(*value),
			ExpressionKind::Unary(UnaryOp::Negative, expr) => {
				if let Some(value) = extract_number(expr) {
					Some(-value)
				} else {
					None
				}
			}
			_ => None,
		}
	}

	struct ConstantFolding;
	impl lang::VisitorMut for ConstantFolding {
		type Result = ();

		fn visit_binary(&mut self, expression: &mut Expression) {
			if let ExpressionKind::Binary(op, left, right) = &mut expression.kind {
				left.visit_mut(self);
				right.visit_mut(self);

				match op {
					BinaryOp::Add => (),
					BinaryOp::Minus => (),
					BinaryOp::Multiply => (),
					BinaryOp::Divide => (),
					_ => return,
				}

				if let Some(left) = extract_number(left) {
					if let Some(right) = extract_number(right) {
						let result = match op {
							BinaryOp::Add => left + right,
							BinaryOp::Minus => left - right,
							BinaryOp::Multiply => left * right,
							BinaryOp::Divide => left / right,
							_ => unreachable!(),
						};

						expression.kind = ExpressionKind::Constant(result);
					}
				}
			} else {
				unreachable!()
			}
		}

		fn visit_unary(&mut self, expression: &mut Expression) {
			if let ExpressionKind::Unary(UnaryOp::Negative, expr) = &mut expression.kind {
				expr.visit_mut(self);

				if let ExpressionKind::Unary(UnaryOp::Negative, expr) = &mut expr.as_mut().kind {

					let span = expression.span.merge(expr.span);

					let kind = std::mem::replace(&mut expr.as_mut().kind, ExpressionKind::Constant(0));
					expression.kind = kind;
					expression.span = span;
				}
			} else {
				unreachable!()
			}
		}

		fn visit_tree(&mut self, expression: &mut Expression) {
			if let ExpressionKind::Tree(expr) = &mut expression.kind {
				expr.visit_mut(self)
			} else {
				unreachable!()
			}
		}

		fn visit_constant(&mut self, expression: &mut Expression) {
		}
	}

	expression.visit_mut(&mut ConstantFolding)
}

