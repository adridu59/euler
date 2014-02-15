/*
 * Project Euler: Problem 6.
 *
 * Copyright (C) 2013, Adrien Tétar. All Rights Reserved.
 */
fn sumSquareDiff() -> int {
	let mut sum = 0;
	let mut square = 0;
	for i in std::iterator::range_inclusive(1, 100) {
		sum += i*i;
		square += i;
	}
	square = square*square;
	return square-sum;
}

fn main() {
	println!(sumSquareDiff());
}
