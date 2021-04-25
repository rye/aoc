#[derive(Debug, Clone, Copy)]
pub enum Token {
	Number(i64),
	OpenParen,
	CloseParen,
	Asterisk,
	Plus,
}

#[derive(Debug)]
pub struct Expr(Vec<Token>);

type Intermediate = Vec<Expr>;
type Solution = u64;

pub fn parse(input: &str) -> Intermediate {
	let mut exprs: Vec<Expr> = Vec::new();

	for line in input.lines() {
		let mut number: Option<u32> = None;

		let mut tokens: Vec<Token> = vec![];

		for c in line.chars() {
			if c.is_digit(10) {
				// TODO Not strictly necessary since we only have single digits
				if let Some(mut n) = number {
					n = n * 10 + c.to_digit(10).expect("invalid digit")
				} else {
					number = Some(c.to_digit(10).expect("invalid digit"))
				}
			} else {
				if let Some(n) = number {
					tokens.push(Token::Number(n as i64));
					number = None;
				}

				match c {
					'(' => tokens.push(Token::OpenParen),
					')' => tokens.push(Token::CloseParen),

					' ' => {}
					'+' => tokens.push(Token::Plus),
					'*' => tokens.push(Token::Asterisk),
					_ => unimplemented!(),
				}
			}
		}

		if let Some(n) = number {
			tokens.push(Token::Number(n as i64));
		}

		println!("Line: {} => {:?}", line, tokens);

		exprs.push(Expr(tokens));
	}

	exprs
}

fn check_parenbal(expr: &Expr) -> bool {
	let mut level: usize = 0;
	let mut balanced: bool = true;

	for token in expr.0.iter() {
		match token {
			Token::OpenParen => {
				level += 1;
			}
			Token::CloseParen => {
				if level > 0 {
					level -= 1;
				} else {
					balanced = false;
					break;
				}
			}
			_ => {}
		}
	}

	if level > 0 {
		balanced = false;
	}

	balanced
}

pub fn eval_expr(expr: &Expr) -> Option<i64> {
	if !check_parenbal(expr) {
		return None;
	}

	let mut result: Option<i64> = None;
	let mut current_op: Option<Token> = None;

	println!("Evaluating {:?}", expr);

	for (idx, token) in expr.0.iter().enumerate() {
		println!(
			"idx={}, token={:?}, result={:?}, current_op={:?}",
			idx, token, result, current_op
		);

		match token {
			Token::Number(n) => {
				if result == None {
					result = Some(*n);
				} else if let Some(Token::Plus) = current_op {
					result = Some(result.unwrap() + n);
					current_op = None;
				} else if let Some(Token::Asterisk) = current_op {
					result = Some(result.unwrap() * n);
					current_op = None;
				} else {
					panic!()
				}
			}
			Token::Plus => {
				if current_op.is_none() {
					current_op = Some(Token::Plus)
				} else {
					panic!()
				}
			}
			Token::Asterisk => {
				if current_op.is_none() {
					current_op = Some(Token::Asterisk)
				} else {
					panic!()
				}
			}
			Token::OpenParen => {
				let mut matching_close_idx: usize = idx;

				let mut level: usize = 0;

				for close_idx in idx..expr.0.len() {
					match expr.0[close_idx] {
						Token::OpenParen => level += 1,
						Token::CloseParen => {
							if level > 0 {
								level -= 1
							} else {
								matching_close_idx = close_idx;
								break;
							}
						}
						_ => {}
					}
				}

				let expr = Expr(expr.0[idx..matching_close_idx].iter().cloned().collect());

				let result = eval_expr(&expr);

				match current_op { _ => todo!() }
			}
			Token::CloseParen => {},
			_ => todo!(),
		}
	}

	result
}

#[test]
fn parenbal_o() {
	assert_eq!(check_parenbal(&Expr(vec![Token::OpenParen,])), false);
}

#[test]
fn parenbal_c() {
	assert_eq!(check_parenbal(&Expr(vec![Token::CloseParen,])), false);
}

#[test]
fn parenbal_oc() {
	assert_eq!(
		check_parenbal(&Expr(vec![Token::OpenParen, Token::CloseParen,])),
		true
	);
}

#[test]
fn parenbal_co() {
	assert_eq!(
		check_parenbal(&Expr(vec![Token::CloseParen, Token::OpenParen,])),
		false
	);
}

#[test]
fn parenbal_oocc() {
	assert_eq!(
		check_parenbal(&Expr(vec![
			Token::OpenParen,
			Token::OpenParen,
			Token::CloseParen,
			Token::CloseParen,
		])),
		true
	);
}

#[test]
fn parenbal_ococ() {
	assert_eq!(
		check_parenbal(&Expr(vec![
			Token::OpenParen,
			Token::CloseParen,
			Token::OpenParen,
			Token::CloseParen,
		])),
		true
	);
}

#[test]
fn eval_expr_71() {
	use Token::*;

	assert_eq!(
		eval_expr(&Expr(vec![
			Number(1),
			Plus,
			Number(2),
			Asterisk,
			Number(3),
			Plus,
			Number(4),
			Asterisk,
			Number(5),
			Plus,
			Number(6),
		])),
		Some(71)
	)
}

#[test]
fn eval_expr_51() {
	use Token::*;

	assert_eq!(
		eval_expr(&Expr(vec![
			Number(1),
			Plus,
			OpenParen,
			Number(2),
			Asterisk,
			Number(3),
			CloseParen,
			Plus,
			OpenParen,
			Number(4),
			Asterisk,
			OpenParen,
			Number(5),
			Plus,
			Number(6),
			CloseParen,
			CloseParen,
		])),
		Some(51)
	)
}

#[test]
fn eval_expr_26() {
	use Token::*;
	assert_eq!(
		eval_expr(&Expr(vec![
			Number(2),
			Asterisk,
			Number(3),
			Plus,
			OpenParen,
			Number(4),
			Asterisk,
			Number(5),
			CloseParen
		])),
		Some(26)
	)
}

#[test]
fn eval_expr_437() {
	use Token::*;
	assert_eq!(
		eval_expr(&Expr(vec![
			Number(5),
			Plus,
			OpenParen,
			Number(8),
			Asterisk,
			Number(3),
			CloseParen
		])),
		Some(437)
	)
}

pub fn part_one(exprs: &Intermediate) -> Option<Solution> {
	None
}

pub fn part_two(exprs: &Intermediate) -> Option<Solution> {
	None
}
