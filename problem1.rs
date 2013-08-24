/*
 * Project Euler: Problem 1.
 *
 * Copyright (C) 2013, Adrien Tétar. All Rights Reserved.
 */
fn main() {
	let mut s = 0;
	for n in range(0, 1000) {
		if n % 3 == 0 || n % 5 == 0 {
			s = s + n;
		}
	}
	printfln!(s);
}
