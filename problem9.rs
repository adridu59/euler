/*
 * Project Euler: Problem 9.
 *
 * Copyright (C) 2013, Adrien Tétar. All Rights Reserved.
 */
fn pythagoreanTriplet(targetSum: int) -> int {
	let mut c = 0;
	loop {
		c += 1;
		for a in range(0, c) {
			for b in range(a, c) {
				if a*a+b*b == c*c {
					if a+b+c == targetSum { return a*b*c; }
				}
			}
		}
	}
}

fn main() {
	printfln!(pythagoreanTriplet(1000));
}
