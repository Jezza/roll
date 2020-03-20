use roll::evaluate_str;

fn main() {
	// @TODO jezza - 13 Jan. 2020: Add custom spans so we know the source range for each expression. 1d6 + 3d(1d9 + 1d10 + 5)
	// @TODO jezza - 13 Jan. 2020: Add support for roll reporting in custom spans. 1d6 + 3d(1d9 = [9] + 1d10 = [2] + 5)
	// @TODO jezza - 13 Jan. 2020: Add custom dice. 1d{5,6}
	// @TODO jezza - 13 Jan. 2020: Add permutations. 1d[5,6]
	// @TODO jezza - 13 Jan. 2020: Add power operator. 2 ^ 2d2
	// @TODO jezza - 13 Jan. 2020: Add modulus operator. 1d6 % 2

//	// roll (2d5 + 2d(1d3)) + 8
//	// roll (2d[1,5,3,6])
//
////    1d(2 + 2)
////    1d2 - 2
////    1d2 * 2
////    1d2 / 2
//
////      1d2 + 1d(2d2)
////      -2d2d2 * 3
////		5 * 2 + 5 + 1d6 + 5 + 5 * 2
////		-6 - 5 + 1d6 - 9 - --1
////		1d6 + 3d(1d9 + 1d10 + 5)
//	let mut expression = parse_expression(r#"
//		(1d6)d6
//    "#);
//
////	flatten_expression(&mut expression);
//
//	let result = evaluate_expression(&expression);
//	println!("Result: {}", result);

	let result = evaluate_str("2d(2 * 2)");
	println!("Result: {}", result);
}
