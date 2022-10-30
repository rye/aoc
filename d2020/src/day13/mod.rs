use num_integer::Integer;

/// Compute the greatest common divisor of `a` and `b` using the Extended Euclidean Algorithm.
pub fn extended_gcd<T: Copy + Integer>(a: T, b: T) -> (T, T, T) {
	if a == T::zero() {
		(b, T::zero(), T::one())
	} else {
		let (gcd, x, y) = extended_gcd(b % a, a);
		(gcd, y - (b / a) * x, x)
	}
}

/// Compute the multiplicative inverse of `n` in Z_`modulus`, if one exists.
pub fn modular_multiplicative_inverse<T: Copy + Integer>(n: T, modulus: T) -> Option<T> {
	let (g, x, _) = extended_gcd(n, modulus);

	if g != T::one() {
		None
	} else {
		Some((x % modulus + modulus) % modulus)
	}
}

/// Compute the unique solution to a system of congruences using the Chinese Remainder Theorem.
///
/// If the `.1` subscripts of the given slice are pairwise coprime, and the `.0` subscripts of the
/// given slice are integers between 0 and their corresponding .1 subscript, then the Chinese
/// Remainder Theorem asserts that there is one and only one integer `x` between 0 and the product
/// of all of the `.1` subscripts for which the remainder of the Euclidean division of `x` and each
/// of the `.1` subscripts is the same as the value in the corresponding `.0` subscript.
///
/// See also the article on [Wikipedia](https://en.wikipedia.org/wiki/Chinese_remainder_theorem).
pub fn crt<T: Copy + Integer>(divisor_remainder_pairs: &[(T, T)]) -> T {
	// First, compute the product of all of the divisors.
	let product = divisor_remainder_pairs
		.iter()
		.fold(T::one(), |acc, (div, _rem)| acc * *div);

	let total = divisor_remainder_pairs
		.iter()
		.fold(T::zero(), |acc, (div, rem)| {
			let partial_product = product / *div;
			acc + *rem * partial_product * modular_multiplicative_inverse(partial_product, *div).unwrap()
		});

	total % product
}

pub type Intermediate = ();
type Solution = usize;

pub fn parse(_: &str) -> Result<Intermediate, core::convert::Infallible> {
	Ok(())
}

pub fn part_one(_: &Intermediate) -> Option<Solution> {
	None
}

pub fn part_two(_: &Intermediate) -> Option<Solution> {
	None
}
