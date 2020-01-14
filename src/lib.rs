use lang::BinaryOp;
use lang::Expression;
use lang::parse_expression;
use lang::UnaryOp;

use crate::lang::VisitorMut;

pub mod lang;

pub fn evaluate_str(input: &str) -> i64 {
	let expression = lang::parse_expression(input);
	evaluate_expression(&expression)
}

pub fn evaluate_expression(expression: &Expression) -> i64 {
	use rand::distributions::Uniform;
	use rand::distributions::Distribution;

	struct Evaluator;
	impl lang::Visitor for Evaluator {
		type Result = i64;

		fn visit_binary(&mut self, expression: &Expression) -> Self::Result {
			if let Expression::Binary(op, left, right) = expression {
				let left = left.visit(self);
				let right = right.visit(self);

				if left == 0 {
					return 0;
				}
				if left < 0 {
					panic!("Cannot roll negative number of dice. (left: {}, right: {})", left, right);
				}
				if right < 1 {
					panic!("Cannot roll die with zero or negative faces. (left: {}, right: {})", left, right);
				}

				match op {
					BinaryOp::Dice => {
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
			if let Expression::Unary(op, expr) = expression {
				let result = expr.visit(self);

				match op {
					UnaryOp::Negative => -result
				}
			} else {
				unreachable!()
			}
		}

		fn visit_constant(&mut self, expression: &Expression) -> Self::Result {
			if let Expression::Constant(value) = expression {
				*value
			} else {
				unreachable!()
			}
		}
	}

	expression.visit(&mut Evaluator)
}

pub fn fold_expression(expression: &mut Expression) {
	fn extract_number(expression: &Expression) -> Option<i64> {
		match expression {
			Expression::Constant(value) => Some(*value),
			Expression::Unary(UnaryOp::Negative, expr) => {
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
			if let Expression::Binary(op, left, right) = expression {
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

//							((9 - (11 + (1d6))) - 1)
						*expression = Expression::Constant(result);
					}
				}
			} else {
				unreachable!()
			}
		}

		fn visit_unary(&mut self, expression: &mut Expression) {
			if let Expression::Unary(UnaryOp::Negative, expr) = expression {
				expr.visit_mut(self);

				if let Expression::Unary(UnaryOp::Negative, expr) = expr.as_mut() {
					let expr = expr.as_mut();
					let new = std::mem::replace(expr, Expression::Constant(0));
					*expression = new;
				}
			} else {
				unreachable!()
			}
		}

		fn visit_constant(&mut self, expression: &mut Expression) {
		}
	}

	expression.visit_mut(&mut ConstantFolding)
}

