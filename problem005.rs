/*
 * Project Euler: Problem 5.
 *
 * Copyright (C) 2013, Adrien Tétar. All Rights Reserved.
 */
fn isEvenlyDivisible (number: int) -> bool {
	for i in std::iterator::range_inclusive(1, 20) {
		if number % i != 0 {
			return false;
		}
	}
	return true;
}

fn main() {
	let mut n = 0;
	loop {
		n += 1;
		if isEvenlyDivisible(n) {
			break;
		}
	}
	println!(n);
}
