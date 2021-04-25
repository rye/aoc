#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
	Number(u64),
	OpenParen,
	CloseParen,
	Asterisk,
	Plus,
}

impl std::fmt::Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Token::Number(n) => write!(f, "{}", n),
			Token::Plus => write!(f, "+"),
			Token::Asterisk => write!(f, "*"),
			Token::OpenParen => write!(f, "("),
			Token::CloseParen => write!(f, ")"),
		}
	}
}

#[derive(Debug)]
pub struct Expr(Vec<Token>);

impl std::fmt::Display for Expr {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let disp = self
			.0
			.iter()
			.map(|tok| tok.to_string())
			.collect::<Vec<String>>()
			.join(" ");

		write!(f, "{}", disp)
	}
}

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
				if let Some(n) = number {
					number = Some(n * 10 + c.to_digit(10).expect("invalid digit"))
				} else {
					number = Some(c.to_digit(10).expect("invalid digit"))
				}
			} else {
				if let Some(n) = number {
					tokens.push(Token::Number(n as u64));
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
			tokens.push(Token::Number(n as u64));
		}

		exprs.push(Expr(tokens));
	}

	exprs
}

impl Expr {
	fn is_paren_balanced(&self) -> bool {
		let mut level: usize = 0;
		let mut balanced: bool = true;

		for token in self.0.iter() {
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

	fn is_valid(&self) -> bool {
		self.is_paren_balanced() && true
	}

	fn matching_close_idx(span: &[Token], open_idx: usize) -> Option<usize> {
		assert!(span[open_idx] == Token::OpenParen);

		let mut level = 0;

		// TODO enumerate/scan/find?

		for idx in open_idx..span.len() {
			match span[idx] {
				Token::OpenParen => level += 1,
				Token::CloseParen => {
					level -= 1;
					if level == 0 {
						return Some(idx);
					}
				}
				_ => {}
			}
		}

		None
	}

	/// Evaluates a `(` `)`-encapsulated span consisting of solely +, *, and literlas.
	fn eval_span(span: &[Token]) -> Option<u64> {
		let mut value: Option<u64> = None;
		let mut op: Option<Token> = None;

		for tok in span {
			match (value, op, tok) {
				(None, _, Token::Number(n)) => value = Some(*n),
				(None, _, _) => panic!("need a starting value first"),
				(Some(_), None, Token::Plus) => op = Some(Token::Plus),
				(Some(_), None, Token::Asterisk) => op = Some(Token::Asterisk),

				(Some(cur), Some(Token::Plus), Token::Number(n)) => {
					value = Some(cur + n);
					op = None;
				}
				(Some(cur), Some(Token::Asterisk), Token::Number(n)) => {
					value = Some(cur * n);
					op = None;
				}
				(Some(_), _, _) => panic!(
					"unexpected span layout, cur={:?}, op={:?}, tok={:?}",
					value, op, tok
				),
			}
		}

		value
	}

	fn evaluate(&self) -> Option<u64> {
		if !self.is_valid() {
			return None;
		}

		// Flatten out the expression, evaluating subexpressions as they are found.
		if let Some(open_paren_idx) = self
			.0
			.iter()
			.enumerate()
			.find(|(_, tok)| **tok == Token::OpenParen)
			.map(|res| res.0)
		{
			let matching_close_paren_idx = Self::matching_close_idx(&self.0, open_paren_idx).unwrap();

			let span_to_evaluate = (open_paren_idx + 1)..matching_close_paren_idx;

			let subexpr = Expr(self.0[span_to_evaluate.clone()].to_vec());

			let result = subexpr
				.evaluate()
				.expect("inner expr failed to evaluate to a value");

			let mut new_expr_body = self.0.clone();
			let _ = new_expr_body.splice(
				open_paren_idx..=matching_close_paren_idx,
				vec![Token::Number(result)],
			);

			let new_expr = Expr(new_expr_body);

			new_expr.evaluate()
		} else {
			Self::eval_span(&self.0)
		}
	}
}

#[cfg(test)]
mod expr {

	use super::{Expr, Token, Token::*};

	#[cfg(test)]
	mod is_paren_balanced {

		use super::*;

		#[test]
		fn o() {
			assert_eq!(Expr(vec![OpenParen,]).is_paren_balanced(), false);
		}

		#[test]
		fn c() {
			assert_eq!(Expr(vec![CloseParen,]).is_paren_balanced(), false);
		}

		#[test]
		fn oc() {
			assert_eq!(Expr(vec![OpenParen, CloseParen,]).is_paren_balanced(), true);
		}

		#[test]
		fn co() {
			assert_eq!(
				Expr(vec![CloseParen, OpenParen,]).is_paren_balanced(),
				false
			);
		}

		#[test]
		fn oocc() {
			assert_eq!(
				Expr(vec![OpenParen, OpenParen, CloseParen, CloseParen,]).is_paren_balanced(),
				true
			);
		}

		#[test]
		fn ococ() {
			assert_eq!(
				Expr(vec![OpenParen, CloseParen, OpenParen, CloseParen,]).is_paren_balanced(),
				true
			);
		}
	}

	#[cfg(test)]
	mod eval_span {
		use super::*;

		#[test]
		fn s1p2m3p4m5p6() {
			assert_eq!(
				Expr::eval_span(&vec![
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
				]),
				Some(71)
			);
		}
	}

	#[cfg(test)]
	mod evaluate {
		use super::*;

		#[test]
		fn e71() {
			assert_eq!(
				Expr(vec![
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
				])
				.evaluate(),
				Some(71)
			)
		}

		#[test]
		fn e51() {
			assert_eq!(
				Expr(vec![
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
				])
				.evaluate(),
				Some(51)
			)
		}

		#[test]
		fn e26() {
			assert_eq!(
				Expr(vec![
					Number(2),
					Asterisk,
					Number(3),
					Plus,
					OpenParen,
					Number(4),
					Asterisk,
					Number(5),
					CloseParen
				])
				.evaluate(),
				Some(26)
			)
		}

		#[test]
		fn e437() {
			assert_eq!(
				Expr(vec![
					Number(5),
					Plus,
					OpenParen,
					Number(8),
					Asterisk,
					Number(3),
					Plus,
					Number(9),
					Plus,
					Number(3),
					Asterisk,
					Number(4),
					Asterisk,
					Number(3),
					CloseParen
				])
				.evaluate(),
				Some(437)
			)
		}

		#[test]
		fn e12240() {
			assert_eq!(
				Expr(vec![
					Number(5),
					Token::Asterisk,
					Number(9),
					Token::Asterisk,
					Token::OpenParen,
					Number(7),
					Token::Asterisk,
					Number(3),
					Token::Asterisk,
					Number(3),
					Token::Plus,
					Number(9),
					Token::Asterisk,
					Number(3),
					Token::Plus,
					Token::OpenParen,
					Number(8),
					Token::Plus,
					Number(6),
					Token::Asterisk,
					Number(4),
					CloseParen,
					CloseParen
				])
				.evaluate(),
				Some(12240)
			)
		}

		#[test]
		fn e13632() {
			assert_eq!(
				Expr(vec![
					OpenParen,
					OpenParen,
					Number(2),
					Plus,
					Number(4),
					Asterisk,
					Number(9),
					CloseParen,
					Asterisk,
					OpenParen,
					Number(6),
					Plus,
					Number(9),
					Asterisk,
					Number(8),
					Plus,
					Number(6),
					CloseParen,
					Plus,
					Number(6),
					CloseParen,
					Plus,
					Number(2),
					Plus,
					Number(4),
					Asterisk,
					Number(2)
				])
				.evaluate(),
				Some(13632)
			)
		}
	}

	#[cfg(test)]
	mod matching_close_idx {
		use super::*;

		#[test]
		fn onpnc() {
			// ( 1 + 2 )
			let span = vec![OpenParen, Number(1), Plus, Number(2), CloseParen];

			assert_eq!(Expr::matching_close_idx(&span, 0), Some(4));
		}

		#[test]
		fn onpnponmncc() {
			// ( 1 + 2 + ( 3 * 4 ) )
			let span = vec![
				OpenParen,
				Number(1),
				Plus,
				Number(2),
				Plus,
				OpenParen,
				Number(3),
				Asterisk,
				Number(4),
				CloseParen,
				CloseParen,
			];

			assert_eq!(Expr::matching_close_idx(&span, 0), Some(10));
			assert_eq!(Expr::matching_close_idx(&span, 5), Some(9));
		}

		#[test]
		fn onpnponmncponpncc() {
			// ( 1 + 2 + ( 3 * 4 ) + ( 5 + 6 ) )
			let span = vec![
				OpenParen,
				Number(1),
				Plus,
				Number(2),
				Plus,
				OpenParen,
				Number(3),
				Asterisk,
				Number(4),
				CloseParen,
				Plus,
				OpenParen,
				Number(5),
				Plus,
				Number(6),
				CloseParen,
				CloseParen,
			];

			assert_eq!(Expr::matching_close_idx(&span, 0), Some(16));
			assert_eq!(Expr::matching_close_idx(&span, 5), Some(9));
			assert_eq!(Expr::matching_close_idx(&span, 11), Some(15));
		}
	}
}

pub fn part_one(exprs: &Intermediate) -> Option<Solution> {
	exprs.iter().map(|expr| expr.evaluate()).sum()
}

pub fn part_two(exprs: &Intermediate) -> Option<Solution> {
	None
}
