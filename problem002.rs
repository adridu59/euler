/*
 * Project Euler: Problem 2.
 *
 * Copyright (C) 2013, Adrien Tétar. All Rights Reserved.
 */
fn main() {
	// Fibo consts
	let mut n1 = 0;
	let mut n2 = 1;
	let mut n3;
	let mut s = 0;
	loop {
		n3 = n1 + n2;
		if n3 < 4000000 {
			if n3 % 2 == 0 {
				s += n3;
			}
		} else {
			break;
		}
		n1 = n2;
		n2 = n3;
	}
	println!(s);
}
