#[warn(rust_2018_idioms)]

use lang::BinaryOp;
use lang::Expression;
use lang::ExpressionKind;
use lang::UnaryOp;

pub mod lang;

pub fn evaluate_str(input: &str) -> i64 {
	println!("{}", input);
	let expression = lang::parse_expression(input);
	evaluate_expression(input, &expression)
}

fn roll_dice(roll_count: i64, faces: i64) -> Vec<i64> {
	if roll_count == 0 {
		return vec![0];
	}
	if roll_count < 0 {
		panic!("Cannot roll negative number of dice. (left: {}, right: {})", roll_count, faces);
	}
	if faces < 1 {
		panic!("Cannot roll die with zero or negative faces. (left: {}, right: {})", roll_count, faces);
	}

	use rand::distributions::Uniform;
	use rand::distributions::Distribution;

	let between = Uniform::new_inclusive(1, faces);
	let mut rng = rand::thread_rng();

	let mut rolls = Vec::with_capacity(roll_count as usize);

	for _ in 0..roll_count {
		let roll = between.sample(&mut rng);

		rolls.push(roll);
	}

//	let result = rolls.iter().sum();

//	println!("{}d{}: {:?} = {}", roll_count, faces, rolls, result);

//	result
	rolls
}

pub fn evaluate_expression(input: &str, expression: &Expression) -> i64 {
	struct Evaluator<'a>(&'a str);

	impl<'a> lang::Visitor for Evaluator<'a> {
		type Result = i64;

		fn visit_binary(&mut self, expression: &Expression) -> Self::Result {
			if let ExpressionKind::Binary(op, left, right) = &expression.kind {
				let left = left.visit(self);
				let right = right.visit(self);

				let input = &self.0[expression.span.range()];

				let result = match op {
					BinaryOp::Dice => {
						let rolls = roll_dice(left, right);
						let mut result = 0;
						let mut buf = String::new();
						use std::fmt::Write;
						buf.push('[');
						for (i, roll) in rolls.into_iter().enumerate() {
							result += roll;
							write!(&mut buf, "{}", roll);
							if ((i + 1) as i64) != left {
								buf.push_str(" + ");
							}
						}
						buf.push(']');
						println!("{} = {}d{} {} = {}", input, left, right, buf, result);
						return result;
					},
					BinaryOp::Multiply => left * right,
					BinaryOp::Divide => left / right,
					BinaryOp::Minus => left - right,
					BinaryOp::Add => left + right,
				};

				println!("{} = {}", input, result);

				result
			} else {
				unreachable!()
			}
		}

		fn visit_unary(&mut self, expression: &Expression) -> Self::Result {
			if let ExpressionKind::Unary(op, expr) = &expression.kind {
				let result = expr.visit(self);
				println!("{} = {}", &self.0[expression.span.range()], result);

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

